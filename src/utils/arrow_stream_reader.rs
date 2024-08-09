use std::{
    collections::HashMap,
    fmt,
    io::{Cursor, Read},
    sync::Arc,
};

use arrow::{
    array::{ArrayRef, RecordBatch, RecordBatchReader},
    buffer::MutableBuffer,
    datatypes::SchemaRef,
    error::ArrowError,
    ipc::reader::{read_dictionary, read_record_batch},
};

const CONTINUATION_MARKER: [u8; 4] = [0xff; 4];

/// Used to check if the current buffer is still valid.
/// Currently used only in  [`ArrowStreamReader::need_update_reader`].
pub trait BufferValid {
    /// Check if the buffer is valid
    fn is_valid(&self) -> bool;
}

/// This structure is a wrapper for [`std::io::Cursor`], specifically designed for use with [`ArrowStreamReader`]
pub struct CursorReader<T>(Cursor<T>);

impl<T> CursorReader<T> {
    /// Construct a reader for [`ArrowStreamReader`]
    pub fn new(inner: T) -> Self {
        Self(Cursor::new(inner))
    }
}

impl<T> Read for CursorReader<T>
where
    T: AsRef<[u8]>,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        self.0.read_exact(buf)
    }
}

impl<T> BufferValid for CursorReader<T>
where
    T: AsRef<[u8]>,
{
    fn is_valid(&self) -> bool {
        (self.0.position() as usize) < self.0.get_ref().as_ref().len()
    }
}

/// Arrow Stream reader
/// Refer to [`arrow::ipc::reader::StreamReader`] for implementation.
/// ## Two differences:
///  - removes projection support
///  - provides the ability to update the `reader`
/// ## Note:
/// The message readable by the reader must be complete, make sure the body or header is not missing.
/// [Arrow IPC Format](https://arrow.apache.org/docs/format/Columnar.html#serialization-and-interprocess-communication-ipc)
pub struct ArrowStreamReader<R: Read + BufferValid> {
    /// Stream reader
    reader: R,

    /// The schema that is read from the stream's first message
    schema: SchemaRef,

    /// Optional dictionaries for each schema field.
    ///
    /// Dictionaries may be appended to in the streaming format.
    dictionaries_by_id: HashMap<i64, ArrayRef>,

    /// An indicator of whether the stream is complete.
    ///
    /// This value is set to `true` the first time the reader's `next()` returns `None`.
    finished: bool,
}

impl<R: Read + BufferValid> fmt::Debug for ArrowStreamReader<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        f.debug_struct("StreamReader<R>")
            .field("reader", &"BufReader<..>")
            .field("schema", &self.schema)
            .field("dictionaries_by_id", &self.dictionaries_by_id)
            .field("finished", &self.finished)
            .finish()
    }
}

impl<R: Read + BufferValid> ArrowStreamReader<R> {
    /// Try to create a new stream reader but do not wrap the reader in a BufReader.
    ///
    /// Unless you need the StreamReader to be unbuffered you likely want to use `StreamReader::try_new` instead.
    pub fn try_new(mut reader: R) -> Result<ArrowStreamReader<R>, ArrowError> {
        // determine metadata length
        let mut meta_size: [u8; 4] = [0; 4];
        reader.read_exact(&mut meta_size)?;
        let meta_len = {
            // If a continuation marker is encountered, skip over it and read
            // the size from the next four bytes.
            if meta_size == CONTINUATION_MARKER {
                reader.read_exact(&mut meta_size)?;
            }
            i32::from_le_bytes(meta_size)
        };

        let mut meta_buffer = vec![0; meta_len as usize];
        reader.read_exact(&mut meta_buffer)?;

        let message = arrow::ipc::root_as_message(meta_buffer.as_slice()).map_err(|err| {
            ArrowError::ParseError(format!("Unable to get root as message: {err:?}"))
        })?;
        // message header is a Schema, so read it
        let ipc_schema: arrow::ipc::Schema = message.header_as_schema().ok_or_else(|| {
            ArrowError::ParseError("Unable to read IPC message as schema".to_string())
        })?;
        let schema = arrow::ipc::convert::fb_to_schema(ipc_schema);

        // Create an array of optional dictionary value arrays, one per field.
        let dictionaries_by_id = HashMap::new();

        Ok(Self {
            reader,
            schema: Arc::new(schema),
            finished: false,
            dictionaries_by_id,
        })
    }

