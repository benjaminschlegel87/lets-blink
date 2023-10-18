use embedded_hal::digital::v2::OutputPin;
/// Enumeration of the possible LED States
#[derive(Debug, Clone, Copy)]
pub enum LedState {
    On,
    Off,
}
/// LED Abstraction over a [OutputPin].
pub struct Led<T: OutputPin> {
    pin: T,
    state: LedState,
}

impl<T: OutputPin> Led<T> {
    /// Creates a new LED with given inital [LedState] from any [OutputPin]
    pub fn new(pin: T, inital_state: LedState) -> Self {
        let mut led = Self {
            pin,
            state: inital_state,
        };
        match inital_state {
            LedState::On => led.on(),
            LedState::Off => led.off(),
        }

        led
    }
    /// Turns the LED on (setting the inner GPIO high)
    pub fn on(&mut self) {
        self.pin.set_high().unwrap_or_default();
        self.state = LedState::On;
    }
    /// Turns the LED off(setting the inner GPIO low)
    pub fn off(&mut self) {
        self.pin.set_low().unwrap_or_default();
        self.state = LedState::Off;
    }
    /// Toggles the LED
    pub fn toggle(&mut self) {
        match self.state {
            LedState::On => self.off(),
            LedState::Off => self.on(),
        }
    }
    /// Returns the [LedState]
    pub fn get_state(&self) -> LedState {
        self.state
    }
}
