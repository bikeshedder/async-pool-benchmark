use tokio::task::JoinHandle;

use crate::BenchmarkConfig;

struct Manager {}

#[async_trait::async_trait]
impl ::mobc_0_7::Manager for Manager {
    type Connection = ();
    type Error = ();
    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Ok(())
    }
    async fn check(&self, _: Self::Connection) -> Result<Self::Connection, Self::Error> {
        Ok(())
    }
}

type Pool = ::mobc_0_7::Pool<Manager>;

pub async fn run(cfg: BenchmarkConfig) -> Vec<JoinHandle<()>> {
    let pool = Pool::builder().max_open(cfg.pool_size as u64).build(Manager {});
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
