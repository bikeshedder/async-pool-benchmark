struct Manager {}

#[async_trait::async_trait]
impl ::bb8_07::ManageConnection for Manager {
    type Connection = ();
    type Error = ();
    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Ok(())
    }
    async fn is_valid(&self, _: &mut bb8_07::PooledConnection<'_, Self>) -> Result<(), Self::Error> {
        Ok(())
    }
    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

type Pool = bb8_07::Pool<Manager>;

pub async fn benchmark_bb8(pool_size: usize, iterations: usize, workers: usize) {
    let pool = Pool::builder()
        .max_size(pool_size as u32)
        .build(Manager {})
        .await
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
