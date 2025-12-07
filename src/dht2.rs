use std::thread::sleep;
use std::time::Duration;
use esp_idf_hal::gpio::{PinDriver};
#[derive(Debug)]
pub enum SensorError {
    ReadError,
}

pub fn read_sensor(sensor_pin: &mut PinDriver<'_, esp_idf_hal::gpio::Gpio4, esp_idf_hal::gpio::InputOutput> ) -> [f32; 2] {
    let result: Result<[f32; 2], SensorError> = {
        sleep(Duration::from_secs(1));
        
        esp_idf_dht::read(sensor_pin)
            .map_err(|_| SensorError::ReadError) 
    };

    match result {
        Ok(values) => values,
        Err(err) =>{
            println!("Error reading sensor: {:?}", err);
            [0.0, 0.0]
        },
    }
}