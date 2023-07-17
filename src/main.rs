use env_logger::Env;
use log::{error, info};
mod wifi;
use wifi::WifiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting wifi-sta example");

    let wifi_client = WifiClient;

    wifi_client.scan_wifi().await?;
    // wifi_client.connect_to_wifi("ssid", "psk").await?;

    Ok(())
}

