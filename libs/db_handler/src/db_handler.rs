use super::*;
use log::{debug, error, info};
impl DBHandler {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let conn_url = "postgresql://admin:admin@localhost:5432/database";
        let pool = sqlx::PgPool::connect(&conn_url).await?;
        debug!("connected to DB");
        Ok(DBHandler {
            pool: Arc::new(pool),
        })
    }
    async fn create_table(&self, query: &str) -> Result<(), Box<dyn Error>> {
        let result = sqlx::query(query).fetch_all(self.pool.as_ref()).await;
        match result {
            Ok(_) => {
                info!("Device Table created succesfuly!");
            }
            Err(e) => {
                error!("failed to create device table: {}", e);
            }
        }
        Ok(())
    }
}
impl IDBHandler for DBHandler {
    async fn init(&self) -> Result<(), Box<dyn Error>> {
        self.create_table(queries::DEVICE_TABLE_QUERY).await?;
        self.create_table(queries::PLANT_TABLE_QUERY).await?;
        self.create_table(queries::LOG_TABLE_QUERY).await?;
        Ok(())
    }

    async fn add_plant(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
