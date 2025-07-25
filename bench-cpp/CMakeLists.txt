cmake_minimum_required(VERSION 3.21)
project(bench_cpp)

set(CMAKE_CXX_STANDARD 20)

add_executable(bench_cpp main.cpp)

add_definitions(-DBENCH_DATA_PATH="${PROJECT_SOURCE_DIR}/../test_data/hits_20.parquet")

find_package(Arrow CONFIG REQUIRED)
target_link_libraries(bench_cpp PRIVATE "$<IF:$<BOOL:${ARROW_BUILD_STATIC}>,Arrow::arrow_static,Arrow::arrow_shared>")

find_package(Parquet CONFIG REQUIRED)
target_link_libraries(bench_cpp PRIVATE "$<IF:$<BOOL:${ARROW_BUILD_STATIC}>,Parquet::parquet_static,Parquet::parquet_shared>")
