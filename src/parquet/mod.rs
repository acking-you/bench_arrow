use std::fs::File;

use arrow2::io::parquet::read;
use parquet::arrow::arrow_reader::ParquetRecordBatchReader;

const PARQUET_FILE_PATH: &str = "./test_data/hits_20.parquet";

pub fn sync_read_with_parquet() {
    let rows = {
        let file = File::open(PARQUET_FILE_PATH).unwrap();
        let parquet_reader = ParquetRecordBatchReader::try_new(file, 8192).unwrap();

        parquet_reader
            .into_iter()
            .map(|v| v.unwrap().num_rows())
            .sum::<usize>()
    };

    assert_eq!(rows, 1000000)
}

pub fn sync_read_with_parquet2() {
    let rows = {
        let mut reader = File::open(PARQUET_FILE_PATH).unwrap();

        // we can read its metadata:
        let metadata = read::read_metadata(&mut reader).unwrap();

        // and infer a [`Schema`] from the `metadata`.
        let schema = read::infer_schema(&metadata).unwrap();

        // we can then read the row groups into chunks
        let chunks = read::FileReader::new(
            reader,
            metadata.row_groups,
            schema,
            Some(1024 * 8 * 8),
            None,
            None,
        );

        chunks
            .into_iter()
            .map(|chunk| {
                let chunk = chunk.unwrap();
                chunk.len()
            })
            .sum::<usize>()
    };

    assert_eq!(rows, 1000000)
}

#[cfg(test)]
mod tests {
    use crate::utils::Timer;

    use super::*;
    #[test]
    fn test_sync_read_parquet() {
        let _time = Timer::new();
        sync_read_with_parquet();
    }

    #[test]
    fn test_sync_read_parquet2() {
        let _time = Timer::new();
        sync_read_with_parquet2();
    }
}
