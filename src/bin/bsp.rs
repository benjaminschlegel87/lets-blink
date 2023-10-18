#![no_main]
#![no_std]

use embedded_hal::timer::CountDown;
use embedded_time::duration::*;
use lets_blink::board;
use nb::block;
use panic_halt as _;
use stm32f3xx_hal::pac::Peripherals;
#[cortex_m_rt::entry]
fn main() -> ! {
    // Take peripherals
    let p = Peripherals::take().unwrap();
    // Create Board abstraction
    let board = board::Board::new(p);
    let mut timer = board.timer;
    let mut orange_led = board.orange_led;
    loop {
        // start Timer
        timer.start(1000.milliseconds());
        // block until elapsed
        block!(timer.wait()).unwrap();
        // toggle LED
        orange_led.toggle();
    }
}
