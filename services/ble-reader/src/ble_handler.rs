use crate::error::Errors;
use btleplug::api::{
    Central, Characteristic, Manager as _, Peripheral as PeripheralTrait, ScanFilter,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures_util::stream::StreamExt;
use log::{self, info};
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use tokio::time;

const DEVICE_NAME: &str = "Plant-Node";

struct Device {
    addr: String,
}

#[derive(Default)]
pub struct BleHandler {
    adapter: Option<Adapter>,
    devices: HashMap<String, Device>,
}

impl BleHandler {
    pub fn new() -> Self {
        return BleHandler {
            adapter: None,
            devices: HashMap::default(),
        };
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
    pub async fn scan(&mut self) -> Result<(), Box<dyn Error>> {
        let adapter = self.adapter.take().unwrap();
        let info = adapter.adapter_info().await?;
        info!("Scanning adapter...: {}", info);
        loop {
            adapter.start_scan(ScanFilter::default()).await?;
            time::sleep(Duration::from_secs(5)).await;
            info!("Scanning end");
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

                    peripheral.connect().await?;
                    let is_connected = peripheral.is_connected().await?;
                    if is_connected {
                        info!(
                            "getting characteristics: {:?}",
                            properties.as_ref().unwrap().local_name
                        );
                        peripheral.discover_services().await?;
                        let charateristics = peripheral.characteristics();
                        for charateristic in charateristics.into_iter() {
                            info!("characteristics {:?}", charateristic);
                            if charateristic.uuid.to_string().contains("aaa2") {
                                self.add_device(
                                    &peripheral,
                                    &charateristic,
                                    &properties.as_ref().unwrap().address.to_string(),
                                )
                                .await?;
                            }
                        }
                    }
                }
            }
            adapter.stop_scan().await?;
        }
    }

    async fn add_device(
        &mut self,
        peripheral: &Peripheral,
        charateristic: &Characteristic,
        addr: &str,
    ) -> Result<(), Box<dyn Error>> {
        if !self.devices.contains_key(addr) {
            peripheral.subscribe(charateristic).await?;
            let mut stream = peripheral.notifications().await?.take(100);
            let addr_copy = addr.to_string();
            info!("adding device to listen: {:?} ", addr_copy);
            self.devices.insert(
                addr_copy.to_string(),
                Device {
                    addr: addr_copy.to_string(),
                },
            );
            tokio::spawn(async move {
                while let Some(data) = stream.next().await {
                    info!(
                        "Received data from[{:?}]: {:?}",
                        addr_copy.clone(),
                        data.value
                    );
                }
            });
            return Ok(());
        }
        info!("Device already registered!");
        Ok(())
    }
}
