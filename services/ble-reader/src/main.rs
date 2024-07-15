use std::error::Error;


mod ble_handler;
mod error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    utils::enable_logger();

    log::info!("Starting bluetooth servce");
    let mut ble_handler = ble_handler::BleHandler::new();
    ble_handler.init().await?;
    ble_handler.scan().await?;
    Ok(())
}
