use std::{convert::Infallible};

use tokio::task::JoinHandle;

use crate::BenchmarkConfig;

struct Manager {}

#[async_trait::async_trait]
impl ::deadpool_0_7::managed::Manager<(), Infallible> for Manager {
    async fn create(&self) -> Result<(), Infallible> {
        Ok(())
    }
    async fn recycle(
        &self,
        _: &mut (),
    ) -> ::deadpool_0_7::managed::RecycleResult<Infallible> {
        Ok(())
    }
}

type Pool = ::deadpool_0_7::managed::Pool<(), Infallible>;

pub async fn run(cfg: BenchmarkConfig) -> Vec<JoinHandle<()>> {
    let pool = Pool::new(Manager {}, cfg.pool_size);
    (0..cfg.workers)
        .map(|_| {
            let pool = pool.clone();
            tokio::spawn(async move {
                for _ in 0..cfg.iterations_per_worker {
                    let _ = pool.get().await;
                }
            })
        })
        .collect()
}
