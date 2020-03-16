#[macro_use]
extern crate bencher;

use bencher::Bencher;

/// Number of bytes in generated test file.
const N_BYTES: u32 = 100000;

/// Number of nanoseconds to sleep for in lengthy computation.
const COMPUTE_NANOS: u64 = 2_000_000;

/// Generates random vector of N_BYTES bytes.
fn gen_bytes() -> Vec<u8> {
    use rand::prelude::*;
    let mut rng = rand::thread_rng();
    (1..N_BYTES).map(|_| rng.gen::<u8>()).collect()
}

/// Simulates random lengthy computation.
async fn compute() {
    use async_std::task;
    use std::time::Duration;

    task::sleep(Duration::from_nanos(COMPUTE_NANOS)).await;
}

/// Computes and writes file synchronously.
fn compute_write() {
    use async_std::task;

    write_file();
    task::block_on(compute());
}

/// Computes and writes file asynchronously with the use of async_std::task and async_std::fs.
async fn compute_write_async_std() {
    use async_std::task;
    let write_handle = task::spawn(write_file_async_std());
    let compute_handle = task::spawn(compute());
    write_handle.await;
    compute_handle.await;
}

/// Computes and writes file asynchronously with the use of futures::join and async_std::fs.
async fn compute_write_async_std_futures() {
    use futures::join;
    let write_future = write_file_async_std();
    let compute_future = compute();
    join!(write_future, compute_future);
}

/// Computes and writes file asynchronously with the use of tokio::join and tokio::fs.
async fn compute_write_tokio() {
    let write_future = write_file_async_std();
    let compute_future = compute();
    tokio::join!(write_future, compute_future);
}

/// Writes file asynchronously in a temporary directory with the use of async_std::fs.
async fn write_file_async_std() {
    use tempfile::tempdir;
    use async_std::fs::File;
    use async_std::prelude::*;

    let dir = tempdir().unwrap();
    let mut file = File::create(dir.path().join("temp_file")).await.unwrap();
    file.write_all(&gen_bytes()).await.unwrap()
}

/// Writes file asynchronously in a temporary directory with the use of tokio::fs.
async fn write_file_tokio() {
    use tempfile::tempdir;
    use tokio::fs::File;
    use tokio::prelude::*;

    let dir = tempdir().unwrap();
    let mut file = File::create(dir.path().join("temp_file")).await.unwrap();
    file.write_all(&gen_bytes()).await.unwrap()
}

/// Writes file synchronously in temporary directory with the use of std::fs.
fn write_file() {
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::prelude::*;

    let dir = tempdir().unwrap();
    let mut file = File::create(dir.path().join("temp_file")).unwrap();
    file.write_all(&gen_bytes()).unwrap()
}

// Benchmarks

fn bench_write_file(bench: &mut Bencher) {
    bench.iter(|| {
        write_file();
    });
}

fn bench_write_file_async_std(bench: &mut Bencher) {
    use async_std::task;

    bench.iter(|| {
        task::block_on(async {
            write_file_async_std().await;
        });
    });
}

fn bench_write_file_tokio(bench: &mut Bencher) {
    use tokio::runtime::Runtime;

    let mut rt = Runtime::new().unwrap();

    bench.iter(|| {
        rt.block_on(async {
            write_file_async_std().await;
        });
    });
}

fn bench_compute_write(bench: &mut Bencher) {
    bench.iter(|| {
        compute_write();
    });
}

fn bench_compute_write_async_std(bench: &mut Bencher) {
    use async_std::task;

    bench.iter(|| {
        task::block_on(async {
            compute_write_async_std().await;
        });
    });
}

fn bench_compute_write_async_std_futures(bench: &mut Bencher) {
    use futures::executor::block_on;

    bench.iter(|| {
        block_on(async {
            compute_write_async_std_futures().await;
        });
    });
}

fn bench_compute_write_tokio(bench: &mut Bencher) {
    use tokio::runtime::Runtime;

    let mut rt = Runtime::new().unwrap();

    bench.iter(|| {
        rt.block_on(async {
            compute_write_async_std_futures().await;
        });
    });
}

benchmark_group!(compute_write_group, bench_compute_write,
 bench_compute_write_async_std,
  bench_compute_write_async_std_futures,
   bench_compute_write_tokio);

benchmark_group!(write_files_group, bench_write_file_async_std, bench_write_file, bench_write_file_tokio);

benchmark_main!(write_files_group, compute_write_group);