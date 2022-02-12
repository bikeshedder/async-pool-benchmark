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

pub async fn benchmark_deadpool(pool_size: usize, workers: usize, iterations: usize) {
    let pool = Pool::new(Manager {}, pool_size);
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
