#![no_std]
#![no_main]

use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // GPIO7 controls the green LED (active-LOW)
    // Start with LED ON by setting GPIO7 LOW
    let mut led = Output::new(peripherals.GPIO7, Level::Low, OutputConfig::default());

    loop {
        // LED ON (active-low: GPIO7 = LOW)
        led.set_low();
        delay.delay_millis(1000);
        
        // LED OFF (active-low: GPIO7 = HIGH)  
        led.set_high();
        delay.delay_millis(1000);
    }
}
