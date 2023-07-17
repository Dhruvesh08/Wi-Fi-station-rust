use wifi_station::{sta, Result};
use log::{error, info};

pub(crate) struct WifiClient;

impl WifiClient {
   pub async fn scan_wifi(&self) -> Result {
        let mut setup = sta::WifiSetup::new()?;
        setup.set_socket_path("/var/run/wpa_supplicant/wlan0");

        let broadcast = setup.get_broadcast_receiver();
        let requester = setup.get_request_client();
        let runtime = setup.complete();

        info!("Requesting scan");
        let scan = requester.get_scan().await?;
        info!("Scan complete");
        for scan in scan.iter() {
            info!("   {:?}", scan);
        }

        let networks = requester.get_networks().await?;
        info!("Known networks");
        for networks in networks.iter() {
            info!("   {:?}", networks);
        }

        Ok(())
    }

    pub async fn connect_to_wifi(&self, ssid: &str, psk: &str) -> Result {
        let mut setup = sta::WifiSetup::new()?;
        setup.set_socket_path("/var/run/wpa_supplicant/wlan0");

        let broadcast = setup.get_broadcast_receiver();
        let requester = setup.get_request_client();
        let runtime = setup.complete();

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
