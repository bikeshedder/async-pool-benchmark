use std::sync::Arc;

use tokio::task::JoinHandle;

use crate::BenchmarkConfig;

type Pool = ::async_object_pool::Pool<()>;

pub async fn run(cfg: BenchmarkConfig) -> Vec<JoinHandle<()>> {
    let pool = Arc::new(Pool::new(cfg.pool_size));
    (0..cfg.workers)
        .map(|_| {
            let pool = pool.clone();
            tokio::spawn(async move {
                for _ in 0..(cfg.operations_per_worker()) {
                    let item = pool.take_or_create(|| ()).await;
                    pool.put(item).await;
                }
            })
        })
        .collect()
}
