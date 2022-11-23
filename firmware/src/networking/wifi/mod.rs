use defmt::debug;
use embassy_futures::yield_now;
use embedded_svc::wifi::{ClientConfiguration, Configuration, Wifi};

#[cfg(feature = "esp-wifi")]
#[path = "esp.rs"]
pub mod ඞ;

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");
const EXPECTED_NEIGHBOURS: usize = 10;
const WIFI_FIND_RETRIES: usize = 10;

pub async fn connect_wifi<W: Wifi>(wifi: &mut W) -> Result<(), W::Error> {
	if !wifi.is_started()? {
		wifi.start()?
	}

	let mut i = 0;
	let ap = loop {
		i += 1;
		debug!("wifi scanning, retry {}...", i);
		let (mut scan_list, count) = wifi.scan_n::<EXPECTED_NEIGHBOURS>()?;
		debug!("found {} APs", count);

		// we yield because scan_n is blocking
		// this also requires a ticker
		//yield_now().await;
		let pos = scan_list.iter().position(|ap| ap.ssid == SSID);

		if let Some(ap) = pos {
			break scan_list.swap_remove(ap);
		} else if i == WIFI_FIND_RETRIES {
			panic!("Couldn't find SSID {}", SSID);
		}
	};
	debug!("found SSID {}", SSID);
	let client_config = Configuration::Client(ClientConfiguration {
		ssid: SSID.into(),
		password: PASSWORD.into(),
		bssid: Some(ap.bssid),
		auth_method: ap.auth_method,
		channel: Some(ap.channel),
	});
	wifi.set_configuration(&client_config)?;

	debug!("{:?}", defmt::Debug2Format(&wifi.get_capabilities()?));
	wifi.connect()?;

	loop {
		if wifi.is_connected()? {
			break;
		}
		//FIXME: Maybe a ticker would be better in this case.
		//yield_now().await;
	}

	Ok(())
}
