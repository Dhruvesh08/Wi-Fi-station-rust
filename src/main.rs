// In main.rs
mod wifi;
use wifi::wifi_client;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scan_results = wifi_client::scan_wifi().await?;
    for scan_result in scan_results.iter() {
        println!("{:?}", scan_result);
    }

    // let ssid = "my_ssid";
    // let psk = "my_psk";
    // wifi_client::connect_wifi(ssid, psk).await?;

    Ok(())

    // Do something with the connected network...
}
