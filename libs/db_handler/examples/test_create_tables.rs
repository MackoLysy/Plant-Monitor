use ::db_handler::{DBHandler, IDBHandler};
use std::error::Error;
use utils::enable_logger;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_logger();
    let db_handler = DBHandler::new().await?;
    db_handler.init().await?;
    Ok(())
}
