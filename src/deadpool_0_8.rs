use tokio::task::JoinHandle;

use crate::BenchmarkConfig;

struct Manager {}

#[async_trait::async_trait]
impl ::deadpool_0_8::managed::Manager for Manager {
    type Type = ();
    type Error = ();
    async fn create(&self) -> Result<Self::Type, Self::Error> {
        Ok(())
    }
    async fn recycle(
        &self,
        _: &mut Self::Type,
    ) -> deadpool_0_8::managed::RecycleResult<Self::Error> {
        Ok(())
    }
}

type Pool = ::deadpool_0_8::managed::Pool<Manager>;

pub async fn run(cfg: BenchmarkConfig) -> Vec<JoinHandle<()>> {
    let pool = Pool::new(Manager {}, cfg.pool_size);
    (0..cfg.workers)
        .map(|_| {
            let pool = pool.clone();
            tokio::spawn(async move {
                for _ in 0..cfg.operations_per_worker() {
                    let _ = pool.get().await;
                }
            })
        })
        .collect()
}