    /// Return the schema of the stream
    pub fn schema(&self) -> SchemaRef {
        self.schema.clone()
    }

    /// Check if the stream is finished
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    /// Check if the reader need update
    pub fn need_update_reader(&self) -> bool {
        !self.reader.is_valid() || self.finished
    }

    /// Check if the reader is valid
    pub fn is_valid_reader(&self) -> bool {
        self.reader.is_valid()
    }

    /// Update the reader
    pub fn update_reader(&mut self, reader: R) {
        self.finished = false;
        self.reader = reader;
        self.dictionaries_by_id.clear();
    }

    fn maybe_next(&mut self) -> Result<Option<RecordBatch>, ArrowError> {
        if self.finished {
            return Ok(None);
        }
        // determine metadata length
        let mut meta_size: [u8; 4] = [0; 4];

        match self.reader.read_exact(&mut meta_size) {
            Ok(()) => (),
            Err(e) => {
                return if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    // Handle EOF without the "0xFFFFFFFF 0x00000000"
                    // valid according to:
                    // https://arrow.apache.org/docs/format/Columnar.html#ipc-streaming-format
                    self.finished = true;
                    Ok(None)
                } else {
                    Err(ArrowError::from(e))
                };
            }
        }

        let meta_len = {
            // If a continuation marker is encountered, skip over it and read
            // the size from the next four bytes.
            if meta_size == CONTINUATION_MARKER {
                self.reader.read_exact(&mut meta_size)?;
            }
            i32::from_le_bytes(meta_size)
        };

        if meta_len == 0 {
            // the stream has ended, mark the reader as finished
            self.finished = true;
            return Ok(None);
        }

        let mut meta_buffer = vec![0; meta_len as usize];
        self.reader.read_exact(&mut meta_buffer)?;

        let vecs = &meta_buffer.to_vec();
        let message = arrow::ipc::root_as_message(vecs).map_err(|err| {
            ArrowError::ParseError(format!("Unable to get root as message: {err:?}"))
        })?;

        match message.header_type() {
            arrow::ipc::MessageHeader::Schema => Err(ArrowError::IpcError(
                "Not expecting a schema when messages are read".to_string(),
            )),
            arrow::ipc::MessageHeader::RecordBatch => {
                let batch = message.header_as_record_batch().ok_or_else(|| {
                    ArrowError::IpcError("Unable to read IPC message as record batch".to_string())
                })?;
                // read the block that makes up the record batch into a buffer
                let mut buf = MutableBuffer::from_len_zeroed(message.bodyLength() as usize);
                self.reader.read_exact(&mut buf)?;

                read_record_batch(
                    &buf.into(),
                    batch,
                    self.schema(),
                    &self.dictionaries_by_id,
                    None,
                    &message.version(),
                )
                .map(Some)
            }
            arrow::ipc::MessageHeader::DictionaryBatch => {
                let batch = message.header_as_dictionary_batch().ok_or_else(|| {
                    ArrowError::IpcError(
                        "Unable to read IPC message as dictionary batch".to_string(),
                    )
                })?;
                // read the block that makes up the dictionary batch into a buffer
                let mut buf = MutableBuffer::from_len_zeroed(message.bodyLength() as usize);
                self.reader.read_exact(&mut buf)?;

                read_dictionary(
                    &buf.into(),
                    batch,
                    &self.schema,
                    &mut self.dictionaries_by_id,
                    &message.version(),
                )?;

                // read the next message until we encounter a RecordBatch
                self.maybe_next()
            }
            arrow::ipc::MessageHeader::NONE => Ok(None),
            t => Err(ArrowError::InvalidArgumentError(format!(
                "Reading types other than record batches not yet supported, unable to read {t:?} "
            ))),
        }
    }

    /// Gets a reference to the underlying reader.
    ///
    /// It is inadvisable to directly read from the underlying reader.
    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    /// Gets a mutable reference to the underlying reader.
    ///
    /// It is inadvisable to directly read from the underlying reader.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.reader
    }
}

impl<R: Read + BufferValid> Iterator for ArrowStreamReader<R> {
    type Item = Result<RecordBatch, ArrowError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.maybe_next().transpose()
    }
}

impl<R: Read + BufferValid> RecordBatchReader for ArrowStreamReader<R> {
    fn schema(&self) -> SchemaRef {
        self.schema.clone()
    }
}
