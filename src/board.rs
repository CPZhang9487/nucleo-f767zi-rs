use embassy_stm32::{
    Peripherals,
    gpio::{Level, Output, Speed},
};

#[allow(dead_code)]
pub struct Board {
    pub ld1: Output<'static>,
    pub ld2: Output<'static>,
    pub ld3: Output<'static>,
}

impl Board {
    pub fn new(p: Peripherals) -> Self {
        Self {
            ld1: Output::new(p.PB0, Level::Low, Speed::Low),
            ld2: Output::new(p.PB7, Level::Low, Speed::Low),
            ld3: Output::new(p.PB14, Level::Low, Speed::Low),
        }
    }
}
