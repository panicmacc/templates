#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

use microbit::{
    hal::{
        prelude::*,
        twim,
        uarte::{
            self,
            Baudrate,
            Parity,
            Uarte
        },
    },
    pac::{
        twim0::frequency::FREQUENCY_A,
        UARTE0,
    },
};

use lsm303agr::{
    AccelOutputDataRate,
    MagOutputDataRate,
    Lsm303agr,
};

mod serial_setup;
use serial_setup::UartePort;

use heapless::Vec;

const ACCELEROMETER_ADDR: u8 = 0b0011001;
const MAGNETOMETER_ADDR: u8 = 0b0011110;

const ACCELEROMETER_ID_REG: u8 = 0x0f;
const MAGNETOMETER_ID_REG: u8 = 0x4f;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut serial = {
        let serial = Uarte::new(
            board.UARTE0, 
            board.uart.into(), 
            Parity::EXCLUDED, 
            Baudrate::BAUD115200
        );
        UartePort::new(serial)
    };
    let mut uartbuf: Vec<u8, 32> = Vec::new();

    // let mut acc = [0];
    // let mut mag = [0];
    // First write the address + register onto the bus, then read the chip's responses
    // i2c.write_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut acc).unwrap();
    // i2c.write_read(MAGNETOMETER_ADDR, &[MAGNETOMETER_ID_REG], &mut mag).unwrap();
    // rprintln!("The accelerometer chip's id is: {:#b}", acc[0]);
    // rprintln!("The magnetometer chip's id is: {:#b}", mag[0]);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz50).unwrap();

    loop {
        match get_uart_command(&mut uartbuf, &mut serial) {
            Some(cmd) => {
                match &cmd[..] {
                    "acc" => {
                        if let Ok(data) = sensor.accel_data() {
                            rprintln!("Acceleration: x {} y {} z {}", data.x, data.y, data.z);
                        } else {
                            rprintln!("No Acceleration Data Available.");
                        }
                        // if sensor.accel_status().unwrap().xyz_new_data {
                        //     let data = sensor.accel_data().unwrap();
                        //     rprintln!("Acceleration: x {} y {} z {}", data.x, data.y, data.z);
                        // } else {
                        //     rprintln!("No Acceleration Data Available.");
                        // }
                    }
                    "mag" => {
                        if let data = nb::block!(sensor.mag_data()).unwrap() {
                            rprintln!("Compass: x {} y {} z {}", data.x, data.y, data.z);
                        } else {
                            rprintln!("No Compass Data Available.");
                        }
                        // if sensor.mag_status().unwrap().xyz_new_data {
                        //     let data = sensor.mag_data().unwrap();
                        //     rprintln!("Compass: x {} y {} z {}", data.x, data.y, data.z);
                        // } else {
                        //     rprintln!("No Compass Data Available.");
                        // }
                    }
                    _ => {
                        rprintln!("Unknown command: {}", cmd);
                    }
                }
            }
            _ => {}
        }

        //i2c.write_read(ACCELEROMETER_ADDR, &ACCELEROMETER_OUT_REGISTERS, &mut acc).unwrap();
        //rprintln!("The accelerometer says: {:#?}", acc);
        //i2c.write_read(MAGNETOMETER_ADDR, &[MAGNETOMETER_ID_REG], &mut mag).unwrap();
    }

    fn get_uart_command(buf: &mut Vec<u8, 32>, serial: &mut UartePort<UARTE0>) -> Option<heapless::String<32>> {
        let rcv_byte = nb::block!(serial.read()).unwrap();
        match rcv_byte {
            13 => {
                let cmd = core::str::from_utf8(&buf[..]).unwrap();
                let cmd: heapless::String<32> = cmd.into();
                buf.clear();
                Some(cmd)
            }
            _ => {
                buf.push(rcv_byte);
                None
            }
        }
    }
}
