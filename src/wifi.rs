// In lib.rs

pub mod wifi_client {
    use std::sync::Arc;
    use log::info;
    use wifi_station::{sta, Result};

    pub async fn scan_wifi() -> Result<Vec<sta::ScanResult>> {
        let mut setup = sta::WifiSetup::new()?;
        setup.set_socket_path("/var/run/wpa_supplicant/wlan0");
        info!("scanning wifi");
        let requester = setup.get_request_client();
        let scan = requester.get_scan().await?;
        let scan_vec = Arc::clone(&scan).to_vec();
        Ok(scan_vec)
    }

    pub async fn connect_wifi(ssid: &str, psk: &str) -> Result {
        let mut setup = sta::WifiSetup::new()?;
        setup.set_socket_path("/var/run/wpa_supplicant/wlan0");
        let requester = setup.get_request_client();

        // Add a new network
        let network_id = requester.add_network().await?;
        info!("Added network: {:?}", network_id);

        // Set the SSID and PSK for the network
        requester
            .set_network_ssid(network_id, ssid.to_string())
            .await?;
        requester
            .set_network_psk(network_id, psk.to_string())
            .await?;

        // Select the network
        requester.select_network(network_id).await?;

        Ok(())
    }
}
