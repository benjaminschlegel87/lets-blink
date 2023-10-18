#![no_main]
#![no_std]

// gives a panic_handler with a endless loop
use core::sync::atomic::AtomicUsize;
use cortex_m_rt::exception;
use panic_halt as _;
use stm32f3::stm32f303::{CorePeripherals, Peripherals};
static CNT: AtomicUsize = AtomicUsize::new(0);
#[cortex_m_rt::entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    p.RCC.ahbenr.modify(|_, w| w.iopeen().enabled());
    p.GPIOE.moder.modify(|_, w| w.moder10().output());
    p.GPIOE.bsrr.write(|w| w.bs10().set());

    let mut core = CorePeripherals::take().unwrap();
    core.SYST.set_reload(1_000);
    core.SYST.enable_counter();
    core.SYST.enable_interrupt();
    loop {
        if CNT
            .compare_exchange(
                1000,
                0,
                core::sync::atomic::Ordering::Relaxed,
                core::sync::atomic::Ordering::Relaxed,
            )
            .is_ok()
        {
            if p.GPIOE.odr.read().odr10().is_high() {
                p.GPIOE.bsrr.write(|w| w.br10().set_bit());
            } else {
                p.GPIOE.bsrr.write(|w| w.bs10().set());
            }
        }
    }
}

#[exception]
fn SysTick() {
    CNT.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
}
