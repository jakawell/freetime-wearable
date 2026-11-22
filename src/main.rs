#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::{
    gpio::{Level, Output, OutputDrive, Pull},
    gpiote::{InputChannel, InputChannelPolarity},
};
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    // enable the button
    let mut button_enable = Output::new(p.P0_15, Level::Low, OutputDrive::Standard);
    // enabling the button can be finicky, so we set it high multiple times to ensure it is enabled
    button_enable.set_high();
    button_enable.set_high();
    button_enable.set_high();
    button_enable.set_high();

    let mut channel = InputChannel::new(
        p.GPIOTE_CH0,
        p.P0_13,
        Pull::Down,
        InputChannelPolarity::LoToHi,
    );

    loop {
        channel.wait_for_high().await;
        info!("Button pressed!");
    }
}
