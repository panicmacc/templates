#![no_std]
#![no_main]

#[cfg(feature = "52833")]
use nrf52833_hal as hal;

// use core::fmt::Write;
// use hal::{
    // gpio,
    // pac::{self, UARTE0},
    // uarte::{self, Baudrate, Parity, Uarte},
// };
// use heapless::Vec;
use rtt_target::{rprintln, rtt_init_print};

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    // let _p = pac::Peripherals::take().unwrap();

    rprintln!("It's happening!");

    loop {
        rprintln!("It's happening!");
        cortex_m::asm::wfi();
    }

    // #[cfg(feature = "52833")]
    // let (uart0, cdc_pins) = {
    //     let p0 = gpio::p0::Parts::new(p.P0);
    //     (
    //         p.UARTE0,
    //         uarte::Pins {
    //             txd: p0.p0_06.into_push_pull_output(gpio::Level::High).degrade(),
    //             rxd: p0.p0_08.into_floating_input().degrade(),
    //             cts: Some(p0.p0_07.into_floating_input().degrade()),
    //             rts: Some(p0.p0_05.into_push_pull_output(gpio::Level::High).degrade()),
    //         },
    //     )
    // };

    // let mut uarte = Uarte::new(
    //     uart0,
    //     cdc_pins,
    //     uarte::Parity::EXCLUDED,
    //     uarte::Baudrate::BAUD115200,
    // );

    // write!(uarte, "Hello, World!\r\n").unwrap();
    // // rprintln!("Foobat.");
    // loop {
    //     write!(uarte, "Hello, Again!\r\n").unwrap();
    //     // rprintln!("Hello, nRF!\r\n");
    //     cortex_m::asm::wfi();
    // }
}

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
   loop {
       cortex_m::asm::bkpt();
   }
}
