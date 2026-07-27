#![no_std]
#![no_main]
#![allow(dead_code)]

mod board;
mod led;

use board::Board;
use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::Timer;
use led::{LED_CHANNEL, LedCommand, LedId, task::led_task};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let config = Config::default();
    let p = embassy_stm32::init(config);
    let board = Board::new(p);

    spawner.spawn(unwrap!(led_task(board.leds)));

    loop {
        info!("Hello World!");
        LED_CHANNEL.send(LedCommand::Toggle(LedId::Green)).await;
        LED_CHANNEL.send(LedCommand::Toggle(LedId::Blue)).await;
        LED_CHANNEL.send(LedCommand::Toggle(LedId::Red)).await;
        Timer::after_secs(1).await;
    }
}
