use tokio::task::JoinHandle;

use crate::BenchmarkConfig;

struct Manager {}

#[async_trait::async_trait]
impl ::qp_0_2::resource::Manage for Manager {
    type Output = ();
    type Error = ();
    async fn try_create(&self) -> Result<Self::Output, Self::Error> {
        Ok(())
    }
}

type Pool = ::qp_0_2::Pool<Manager>;

pub async fn run(cfg: BenchmarkConfig) -> Vec<JoinHandle<()>> {
    let pool = Pool::new(Manager {}, cfg.pool_size);
    (0..cfg.workers)
        .map(|_| {
            let pool = pool.clone();
            tokio::spawn(async move {
                for _ in 0..cfg.iterations_per_worker {
                    let _ = pool.acquire().await;
                }
            })
        })
        .collect()
}
