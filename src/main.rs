#![no_std]
#![no_main]
#![allow(dead_code)]

mod board;
mod gpio;
mod itm_debug;
mod led;
mod mcu;
mod reg;

use cortex_m::peripheral::syst;
use cortex_m::Peripherals;
use cortex_m_rt::{entry, exception};
use panic_halt as _;

static mut PERIPHERALS: Option<Peripherals> = None;
#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();
    led::led_init(board::BLUE_LED_PORT, board::BLUE_LED_PIN);
    led::led_on(board::BLUE_LED_PORT, board::BLUE_LED_PIN);

    // Store peripherals globally
    let mut peripherals = Peripherals::take().unwrap();
    systick_init(&mut peripherals.SYST, 2);

    loop {
        rtt_target::rprintln!("hello,from main!");
    }
}

#[exception]
fn SysTick() {
    led::led_toggle(board::BLUE_LED_PORT, board::BLUE_LED_PIN);

    rtt_target::rprintln!("hello,from systick!");
    
}

fn systick_init(systick: &mut cortex_m::peripheral::SYST, freq: u32) {
    systick.set_clock_source(syst::SystClkSource::Core);
    systick.set_reload(16_000_000 / freq - 1); /* 16Mhz /2HZ */
    systick.clear_current();
    systick.enable_interrupt();
    systick.enable_counter();
}
