use tokio::task::JoinHandle;

use crate::BenchmarkConfig;

struct Manager {}

#[async_trait::async_trait]
impl ::bb8_0_7::ManageConnection for Manager {
    type Connection = ();
    type Error = ();
    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Ok(())
    }
    async fn is_valid(
        &self,
        _: &mut bb8_0_7::PooledConnection<'_, Self>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

type Pool = bb8_0_7::Pool<Manager>;

pub async fn run(cfg: BenchmarkConfig) -> Vec<JoinHandle<()>> {
    let pool = Pool::builder()
        .max_size(cfg.pool_size as u32)
        .build(Manager {})
        .await
        .unwrap();
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
