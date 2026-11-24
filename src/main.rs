#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    gpio::{Input, Level, Output, OutputDrive, Pull},
    peripherals,
    spim::{self, Spim},
};
use embassy_time::{Delay, Timer};
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_hal_bus::spi::ExclusiveDevice;
use mipidsi::{Builder, interface::SpiInterface, models::ST7789, options::ColorInversion};
use panic_probe as _;

bind_interrupts!(struct Irqs {
    TWISPI0 => spim::InterruptHandler<peripherals::TWISPI0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    let mut cfg = spim::Config::default();
    cfg.frequency = spim::Frequency::M8;
    cfg.mode = spim::MODE_3;

    // serial peripheral interface master (SPI) for the display
    let spi = Spim::new_txonly(p.TWISPI0, Irqs, p.P0_02, p.P0_03, cfg);
    // chip select pin for the display
    let cs = Output::new(p.P0_25, Level::High, OutputDrive::Standard);
    // command/data pin for the display
    let dc = Output::new(p.P0_18, Level::Low, OutputDrive::Standard);
    // reset pin for the display
    let rst = Output::new(p.P0_26, Level::High, OutputDrive::Standard);
    // backlight pin for the display (low is P14, medium is P22, high is P23)
    let _bl = Output::new(p.P0_22, Level::Low, OutputDrive::Standard);
    // device for the display
    let spi_dev = ExclusiveDevice::new_no_delay(spi, cs).unwrap();
    // scratch buffer for the display
    let mut buffer = [0u8; 512];
    // display interface
    let di = SpiInterface::new(spi_dev, dc, &mut buffer);

    let mut display = Builder::new(ST7789, di)
        .display_size(240, 240)
        .invert_colors(ColorInversion::Inverted)
        .reset_pin(rst)
        .init(&mut Delay)
        .unwrap();

    display.clear(Rgb565::GREEN).unwrap();

    // enable the button
    let mut button_enable = Output::new(p.P0_15, Level::Low, OutputDrive::Standard);
    // enabling the button can be finicky, so we set it high multiple times to ensure it is enabled
    button_enable.set_high();
    button_enable.set_high();
    button_enable.set_high();
    button_enable.set_high();

    let mut button = Input::new(p.P0_13, Pull::Down);

    loop {
        button.wait_for_high().await;

        // debounce
        Timer::after_millis(20).await;
        if !button.is_high() {
            continue;
        }
        info!("Button pressed!");
        display.clear(Rgb565::BLUE).unwrap();

        loop {
            button.wait_for_low().await;

            // debounce
            Timer::after_millis(20).await;
            if button.is_low() {
                break;
            }
        }
        info!("Button released!");
        display.clear(Rgb565::GREEN).unwrap();
    }
}
