//! Prints "Hello, world" on the OpenOCD console

#![no_std]

extern crate cortex_m_semihosting as sh;
extern crate stm32f103xx_hal;

use core::fmt::Write;

use sh::hio;

fn main() {
    let mut hstdout = hio::hstdout().unwrap();

    writeln!(hstdout, "Hello, world!").unwrap();
}
