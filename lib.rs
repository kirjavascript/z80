#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]


mod z80;
pub use z80::*;

#[cfg(test)]
mod z80_tests;
