#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Pin, Speed};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let ld3 = Output::new(p.PE9.degrade(), Level::Low, Speed::Low);
    let ld5 = Output::new(p.PE10.degrade(), Level::Low, Speed::Low);
    let ld7 = Output::new(p.PE11.degrade(), Level::Low, Speed::Low);
    let ld9 = Output::new(p.PE12.degrade(), Level::Low, Speed::Low);
    let ld10 = Output::new(p.PE13.degrade(), Level::Low, Speed::Low);
    let ld8 = Output::new(p.PE14.degrade(), Level::Low, Speed::Low);
    let ld6 = Output::new(p.PE15.degrade(), Level::Low, Speed::Low);
    let ld4 = Output::new(p.PE8.degrade(), Level::Low, Speed::Low);

    let mut circle: [Output<_>; 8] = [ld3, ld5, ld7, ld9, ld10, ld8, ld6, ld4];
    let mut index = 0;

    loop {
        let led = &mut circle[index];

        info!("high");
        led.set_high();
        Timer::after(Duration::from_millis(100)).await;

        info!("low");
        led.set_low();
        Timer::after(Duration::from_millis(100)).await;

        index = (index + 1) % 8;
    }
}
