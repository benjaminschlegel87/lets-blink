#![no_main]
#![no_std]

// gives a panic_handler with a endless loop
use panic_halt as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    loop {}
}
