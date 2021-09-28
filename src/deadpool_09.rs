struct Manager {}

#[async_trait::async_trait]
impl ::deadpool_09::managed::Manager for Manager {
    type Type = ();
    type Error = ();
    async fn create(&self) -> Result<Self::Type, Self::Error> {
        Ok(())
    }
    async fn recycle(
        &self,
        _: &mut Self::Type,
    ) -> deadpool_09::managed::RecycleResult<Self::Error> {
        Ok(())
    }
}

type Pool = ::deadpool_09::managed::Pool<Manager>;

pub async fn benchmark_deadpool(pool_size: usize, iterations: usize, workers: usize) {
    let pool = Pool::builder(Manager {})
        .max_size(pool_size)
        .build()
        .unwrap();
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
