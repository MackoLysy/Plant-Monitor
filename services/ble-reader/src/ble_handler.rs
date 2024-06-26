use crate::error::Errors;
use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use log::{self, info};
use std::error::Error;
use std::time::Duration;
use tokio::time;
const DEVICE_NAME: &str = "Plant-Node";
#[derive(Default)]
pub struct Ble_Handler {
    adapter: Option<Adapter>,
}

impl Ble_Handler {
    pub fn new() -> Self {
        return Ble_Handler { adapter: None };
    }
    pub async fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let manager = Manager::new().await?;
        let adapter_list = manager.adapters().await?;
        if adapter_list.is_empty() {
            return Err(Box::new(Errors::new("unable to find ble adapter")));
        }
        self.adapter = Some(adapter_list[0].to_owned());
        Ok(())
    }
    pub async fn scan(&self) -> Result<(), Box<dyn Error>> {
        let adapter = self.adapter.as_ref().unwrap();
        let info = adapter.adapter_info().await?;
        info!("Scanning adapter...: {}", info);
        loop {
            adapter
                .start_scan(ScanFilter::default())
                .await
                .expect("Cant scan for devices!");
            time::sleep(Duration::from_secs(10)).await;
            let peripherals = adapter.peripherals().await?;
            for peripheral in peripherals.into_iter() {
                let properties = peripheral.properties().await?;
                if properties.is_some()
                    && properties.as_ref().unwrap().local_name == Some(DEVICE_NAME.to_string())
                {
                    info!(
                        "Find device: {:?} with addr: {:?}",
                        properties.as_ref().unwrap().local_name,
                        properties.as_ref().unwrap().address
                    );
                }
            }
            adapter.stop_scan().await?;
        }
        Ok(())
    }
}
