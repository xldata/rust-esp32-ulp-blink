#![no_std]
#![no_main]

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;

use esp_idf_hal::delay;
use esp_idf_hal::prelude::*;
use esp_idf_hal::gpio::*;

extern crate panic_halt;

#[no_mangle]
static mut CYCLES: u32 = 2;

#[no_mangle]
fn main() {
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let mut delay = delay::Ulp;
    let mut led = PinDriver::output(pins.gpio4).unwrap();

    while get_cycles() > 0 {
        led.set_high().unwrap();
        delay.delay_ms(1000_u32);

        led.set_low().unwrap();
        delay.delay_ms(1000_u32);

        decr_cycles();
    }
}

fn get_cycles() -> u32 {
    unsafe { core::ptr::read_volatile(&CYCLES) }
}

fn decr_cycles() {
    unsafe {
        let cycles = core::ptr::read_volatile(&CYCLES);

        if cycles > 0 {
            core::ptr::write_volatile(&mut CYCLES, cycles - 1);
        }
    }
}
