use esp_idf_hal::{delay::FreeRtos, gpio::*, peripherals::Peripherals};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() -> anyhow::Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio8)?;

    loop {
        led.set_high()?;
        FreeRtos::delay_ms(1000);

        led.set_high()?;
        FreeRtos::delay_ms(1000);
    }
}
