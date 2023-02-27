use crate::z80::{z80, z80Ctrl};

struct Memory {
    mem: [u8; 0x10000],
}

impl z80Ctrl for Memory {
    fn read_byte(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        self.mem[addr as usize] = value;
    }
    fn port_in(&self, addr: u16) -> u8 {
        0
    }
    fn port_out(&mut self, addr: u16, value: u8) {
    }
}

#[test]
pub fn main() {
    let memory = Box::new(Memory {
        mem: [0; 0x10000],
    });
    let cpu = z80::new(memory);

}
