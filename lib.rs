#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

//! A fast and accurate instruction-stepped Z80 emulator written in C11 and ported to Rust
//!
//! Basic usage:
//! ```rust
//!use z80::{Z80, Z80_io};
//!
//!struct IO {
//!    pub mem: [u8; 0x10000],
//!}
//!
//!impl Z80_io for IO {
//!    fn read_byte(&self, addr: u16) -> u8 {
//!        self.mem[addr as usize]
//!    }
//!
//!    fn write_byte(&mut self, addr: u16, value: u8) {
//!        self.mem[addr as usize] = value;
//!    }
//!    fn port_in(&self, _addr: u16) -> u8 { 0xff }
//!    fn port_out(&mut self, _addr: u16, _value: u8) { }
//!}
//!
//!let mut cpu = Z80::new(IO {
//!    mem: [0; 0x10000],
//!});
//!
//!let rom = vec![0, 0, 0]; // etc
//!
//!for (i, byte) in rom.iter().enumerate() {
//!    cpu.write_byte(i as _, *byte);
//!}
//!
//!loop {
//!    cpu.step();
//!    break;
//!}
//!```

mod z80;
pub use self::z80::*;

#[cfg(test)]
mod z80_tests;
