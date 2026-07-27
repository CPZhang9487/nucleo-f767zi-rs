use crate::led::Leds;
use embassy_stm32::{
    Peripherals,
    gpio::{Level, Output, Speed},
};

pub struct Board {
    pub leds: Leds,
}

impl Board {
    pub fn new(p: Peripherals) -> Self {
        let led_green = Output::new(p.PB0, Level::Low, Speed::Low);
        let led_blue = Output::new(p.PB7, Level::Low, Speed::Low);
        let led_red = Output::new(p.PB14, Level::Low, Speed::Low);
        Self {
            leds: Leds::new(led_green, led_blue, led_red),
        }
    }
}
