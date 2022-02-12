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

pub async fn benchmark_mobc(pool_size: usize, workers: usize, iterations: usize) {
    let pool = Pool::builder().max_open(pool_size as u64).build(Manager {});
    let handles = (0..workers)
        .map(|_| {
            let pool = pool.clone();
            tokio::spawn(async move {
                for _ in 0..iterations {
                    let _ = pool.get().await;
                }
            })
        })
        .collect::<Vec<_>>();
    for handle in handles {
        handle.await.unwrap();
    }
}
