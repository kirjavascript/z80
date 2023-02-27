use crate::z80::{z80, z80Mem};

#[derive(Debug)]
struct Memory {
    mem: [u8; 0x10000],
}

impl z80Mem for Memory {
    fn read_byte(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        self.mem[addr as usize] = value;
    }
}


#[test]
pub fn main_0() {
    let memory = Memory {
        mem: [0; 0x10000],
    };
    let cpu = z80::new(Box::new(memory));

}
