use super::{LED_CHANNEL, LedCommand, LedId, Leds};
use embassy_executor;

#[embassy_executor::task]
pub async fn led_task(mut leds: Leds) {
    loop {
        let cmd = LED_CHANNEL.receive().await;

        match cmd {
            LedCommand::On(LedId::Green) => leds.green.on(),
            LedCommand::On(LedId::Blue) => leds.blue.on(),
            LedCommand::On(LedId::Red) => leds.red.on(),

            LedCommand::Off(LedId::Green) => leds.green.off(),
            LedCommand::Off(LedId::Blue) => leds.blue.off(),
            LedCommand::Off(LedId::Red) => leds.red.off(),

            LedCommand::Toggle(LedId::Green) => leds.green.toggle(),
            LedCommand::Toggle(LedId::Blue) => leds.blue.toggle(),
            LedCommand::Toggle(LedId::Red) => leds.red.toggle(),
        }
    }
}
