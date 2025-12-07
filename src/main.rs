use core::convert::TryInto;
use std::{thread::sleep, time::Duration};
use embedded_svc::{
    http::{client::Client as HttpClient},
    io::Write,
    wifi::{AuthMethod, ClientConfiguration, Configuration},
};
use esp_idf_hal::gpio::PinDriver;

use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::http::client::EspHttpConnection;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};

use log::{info};
mod dht2;

const SSID: &str = env!("WIFI_SSID");
const PASSWORD: &str = env!("WIFI_PASS");

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();

    let peripherals: Peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let pins = peripherals.pins;
    let mut sensor_pin: PinDriver<'_, esp_idf_hal::gpio::Gpio4, esp_idf_hal::gpio::InputOutput> = PinDriver::input_output_od(pins.gpio4)?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    connect_wifi(&mut wifi)?;

    // Create HTTP client
    // Note: To send a request to an HTTPS server, you can do:
    // ```

    use esp_idf_svc::http::client::{Configuration as HttpConfiguration, EspHttpConnection};

    let config = &HttpConfiguration {
        crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
        ..Default::default()
    };

    let mut client = HttpClient::wrap(EspHttpConnection::new(&config)?);

    // ```
    // let mut client = HttpClient::wrap(EspHttpConnection::new(&Default::default())?);
    // POST

    loop{
        let dur: Duration = Duration::new(10,0);
        let sensor_values : [f32; 2] = dht2::read_sensor(& mut sensor_pin);
        post_request(&mut client, sensor_values)?;
        sleep(dur);
    }
}



fn post_request(client: &mut HttpClient<EspHttpConnection>, values:[f32;2]) -> anyhow::Result<()> {
    let payload_string = format!(r#"{{"tmp":{},"hdt":{}}}"#, values[0], values[1]);
    let payload = payload_string.as_bytes();
    let content_length_string = format!("{}", payload.len());

    let headers = [
        ("content-type", "application/json"),
        ("content-length", content_length_string.as_str()),
    ];
    
    let url = env!("SERVER_URL");

    let mut request = client.post(url, &headers)?;
    request.write_all(payload)?;
    request.flush()?;
    info!("-> POST {url}");
    let mut response = request.submit()?;

    Ok(())
}

fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {

    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.try_into().unwrap(),
        channel: None,
        ..Default::default()
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start()?;
    info!("Wifi started");

    wifi.connect()?;
    info!("Wifi connected");

    wifi.wait_netif_up()?;
    info!("Wifi netif up");

    Ok(())
}