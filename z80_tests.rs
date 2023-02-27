use crate::z80::{z80, z80Ctrl};

struct Memory {
    pub mem: [u8; 0x10000],
    pub test_finished: bool,
}

impl z80Ctrl for Memory {
    fn read_byte(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        self.mem[addr as usize] = value;
    }
    fn port_in(&self, _addr: u16) -> u8 {
        0xff
    }
    fn port_out(&mut self, _addr: u16, _value: u8) {
        self.test_finished = true;
    }
    fn test_finished(&self) -> bool {
        self.test_finished
    }
}

fn run_test(rom: &[u8], cyc_expected: u64) {
    let mut memory = Box::new(Memory {
        mem: [0; 0x10000],
        test_finished: false,
    });

    for (i, byte) in rom.iter().enumerate() {
        memory.mem[0x100 + i] = *byte;
    }

    let mut cyc: u64 = 0;

    let mut cpu = z80::new(memory);
    cpu.init();
    cpu.pc = 0x100;
    cpu.ctrl.write_byte(0,0xd3);
    cpu.ctrl.write_byte(1,0);
    cpu.ctrl.write_byte(5,0xdb);
    cpu.ctrl.write_byte(6,0);
    cpu.ctrl.write_byte(7,0xc9);

    let mut nb_instructions: u64 = 0;
    while !cpu.ctrl.test_finished() {
        nb_instructions += 1;
        cyc = cyc.wrapping_add(cpu.step().into());
    }
    let diff = cyc_expected.wrapping_sub(cyc);
    println!(
        "\n*** {} instructions executed on {} cycles (expected={}, diff={})\n\n",
        nb_instructions,
        cyc,
        cyc_expected,
        diff,
    );

    assert_eq!(cyc, cyc_expected);
}

#[test]
pub fn main() {
    run_test(include_bytes!("./roms/prelim.com"), 8721);
    run_test(include_bytes!("./roms/zexdoc.cim"), 46734978649);
    run_test(include_bytes!("roms/zexall.cim"), 46734978649);
}
