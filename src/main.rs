#![no_std]
#![no_main]

mod board;

use board::Board;
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let config = Config::default();
    let p = embassy_stm32::init(config);
    let mut board = Board::new(p);

    loop {
        info!("Hello World!");
        board.ld1.toggle();
        board.ld2.toggle();
        board.ld3.toggle();
        Timer::after_secs(1).await;
    }
}
