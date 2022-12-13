use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use std::thread;
use std::time::Duration;

// use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;
// esp-idf-hal >= "0.39.3"
use esp_idf_hal::gpio::*; // PinDriver
// esp-idf-hal <= "0.38.1"
// use embedded_hal::digital::v2::OutputPin;


fn main() -> anyhow::Result<()> {
    let peripherals = Peripherals::take().unwrap();
    // my orig ESP32-C3 borad
    // let pin = peripherals.pins.gpio8;
    // my orig ESP32 borad
    let pin = peripherals.pins.gpio2;

    // esp-idf-hal >= "0.39.3"
    let mut led = PinDriver::output(pin)?;
    // esp-idf-hal <= "0.38.1"
    // let mut led = pin.into_output()?;

    loop {
        led.set_high()?;
        // we are using thread::sleep here to make sure the watchdog isn't triggered
        // FreeRtos::delay_ms(1000);
        thread::sleep(Duration::from_millis(500));

        led.set_low()?;
        // FreeRtos::delay_ms(1000);
        thread::sleep(Duration::from_millis(500));
    }
}
