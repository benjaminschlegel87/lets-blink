#![no_main]
#![no_std]

use core::convert::Infallible;
use core::future::Future;
use core::pin::{pin, Pin};
use core::task::{RawWaker, RawWakerVTable};
use embedded_hal::timer::CountDown;
use embedded_time::duration::*;
use lets_blink::board::{self, OrangeLed};
use panic_halt as _;
use stm32f3xx_hal::pac::{Peripherals, TIM1, TIM2};
use stm32f3xx_hal::timer::Timer;

/// Async Timer Wrapper around any normal Timer implementing [embedded_hal::timer::CountDown]
struct AsyncTimer<'a, T: CountDown> {
    timer: &'a mut T,
}
impl<'a, T: CountDown> AsyncTimer<'a, T> {
    /// Create a new Async Timer by borrowing the [CountDown] Timer and start it with the given delay
    pub fn new<E: Into<T::Time>>(timer: &'a mut T, delay: E) -> Self {
        timer.start(delay);
        Self { timer }
    }
}
/// Implementing [core::future::Future] trait for [AsyncTimer]
impl<'a, T: CountDown> Future for AsyncTimer<'a, T> {
    type Output = ();
    fn poll(
        mut self: core::pin::Pin<&mut Self>,
        _cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        if let Ok(_) = self.timer.wait() {
            return core::task::Poll::Ready(());
        } else {
            return core::task::Poll::Pending;
        }
    }
}
/// Async provides Waker possibility to only wake async tasks if progress is exepcted
/// we use a no operation waker as we just poll futures without sleeping
static NOOP_VTABLE: RawWakerVTable = RawWakerVTable::new(
    |x| RawWaker::new(x, &NOOP_VTABLE), // clone
    |_| (),                             // wake
    |_| (),                             // wake_by_ref
    |_| (),                             // drop
);
/// absolut minimal async executor
fn execute(task: &mut [Pin<&mut dyn Future<Output = Infallible>>]) -> ! {
    let w = unsafe { core::task::Waker::from_raw(RawWaker::new(core::ptr::null(), &NOOP_VTABLE)) };
    let mut c = core::task::Context::from_waker(&w);
    loop {
        for t in task.iter_mut() {
            match t.as_mut().poll(&mut c) {
                core::task::Poll::Ready(_) => unreachable!(),
                core::task::Poll::Pending => (),
            }
        }
    }
}

/// Async Task Blinking Orange LED with 1Hz
async fn blinky_task(mut orange_led: OrangeLed, mut timer: Timer<TIM1>) -> Infallible {
    loop {
        orange_led.toggle();
        AsyncTimer::new(&mut timer, 1000.milliseconds()).await;
    }
}
/// Async Task Blinking Red LED with 1Hz
async fn blinky_red_task(mut red_led: board::RedLed, mut timer: Timer<TIM2>) -> Infallible {
    loop {
        red_led.toggle();
        AsyncTimer::new(&mut timer, 500.milliseconds()).await;
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    // Take peripherals
    let p = Peripherals::take().unwrap();
    // Create Board abstraction
    let board = board::Board::new(p);
    // pin async tasks
    let task = pin!(blinky_task(board.orange_led, board.timer));
    let task_red = pin!(blinky_red_task(board.red_led, board.timer_red));
    // run executor forever
    execute(&mut [task, task_red]);
}
