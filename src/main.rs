mod bb8_0_7;
mod bb8_0_8;
mod deadpool_0_7;
mod deadpool_0_8;
mod deadpool_0_9;
mod mobc_0_7;
mod qp_0_2;

const TIMEOUT: Duration = Duration::from_secs(5);

use std::{
    fs::File,
    future::Future,
    time::{Duration, Instant},
};

use abort_on_drop::ChildTask;
use serde::Serialize;
use tokio::task::JoinHandle;

#[derive(Default, Copy, Clone, Serialize)]
pub struct BenchmarkConfig {
    pub pool_size: usize,
    pub workers: usize,
    pub iterations: usize,
    pub iterations_per_worker: usize,
}

#[derive(Serialize)]
pub struct Benchmark {
    pub config: BenchmarkConfig,
    pub results: Vec<BenchmarkResult>,
}

impl Benchmark {
    pub async fn run<F: Future<Output = Vec<JoinHandle<()>>>>(
        &mut self,
        name: &'static str,
        version: &'static str,
        f: fn(BenchmarkConfig) -> F,
    ) {
        let start = Instant::now();
        let handles = f(self.config)
            .await
            .into_iter()
            .map(ChildTask::from)
            .collect::<Vec<_>>();
        let success = tokio::time::timeout(TIMEOUT, async {
            for handle in handles {
                handle.await.unwrap();
            }
        })
        .await
        .is_ok();
        let end = Instant::now();
        let duration = end.duration_since(start);
        println!(
            "{} {}: {}ms{}",
            name,
            version,
            duration.as_millis(),
            if success { "" } else { " [TIMEOUT]" }
        );
        self.results.push(BenchmarkResult {
            name,
            version,
            duration: if success { Some(duration) } else { None },
        });
    }
}

#[derive(Serialize)]
pub struct BenchmarkResult {
    pub name: &'static str,
    pub version: &'static str,
    pub duration: Option<Duration>,
}

async fn run_benchmarks(pool_size: usize, workers: usize, iterations: usize) -> Benchmark {
    println!("-----------------------------");
    println!("pool_size: {}", pool_size);
    println!("workers: {}", workers);
    println!("iterations: {}", iterations);
    println!("-----------------------------");
    let mut benchmark = Benchmark {
        config: BenchmarkConfig {
            pool_size,
            workers,
            iterations,
            iterations_per_worker: iterations / workers,
        },
        results: Vec::new(),
    };
    benchmark.run("deadpool", "0.7", deadpool_0_7::run).await;
    benchmark.run("deadpool", "0.8", deadpool_0_8::run).await;
    benchmark.run("deadpool", "0.9", deadpool_0_9::run).await;
    benchmark.run("bb8", "0.7", bb8_0_7::run).await;
    benchmark.run("bb8", "0.8", bb8_0_8::run).await;
    benchmark.run("mobc", "0.7", mobc_0_7::run).await;
    benchmark.run("qp", "0.2", qp_0_2::run).await;
    println!();
    benchmark
}

#[tokio::main]
async fn main() {
    let mut benchmarks = Vec::new();
    for &pool_size in &[8, 16, 32] {
        for &workers in &[8, 16, 32, 64, 128, 256] {
            benchmarks.push(run_benchmarks(pool_size, workers, 1048576).await);
        }
    }
    let file = File::create("results.json").unwrap();
    serde_json::to_writer_pretty(&file, &benchmarks).unwrap();
}
