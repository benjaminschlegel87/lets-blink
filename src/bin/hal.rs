#![no_main]
#![no_std]
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::timer::CountDown;
use embedded_time::duration::*;
use nb::block;
use panic_halt as _;
use stm32f3xx_hal as _;
use stm32f3xx_hal::pac::Peripherals;
use stm32f3xx_hal::prelude::{_stm32f3xx_hal_flash_FlashExt, _stm32f3xx_hal_gpio_GpioExt};
use stm32f3xx_hal::rcc::RccExt;
use stm32f3xx_hal::timer;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let gpioe = p.GPIOE;
    let mut pins = gpioe.split(&mut rcc.ahb);
    let pe10 = pins.pe10;
    let mut pe10_output = pe10.into_push_pull_output(&mut pins.moder, &mut pins.otyper);
    let mut flash = p.FLASH.constrain();

    let mut timer = timer::Timer::new(p.TIM1, rcc.cfgr.freeze(&mut flash.acr), &mut rcc.apb2);

    loop {
        timer.start(1000.milliseconds());
        pe10_output.set_low().unwrap_or_default();
        block!(timer.wait()).unwrap();
        pe10_output.set_high().unwrap_or_default();
        timer.start(1000.milliseconds());
        block!(timer.wait()).unwrap();
    }
}
