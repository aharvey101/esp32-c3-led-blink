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
    
    // J07 LED on GPIO7 (active-low: set_low() = ON, set_high() = OFF)
    let mut led = Output::new(peripherals.GPIO7, Level::High, OutputConfig::default());
    
    loop {
        // Pattern 1: Heartbeat pulse
        heartbeat_pattern(&mut led, &delay);
        delay.delay_millis(1000);
        
        // Pattern 2: SOS morse code
        sos_pattern(&mut led, &delay);
        delay.delay_millis(2000);
        
        // Pattern 3: Breathing effect
        breathing_pattern(&mut led, &delay);
        delay.delay_millis(1000);
        
        // Pattern 4: Rapid blink sequence
        rapid_sequence(&mut led, &delay);
        delay.delay_millis(2000);
        
        // Pattern 5: Double pulse
        double_pulse_pattern(&mut led, &delay);
        delay.delay_millis(1500);
    }
}

fn heartbeat_pattern(led: &mut Output, delay: &Delay) {
    // Quick double pulse like a heartbeat
    for _ in 0..3 {
        led.set_low();  // ON
        delay.delay_millis(80);
        led.set_high(); // OFF
        delay.delay_millis(80);
        led.set_low();  // ON
        delay.delay_millis(80);
        led.set_high(); // OFF
        delay.delay_millis(400);
    }
}

fn sos_pattern(led: &mut Output, delay: &Delay) {
    // SOS in morse: ... --- ...
    
    // S: three short
    for _ in 0..3 {
        led.set_low();
        delay.delay_millis(150);
        led.set_high();
        delay.delay_millis(150);
    }
    
    delay.delay_millis(200);
    
    // O: three long
    for _ in 0..3 {
        led.set_low();
        delay.delay_millis(450);
        led.set_high();
        delay.delay_millis(150);
    }
    
    delay.delay_millis(200);
    
    // S: three short
    for _ in 0..3 {
        led.set_low();
        delay.delay_millis(150);
        led.set_high();
        delay.delay_millis(150);
    }
}

fn breathing_pattern(led: &mut Output, delay: &Delay) {
    // Simulate breathing with variable timing
    let timings = [20, 30, 50, 70, 100, 120, 150, 120, 100, 70, 50, 30];
    
    for _ in 0..2 {
        // Breathing in (faster blinks)
        for &timing in timings.iter() {
            led.set_low();
            delay.delay_millis(timing / 2);
            led.set_high();
            delay.delay_millis(timing / 2);
        }
        
        // Hold breath
        delay.delay_millis(300);
        
        // Breathing out (slower blinks)
        for &timing in timings.iter().rev() {
            led.set_low();
            delay.delay_millis(timing / 2);
            led.set_high();
            delay.delay_millis(timing / 2);
        }
        
        delay.delay_millis(500);
    }
}

fn rapid_sequence(led: &mut Output, delay: &Delay) {
    // Rapid fire sequence
    for i in 0..10 {
        let speed = 50 + (i * 5); // Gradually slower
        led.set_low();
        delay.delay_millis(speed);
        led.set_high();
        delay.delay_millis(speed);
    }
}

fn double_pulse_pattern(led: &mut Output, delay: &Delay) {
    // Double pulse with varying intervals
    for _ in 0..5 {
        // First pulse
        led.set_low();
        delay.delay_millis(60);
        led.set_high();
        delay.delay_millis(120);
        
        // Second pulse
        led.set_low();
        delay.delay_millis(60);
        led.set_high();
        delay.delay_millis(300);
    }
}