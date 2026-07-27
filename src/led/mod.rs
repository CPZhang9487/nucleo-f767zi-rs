pub mod task;

use embassy_stm32::gpio::Output;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};

struct Led {
    pin: Output<'static>,
}

impl Led {
    fn new(pin: Output<'static>) -> Self {
        Self { pin }
    }

    fn on(&mut self) {
        self.pin.set_high();
    }

    fn off(&mut self) {
        self.pin.set_low();
    }

    fn toggle(&mut self) {
        self.pin.toggle()
    }
}

pub struct Leds {
    green: Led,
    blue: Led,
    red: Led,
}

impl Leds {
    pub fn new(green: Output<'static>, blue: Output<'static>, red: Output<'static>) -> Self {
        Self {
            green: Led::new(green),
            blue: Led::new(blue),
            red: Led::new(red),
        }
    }
}

#[derive(Clone, Copy)]
pub enum LedId {
    Green,
    Blue,
    Red,
}

pub enum LedCommand {
    On(LedId),
    Off(LedId),
    Toggle(LedId),
}

pub static LED_CHANNEL: Channel<CriticalSectionRawMutex, LedCommand, 8> = Channel::new();
