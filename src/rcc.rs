use embassy_stm32::{rcc, time};

pub fn init(rcc_config: &mut rcc::Config) {
    rcc_config.hse = Some(rcc::Hse {
        freq: time::Hertz(8_000_000),
        mode: rcc::HseMode::Bypass,
    });

    rcc_config.pll_src = rcc::PllSource::HSE;

    rcc_config.pll = Some(rcc::Pll {
        prediv: rcc::PllPreDiv::DIV4,
        mul: rcc::PllMul::MUL96,
        divp: Some(rcc::PllPDiv::DIV2),
        divq: Some(rcc::PllQDiv::DIV4),
        divr: None,
    });

    rcc_config.sys = rcc::Sysclk::PLL1_P;

    rcc_config.ahb_pre = rcc::AHBPrescaler::DIV1;

    rcc_config.apb1_pre = rcc::APBPrescaler::DIV2;

    rcc_config.apb2_pre = rcc::APBPrescaler::DIV1;
}
