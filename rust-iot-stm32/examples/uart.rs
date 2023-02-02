#![no_std]
#![no_main]

#[cfg(feature = "f103")]
use stm32f1xx_hal as hal;

use cortex_m_semihosting::hprintln;
use hal::{pac, prelude::*, timer::Timer};
use nb::block;

#[cortex_m_rt::entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let mut gpioc = dp.GPIOC.split();

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(1.Hz()).unwrap();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = p.AFIO.constrain();
    let mut gpioa = p.GPIOA.split();

    // USART1 on Pins A9 and A10
    let pin_tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let pin_rx = gpioa.pa10;
    // Create an interface struct for USART1 with 9600 Baud
    let serial = Serial::usart1(
       p.USART1,
       (pin_tx, pin_rx),
       &mut afio.mapr,
       Config::default()
           .baudrate(9_600.bps())
           .wordlength_9bits()
           .parity_none(),
       clocks,
    );
    
    // Switching the 'Word' type parameter for the 'Read' and 'Write' traits between u8 and u16.
    let serial = serial.with_u16_data();
    let serial = serial.with_u8_data();
    
    
    // Separate into tx and rx channels
    let (mut tx, mut rx) = serial.split();

    // Switch tx to u16.
    let mut tx = tx.with_u16_data();

    // Write data to the USART.
    // Depending on the configuration, only the lower 7, 8, or 9 bits are used.
    block!(tx.write(0x1FF)).ok();

    // Switch tx back to u8
    let mut tx = tx.with_u8_data();

    // Write 'R' to the USART
    block!(tx.write(b'R')).ok();

    // Switch rx to u16.
    let mut rx = rx.with_u16_data();

    // Receive a data from the USART and store it in "received"
    let received: u16 = block!(rx.read()).unwrap();

    // Switch rx back to u8.
    let mut rx = rx.with_u8_data();

    // Receive a data from the USART and store it in "received"
    let received: u8 = block!(rx.read()).unwrap();
    
    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        block!(timer.wait()).unwrap();
        led.set_high();
        hprintln!("Blinky!");
        block!(timer.wait()).unwrap();
        led.set_low();
    }
}

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
   loop {
       cortex_m::asm::bkpt();
   }
}
