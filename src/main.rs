mod bb8_07;
mod deadpool_08;
mod deadpool_09;
mod mobc_07;

use std::time::Instant;

async fn run_benchmarks(pool_size: usize, workers: usize, iterations: usize) {
    println!("-----------------------------");
    println!("pool_size: {}", pool_size);
    println!("workers: {}", workers);
    println!("iterations: {}", iterations);
    println!("-----------------------------");

    let iterations_per_worker = iterations / workers;

    {
        let start = Instant::now();
        deadpool_09::benchmark_deadpool(pool_size, workers, iterations_per_worker).await;
        let end = Instant::now();
        println!("deadpool 0.9: {}ms", end.duration_since(start).as_millis());
    }
    {
        let start = Instant::now();
        deadpool_08::benchmark_deadpool(pool_size, workers, iterations_per_worker).await;
        let end = Instant::now();
        println!("deadpool 0.8: {}ms", end.duration_since(start).as_millis());
    }
    {
        let start = Instant::now();
        bb8_07::benchmark_bb8(pool_size, workers, iterations_per_worker).await;
        let end = Instant::now();
        println!("bb8 0.8: {}ms", end.duration_since(start).as_millis());
    }
    {
        let start = Instant::now();
        mobc_07::benchmark_mobc(pool_size, workers, iterations_per_worker).await;
        let end = Instant::now();
        println!("mobc 0.7: {}ms", end.duration_since(start).as_millis());
    }
    println!("");
    println!("");
}

#[tokio::main]
async fn main() {
    run_benchmarks(1, 16, 1048576).await;
    run_benchmarks(8, 16, 1048576).await;
    run_benchmarks(16, 16, 1048576).await;
    run_benchmarks(16, 32, 1048576).await;
    run_benchmarks(32, 32, 1048576).await;
    run_benchmarks(32, 128, 1048576).await;
}
