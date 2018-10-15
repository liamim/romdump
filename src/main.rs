#![feature(uniform_paths)]
#![no_std]
#![no_main]

extern crate panic_semihosting;
// extern crate panic_halt;
// extern crate stm32f4xx_hal;
use stm32f4xx_hal as hal;

#[macro_use]
mod pinblock;
mod address_lines;
mod data_lines;

use cortex_m_rt::entry;
use nb::block;
use hal::hal::digital::OutputPin;
use hal::prelude::*;
use hal::serial::{config::Config, Serial};
use hal::stm32;

use core::fmt::Write;
use cortex_m_semihosting::{debug, hio};

use address_lines::AddressLines;
use data_lines::DataLines;

trait OutputPinBool: OutputPin {
    fn set_value(&mut self, val: bool) {
        if val {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}

impl<T> OutputPinBool for T where T: OutputPin {}


#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();

    let p = hal::stm32::Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();
    let gpiob = p.GPIOB.split();
    let gpioc = p.GPIOC.split();
    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let txp = gpioa.pa2.into_alternate_af7();
    let rxp = gpioa.pa3.into_alternate_af7();

    let serial = Serial::usart2(
        p.USART2,
        (txp, rxp),
        Config::default().baudrate(115_200.bps()),
        clocks,
    )
    .unwrap();

    // Separate out the sender and receiver of the serial port
    let (mut tx, mut rx_) = serial.split();
/*
    // ---- pins for ROM ----
    let mut address_lines = AddressLines::new(
        // TODO: fixme
        // find 19 pins and into_push_pull_output() them.
    );
*/
    let data_lines = DataLines::new(
        gpioa.pa5.into_floating_input(),
        gpioa.pa6.into_floating_input(),
        gpioa.pa7.into_floating_input(),
        gpiob.pb6.into_floating_input(),
        gpioc.pc7.into_floating_input(),
        gpioa.pa9.into_floating_input(),
        gpioa.pa8.into_floating_input(),
        gpiob.pb10.into_floating_input(),
    );

    loop {
        let val = data_lines.read();
        writeln!(tx, "{:08b}", val);   
    }
}
