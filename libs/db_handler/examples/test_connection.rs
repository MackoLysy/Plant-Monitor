
use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello Example!");
    let conn_url = "postgresql://admin:admin@localhost:5432/database";
    let pool = sqlx::PgPool::connect(&conn_url).await;
    match pool {
        Ok(pool) => {
            println!("Successfully connected to the database. {:?}", pool);
        }
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
        }
    }
    Ok(())
}
