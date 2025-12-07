use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::gpio::PinDriver;
use std::thread::sleep;
use std::time::Duration;
use esp_idf_dht;

pub fn read_sensor()  -> Result<[f32; 2], String> {
    let peripherals: Peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let mut sensor = PinDriver::input_output_od(pins.gpio4).unwrap();
    sleep(Duration::from_secs(1));
  
    esp_idf_dht::read(&mut sensor)
}