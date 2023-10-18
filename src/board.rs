use crate::led::{self};
use stm32f3xx_hal::{
    gpio::{Gpioe, Output, Pin, PushPull, U},
    pac::{Peripherals, TIM1, TIM2},
    prelude::{_stm32f3xx_hal_flash_FlashExt, _stm32f3xx_hal_gpio_GpioExt},
    rcc::RccExt,
    timer,
};
pub type OrangeLed = led::Led<Pin<Gpioe, U<10>, Output<PushPull>>>;
pub type RedLed = led::Led<Pin<Gpioe, U<9>, Output<PushPull>>>;

pub struct Board {
    pub orange_led: OrangeLed,
    pub timer: timer::Timer<TIM1>,
    pub red_led: RedLed,
    pub timer_red: timer::Timer<TIM2>,
}
impl Board {
    pub fn new(p: Peripherals) -> Self {
        let mut rcc = p.RCC.constrain();
        let gpioe = p.GPIOE;
        let mut pins = gpioe.split(&mut rcc.ahb);
        let pe10 = pins.pe10;
        let pe10_output = pe10.into_push_pull_output(&mut pins.moder, &mut pins.otyper);
        let pe9_output = pins
            .pe9
            .into_push_pull_output(&mut pins.moder, &mut pins.otyper);
        let mut flash = p.FLASH.constrain();
        let clocks = rcc.cfgr.freeze(&mut flash.acr);
        let timer = timer::Timer::new(p.TIM1, clocks, &mut rcc.apb2);
        let timer_red = timer::Timer::new(p.TIM2, clocks, &mut rcc.apb1);
        let orange_led: OrangeLed = led::Led::new(pe10_output, led::LedState::Off);
        let red_led: RedLed = led::Led::new(pe9_output, led::LedState::Off);

        Self {
            orange_led,
            timer,
            red_led,
            timer_red,
        }
    }
}
