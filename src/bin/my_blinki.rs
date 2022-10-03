//attempt
#![deny(unsafe_code)]
#![no_main]
#![no_std]
#![allow(clippy::empty_loop)]

use my_app as _; // global logger + panicking-behavior + memory layout

use stm32f4xx_hal as hal;

use hal::{pac, prelude::*};

#[cortex_m_rt::entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = cp.SYST.delay(&clocks);

        //loop {
        for _ in 0..10 {
            // On for 1s, off for 1s.
            led.set_high();
            delay.delay_ms(1000_u32);
            led.set_low();
            delay.delay_ms(1000_u32);
        }
        defmt::println!("test word");
    }
    my_app::exit();

    //loop {}
}
