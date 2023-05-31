use env_logger::Env;
use log::{error, info};
use wifi_station::{sta, Result};

#[tokio::main]
async fn main() -> Result {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting wifi-sta example");

    let mut setup = sta::WifiSetup::new()?;
    // Use something like ifconfig to figure out the name of your WiFi interface
    setup.set_socket_path("/var/run/wpa_supplicant/wlan0");

    let broadcast = setup.get_broadcast_receiver();
    let requester = setup.get_request_client();
    let runtime = setup.complete();

    let (_runtime, _app, _broadcast) = tokio::join!(
        async move {
            if let Err(e) = runtime.run().await {
                error!("Error: {e}");
            }
        },
        app(requester),
        broadcast_listener(broadcast),
    );
    Ok(())
}

async fn app(requester: sta::RequestClient) -> Result {
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

        let wifi_ssid = "Actonate_Tenda_2.4";
        let wifi_psk = "actonate9522";
        connect_to_network(&requester, wifi_ssid, wifi_psk).await?;
    }
    info!("Shutting down");
    requester.shutdown().await?;
    Ok(())
}

async fn broadcast_listener(mut broadcast_receiver: sta::BroadcastReceiver) -> Result {
    while let Ok(broadcast) = broadcast_receiver.recv().await {
        info!("Broadcast: {:?}", broadcast);
    }
    Ok(())
}

async fn connect_to_network(requester: &sta::RequestClient, ssid: &str, psk: &str) -> Result {
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
