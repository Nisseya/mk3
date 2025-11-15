use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::gpio::PinDriver;
use std::thread::sleep;
use std::time::Duration;
use esp_idf_dht;

fn main()  -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("starting, attach debugger if needed");
    sleep(Duration::from_secs(4));
    log::info!("started");

    let peripherals: Peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let mut sensor = PinDriver::input_output_od(pins.gpio4).unwrap();
    sleep(Duration::from_secs(1));
  
    loop {
        let vals = esp_idf_dht::read(&mut sensor).unwrap();

        log::info!("values are {vals:?}");

        sleep(Duration::from_secs(0));
    }
}