/*

MIT License

Copyright (c) 2019 Nicolas Allemand
Copyright (c) 2020-2022 Rupert Carmichael
Copyright (c) 2022 rofl0r
Copyright (c) 2023 Kirjava

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

*/

pub type int8_t = i8;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type z80_flagbit = u32;

pub const Z80_ASSERT: u32 = 2;
pub const Z80_PULSE: u32 = 1;
pub const sf: z80_flagbit = 7;
pub const zf: z80_flagbit = 6;
pub const yf: z80_flagbit = 5;
pub const hf: z80_flagbit = 4;
pub const xf: z80_flagbit = 3;
pub const pf: z80_flagbit = 2;
pub const nf: z80_flagbit = 1;
pub const cf: z80_flagbit = 0;

pub trait z80Ctrl {
    fn read_byte(&self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, value: u8);
    fn port_in(&self, addr: u16) -> u8;
    fn port_out(&mut self, addr: u16, value: u8);
}

pub struct z80 {
    pub ctrl: Box<dyn z80Ctrl>,
    pub pc: uint16_t,
    pub sp: uint16_t,
    pub ix: uint16_t,
    pub iy: uint16_t,
    pub mem_ptr: uint16_t,
    pub c2rust_unnamed: C2RustUnnamed_14,
    pub c2rust_unnamed_0: C2RustUnnamed_12,
    pub c2rust_unnamed_1: C2RustUnnamed_10,
    pub c2rust_unnamed_2: C2RustUnnamed_8,
    pub c2rust_unnamed_3: C2RustUnnamed_6,
    pub c2rust_unnamed_4: C2RustUnnamed_4,
    pub c2rust_unnamed_5: C2RustUnnamed_2,
    pub c2rust_unnamed_6: C2RustUnnamed_0,
    pub i: uint8_t,
    pub r: uint8_t,
    pub iff_delay: uint8_t,
    pub interrupt_mode: uint8_t,
    pub irq_data: uint8_t,
    pub irq_pending: uint8_t,
    pub nmi_pending: uint8_t,
    halted: bool,
    iff1: bool,
    iff2: bool,
    f_szpxy: [uint8_t; 256],
    #[cfg(test)]
    pub test_finished: bool,
}

impl z80 {
    pub fn new(ctrl: Box<dyn z80Ctrl>) -> Self {
        z80 {
            ctrl,
            pc: 0,
            sp: 0,
            ix: 0,
            iy: 0,
            mem_ptr: 0,
            c2rust_unnamed: C2RustUnnamed_14 {
                c2rust_unnamed: C2RustUnnamed_15 { f: 0, a: 0, },
            },
            c2rust_unnamed_0: C2RustUnnamed_12 {
                c2rust_unnamed: C2RustUnnamed_13 { c: 0, b: 0, },
            },
            c2rust_unnamed_1: C2RustUnnamed_10 {
                c2rust_unnamed: C2RustUnnamed_11 { e: 0, d: 0, },
            },
            c2rust_unnamed_2: C2RustUnnamed_8 {
                c2rust_unnamed: C2RustUnnamed_9 { l: 0, h: 0, },
            },
            c2rust_unnamed_3: C2RustUnnamed_6 {
                c2rust_unnamed: C2RustUnnamed_7 { f_: 0, a_: 0, },
            },
            c2rust_unnamed_4: C2RustUnnamed_4 {
                c2rust_unnamed: C2RustUnnamed_5 { c_: 0, b_: 0, },
            },
            c2rust_unnamed_5: C2RustUnnamed_2 {
                c2rust_unnamed: C2RustUnnamed_3 { e_: 0, d_: 0, },
            },
            c2rust_unnamed_6: C2RustUnnamed_0 {
                c2rust_unnamed: C2RustUnnamed_1 { l_: 0, h_: 0, },
            },
            i: 0,
            r: 0,
            iff_delay: 0,
            interrupt_mode: 0,
            irq_data: 0,
            irq_pending: 0,
            nmi_pending: 0,
            halted: false,
            iff1: false,
            iff2: false,
            f_szpxy: [
                0x44, 0, 0, 0x4, 0, 0x4, 0x4, 0, 0x8, 0xc, 0xc, 0x8, 0xc, 0x8, 0x8, 0xc, 0, 0x4, 0x4, 0, 0x4, 0, 0, 0x4, 0xc, 0x8, 0x8, 0xc, 0x8, 0xc, 0xc, 0x8, 0x20, 0x24, 0x24, 0x20, 0x24, 0x20, 0x20, 0x24, 0x2c, 0x28, 0x28, 0x2c, 0x28, 0x2c, 0x2c, 0x28, 0x24, 0x20, 0x20, 0x24, 0x20, 0x24, 0x24, 0x20, 0x28, 0x2c, 0x2c, 0x28, 0x2c, 0x28, 0x28, 0x2c, 0, 0x4, 0x4, 0, 0x4, 0, 0, 0x4, 0xc, 0x8, 0x8, 0xc, 0x8, 0xc, 0xc, 0x8, 0x4, 0, 0, 0x4, 0, 0x4, 0x4, 0, 0x8, 0xc, 0xc, 0x8, 0xc, 0x8, 0x8, 0xc, 0x24, 0x20, 0x20, 0x24, 0x20, 0x24, 0x24, 0x20, 0x28, 0x2c, 0x2c, 0x28, 0x2c, 0x28, 0x28, 0x2c, 0x20, 0x24, 0x24, 0x20, 0x24, 0x20, 0x20, 0x24, 0x2c, 0x28, 0x28, 0x2c, 0x28, 0x2c, 0x2c, 0x28, 0x80, 0x84, 0x84, 0x80, 0x84, 0x80, 0x80, 0x84, 0x8c, 0x88, 0x88, 0x8c, 0x88, 0x8c, 0x8c, 0x88, 0x84, 0x80, 0x80, 0x84, 0x80, 0x84, 0x84, 0x80, 0x88, 0x8c, 0x8c, 0x88, 0x8c, 0x88, 0x88, 0x8c, 0xa4, 0xa0, 0xa0, 0xa4, 0xa0, 0xa4, 0xa4, 0xa0, 0xa8, 0xac, 0xac, 0xa8, 0xac, 0xa8, 0xa8, 0xac, 0xa0, 0xa4, 0xa4, 0xa0, 0xa4, 0xa0, 0xa0, 0xa4, 0xac, 0xa8, 0xa8, 0xac, 0xa8, 0xac, 0xac, 0xa8, 0x84, 0x80, 0x80, 0x84, 0x80, 0x84, 0x84, 0x80, 0x88, 0x8c, 0x8c, 0x88, 0x8c, 0x88, 0x88, 0x8c, 0x80, 0x84, 0x84, 0x80, 0x84, 0x80, 0x80, 0x84, 0x8c, 0x88, 0x88, 0x8c, 0x88, 0x8c, 0x8c, 0x88, 0xa0, 0xa4, 0xa4, 0xa0, 0xa4, 0xa0, 0xa0, 0xa4, 0xac, 0xa8, 0xa8, 0xac, 0xa8, 0xac, 0xac, 0xa8, 0xa4, 0xa0, 0xa0, 0xa4, 0xa0, 0xa4, 0xa4, 0xa0, 0xa8, 0xac, 0xac, 0xa8, 0xac, 0xa8, 0xa8, 0xac,
            ],
            #[cfg(test)]
            test_finished: false,
        }
    }
    pub fn init(&mut self) {
        self.pc = 0 as i32 as uint16_t;
        self.sp = 0xffff as i32 as uint16_t;
        self.ix = 0 as i32 as uint16_t;
        self.iy = 0 as i32 as uint16_t;
        self.mem_ptr = 0 as i32 as uint16_t;
        self.c2rust_unnamed.af = 0xffff as i32 as uint16_t;
        self.c2rust_unnamed_0.bc = 0 as i32 as uint16_t;
        self.c2rust_unnamed_1.de = 0 as i32 as uint16_t;
        self.c2rust_unnamed_2.hl = 0 as i32 as uint16_t;
        self.c2rust_unnamed_3.a_f_ = 0 as i32 as uint16_t;
        self.c2rust_unnamed_4.b_c_ = 0 as i32 as uint16_t;
        self.c2rust_unnamed_5.d_e_ = 0 as i32 as uint16_t;
        self.c2rust_unnamed_6.h_l_ = 0 as i32 as uint16_t;
        self.i = 0 as i32 as uint8_t;
        self.r = 0 as i32 as uint8_t;
        self.iff_delay = 0 as i32 as uint8_t;
        self.interrupt_mode = 0 as i32 as uint8_t;
        self.iff1 = 0 as i32 != 0;
        self.iff2 = 0 as i32 != 0;
        self.halted = 0 as i32 != 0;
        self.irq_pending = 0 as i32 as uint8_t;
        self.nmi_pending = 0 as i32 as uint8_t;
        self.irq_data = 0 as i32 as uint8_t;
        #[cfg(test)]
        {
            self.test_finished = false;
        }
    }
    pub fn step(&mut self) -> u32 {
        self.z80_step_s()
    }
    fn internal_port_in(&self, addr: u16) -> u8 {
        #[cfg(test)]
        {
            let mut operation = self.c2rust_unnamed_0.c2rust_unnamed.c;
            if operation == 2{
                print!( "{}", self.c2rust_unnamed_1.c2rust_unnamed.e);
            } else if operation == 9 {
                let mut addr = ((self.c2rust_unnamed_1.c2rust_unnamed.d as i32)
                    << 8 as i32 | self.c2rust_unnamed_1.c2rust_unnamed.e as i32)
                    as u16;
                loop {
                    let fresh0 = addr;
                    addr = addr.wrapping_add(1);
                    print!("{}", String::from_utf8(vec![self.ctrl.read_byte(fresh0)]).unwrap());
                    if !(self.ctrl.read_byte(addr) as i32 != '$' as i32 as i32) {
                        break;
                    }
                }
            }

        }
        self.ctrl.port_in(addr)
    }
    fn internal_port_out(&mut self, addr: u16, value: u8) {
        #[cfg(test)]
        {
            self.test_finished = true;
        }
        self.ctrl.port_out(addr, value);
    }

    fn flag_val(mut bit: z80_flagbit, mut cond: bool) -> uint8_t {
        return ((cond as i32) << bit as u32) as uint8_t;
    }
    fn flag_get(&mut self, mut bit: z80_flagbit) -> bool {
        return self.c2rust_unnamed.c2rust_unnamed.f as i32
            & (1 as i32) << bit as u32 != 0;
    }
    fn flag_set(&mut self, mut bit: z80_flagbit, mut val: bool) {
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.c2rust_unnamed.c2rust_unnamed.f as i32
            & !((1 as i32) << bit as u32)) as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.c2rust_unnamed.c2rust_unnamed.f as i32
            | (val as i32) << bit as u32) as uint8_t;
    }
    fn rw(&mut self, mut addr: uint16_t) -> uint16_t {
        return (
            (self.ctrl.read_byte((addr as i32 + 1 as i32) as uint16_t) as i32) << 8 as i32
            | self.ctrl.read_byte(addr) as i32
        ) as uint16_t;
    }
    fn ww(&mut self, mut addr: uint16_t, mut val: uint16_t) {
        self.ctrl.write_byte(addr, (val as i32 & 0xff as i32) as uint8_t);
        self.ctrl.write_byte(
            (addr as i32 + 1 as i32) as uint16_t,
            (val as i32 >> 8 as i32) as uint8_t,
        );
    }
    fn pushw(&mut self, mut val: uint16_t) {
        self.sp = (self.sp as i32 - 2 as i32) as uint16_t;
        self.ww(self.sp, val);
    }
    fn popw(&mut self) -> uint16_t {
        self.sp = (self.sp as i32 + 2 as i32) as uint16_t;
        return self.rw((self.sp as i32 - 2 as i32) as uint16_t);
    }
    fn nextb(&mut self) -> uint8_t {
        let fresh0 = self.pc;
        self.pc = (self.pc).wrapping_add(1);
        return self.ctrl.read_byte(fresh0);
    }
    fn nextw(&mut self) -> uint16_t {
        self.pc = (self.pc as i32 + 2 as i32) as uint16_t;
        return self.rw((self.pc as i32 - 2 as i32) as uint16_t);
    }
    fn inc_r(&mut self) {
        self
            .r = (self.r as i32 & 0x80 as i32
            | self.r as i32 + 1 as i32 & 0x7f as i32) as uint8_t;
    }
    fn parity(mut v: uint8_t) -> bool {
        v = (v as i32 ^ v as i32 >> 4 as i32) as uint8_t;
        v = (v as i32 & 0xf as i32) as uint8_t;
        return 0x6996 as i32 >> v as i32 & 1 as i32 == 0;
    }
    fn jump(&mut self, mut addr: uint16_t) {
        self.pc = addr;
        self.mem_ptr = addr;
    }
    fn cond_jump(&mut self, mut condition: bool) {
        let addr: uint16_t = self.nextw();
        if condition {
            self.jump(addr);
        }
        self.mem_ptr = addr;
    }
    fn call(&mut self, mut addr: uint16_t) {
        self.pushw(self.pc);
        self.pc = addr;
        self.mem_ptr = addr;
    }
    fn cond_call(&mut self, mut condition: bool) -> u32 {
        let addr: uint16_t = self.nextw();
        let mut cyc: u32 = 0;
        if condition {
            self.call(addr);
            cyc = 7;
        }
        self.mem_ptr = addr;
        return cyc;
    }
    fn ret(&mut self) {
        self.pc = self.popw();
        self.mem_ptr = self.pc;
    }
    fn cond_ret(&mut self, mut condition: bool) -> u32 {
        if condition {
            self.ret();
            return 6;
        }
        return 0;
    }
    fn jr(&mut self, mut displacement: int8_t) {
        self.pc = (self.pc as i32 + displacement as i32) as uint16_t;
        self.mem_ptr = self.pc;
    }
    fn cond_jr(&mut self, mut condition: bool) -> u32 {
        let b: int8_t = self.nextb() as int8_t;
        if condition {
            self.jr(b);
            return 5;
        }
        return 0;
    }
    fn addb(
        &mut self,
        mut a: uint32_t,
        mut b: uint32_t,
        mut cy: bool,
    ) -> uint8_t {
        let mut result: int32_t = a.wrapping_add(b).wrapping_add(cy as u32)
            as int32_t;
        let mut carry: int32_t = (result as u32 ^ a ^ b) as int32_t;
        result &= 0xff as i32;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.f_szpxy[result as usize] as i32
            & !((1 as i32) << pf as i32)) as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.c2rust_unnamed.c2rust_unnamed.f as i32
            | carry & (1 as i32) << hf as i32) as uint8_t;
        carry >>= 6 as i32;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.c2rust_unnamed.c2rust_unnamed.f as i32
            | carry + 2 as i32 & 4 as i32) as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.c2rust_unnamed.c2rust_unnamed.f as i32
            | carry >> 2 as i32) as uint8_t;
        return result as uint8_t;
    }
    fn subb(
        &mut self,
        mut a: uint32_t,
        mut b: uint32_t,
        mut cy: bool,
    ) -> uint8_t {
        let mut result: int32_t = a.wrapping_sub(b).wrapping_sub(cy as u32)
            as int32_t;
        let mut carry: int32_t = (result as u32 ^ a ^ b) as int32_t;
        result &= 0xff as i32;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = ((1 as i32) << nf as i32
            | self.f_szpxy[result as usize] as i32
                & !((1 as i32) << pf as i32)) as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.c2rust_unnamed.c2rust_unnamed.f as i32
            | carry & (1 as i32) << hf as i32) as uint8_t;
        carry >>= 6 as i32;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.c2rust_unnamed.c2rust_unnamed.f as i32
            | carry + 2 as i32 & 4 as i32) as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.c2rust_unnamed.c2rust_unnamed.f as i32
            | carry >> 2 as i32 & 1 as i32) as uint8_t;
        return result as uint8_t;
    }
    fn addw(
        &mut self,
        mut a: uint16_t,
        mut b: uint16_t,
        mut cy: bool,
    ) -> uint16_t {
        let mut lsb: uint8_t = self.addb(a as uint32_t, b as uint32_t, cy);
        let mut msb: uint8_t = self.addb(
            (a as i32 >> 8 as i32) as uint32_t,
            (b as i32 >> 8 as i32) as uint32_t,
            self.flag_get(cf),
        );
        let mut result: uint16_t = ((msb as i32) << 8 as i32
            | lsb as i32) as uint16_t;
        self.flag_set(zf, result as i32 == 0 as i32);
        self.mem_ptr = (a as i32 + 1 as i32) as uint16_t;
        return result;
    }
    fn subw(
        &mut self,
        mut a: uint16_t,
        mut b: uint16_t,
        mut cy: bool,
    ) -> uint16_t {
        let mut lsb: uint8_t = self.subb(a as uint32_t, b as uint32_t, cy);
        let mut msb: uint8_t = self.subb(
            (a as i32 >> 8 as i32) as uint32_t,
            (b as i32 >> 8 as i32) as uint32_t,
            self.flag_get(cf),
        );
        let mut result: uint16_t = ((msb as i32) << 8 as i32
            | lsb as i32) as uint16_t;
        self.flag_set(zf, result as i32 == 0 as i32);
        self.mem_ptr = (a as i32 + 1 as i32) as uint16_t;
        return result;
    }
    fn addhl(&mut self, mut val: uint16_t) {
        let mut sfc: bool = self.flag_get(sf);
        let mut zfc: bool = self.flag_get(zf);
        let mut pfc: bool = self.flag_get(pf);
        let mut result: uint16_t = self.addw(
            self.c2rust_unnamed_2.hl,
            val,
            0 as i32 != 0,
        );
        self.c2rust_unnamed_2.hl = result;
        self.flag_set(sf, sfc);
        self.flag_set(zf, zfc);
        self.flag_set(pf, pfc);
    }
    fn addiz(&mut self, mut reg: *mut uint16_t, mut val: uint16_t) {
        let mut sfc: bool = self.flag_get(sf);
        let mut zfc: bool = self.flag_get(zf);
        let mut pfc: bool = self.flag_get(pf);
        let mut result: uint16_t = self.addw(*reg, val, 0 as i32 != 0);
        *reg = result;
        self.flag_set(sf, sfc);
        self.flag_set(zf, zfc);
        self.flag_set(pf, pfc);
    }
    fn adchl(&mut self, mut val: uint16_t) {
        let mut result: uint16_t = self.addw(self.c2rust_unnamed_2.hl, val, self.flag_get(cf));
        self.flag_set(sf, result as i32 >> 15 as i32 != 0);
        self.flag_set(zf, result as i32 == 0 as i32);
        self.c2rust_unnamed_2.hl = result;
    }
    fn sbchl(&mut self, mut val: uint16_t) {
        let result: uint16_t = self.subw(self.c2rust_unnamed_2.hl, val, self.flag_get(cf));
        self.flag_set(sf, result as i32 >> 15 as i32 != 0);
        self.flag_set(zf, result as i32 == 0 as i32);
        self.c2rust_unnamed_2.hl = result;
    }
    fn inc(&mut self, mut a: uint8_t) -> uint8_t {
        let mut cfc: bool = self.flag_get(cf);
        let mut result: uint8_t = self.addb(
            a as uint32_t,
            1 as i32 as uint32_t,
            0 as i32 != 0,
        );
        self.flag_set(cf, cfc);
        return result;
    }
    fn dec(&mut self, mut a: uint8_t) -> uint8_t {
        let mut cfc: bool = self.flag_get(cf);
        let mut result: uint8_t = self.subb(
            a as uint32_t,
            1 as i32 as uint32_t,
            0 as i32 != 0,
        );
        self.flag_set(cf, cfc);
        return result;
    }
    fn land(&mut self, mut val: uint8_t) {
        let result: uint8_t = (self.c2rust_unnamed.c2rust_unnamed.a as i32
            & val as i32) as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.f_szpxy[result as usize] as i32
            | Self::flag_val(hf, 1 as i32 != 0) as i32
            | Self::flag_val(nf, 0 as i32 != 0) as i32
            | Self::flag_val(cf, 0 as i32 != 0) as i32) as uint8_t;
        self.c2rust_unnamed.c2rust_unnamed.a = result;
    }
    fn lxor(&mut self, val: uint8_t) {
        let result: uint8_t = (self.c2rust_unnamed.c2rust_unnamed.a as i32
            ^ val as i32) as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.f_szpxy[result as usize] as i32
            | Self::flag_val(hf, 0 as i32 != 0) as i32
            | Self::flag_val(nf, 0 as i32 != 0) as i32
            | Self::flag_val(cf, 0 as i32 != 0) as i32) as uint8_t;
        self.c2rust_unnamed.c2rust_unnamed.a = result;
    }
    fn lor(&mut self, val: uint8_t) {
        let result: uint8_t = (self.c2rust_unnamed.c2rust_unnamed.a as i32
            | val as i32) as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.f_szpxy[result as usize] as i32
            | Self::flag_val(hf, 0 as i32 != 0) as i32
            | Self::flag_val(nf, 0 as i32 != 0) as i32
            | Self::flag_val(cf, 0 as i32 != 0) as i32) as uint8_t;
        self.c2rust_unnamed.c2rust_unnamed.a = result;
    }
    fn cp(&mut self, val: uint32_t) {
        let mut result: int32_t = (self.c2rust_unnamed.c2rust_unnamed.a as u32)
            .wrapping_sub(val) as int32_t;
        let mut carry: int32_t = ((result
            ^ self.c2rust_unnamed.c2rust_unnamed.a as i32) as u32 ^ val)
            as int32_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (((1 as i32) << nf as i32) as u32
            | val
                & ((1 as i32) << xf as i32
                    | (1 as i32) << yf as i32) as u32
            | (result & (1 as i32) << sf as i32) as u32
            | (((result & 0xff as i32 == 0) as i32) << zf as i32)
                as u32) as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.c2rust_unnamed.c2rust_unnamed.f as i32
            | carry & (1 as i32) << hf as i32) as uint8_t;
        carry >>= 6 as i32;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.c2rust_unnamed.c2rust_unnamed.f as i32
            | carry + 2 as i32 & 4 as i32) as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.c2rust_unnamed.c2rust_unnamed.f as i32
            | carry >> 2 as i32 & 1 as i32) as uint8_t;
    }
    fn cb_rlc(&mut self, mut val: uint8_t) -> uint8_t {
        let old: bool = val as i32 >> 7 as i32 != 0;
        val = ((val as i32) << 1 as i32 | old as i32) as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.f_szpxy[val as usize] as i32
            | Self::flag_val(nf, 0 as i32 != 0) as i32
            | Self::flag_val(hf, 0 as i32 != 0) as i32
            | Self::flag_val(cf, old) as i32) as uint8_t;
        return val;
    }
    fn cb_rrc(&mut self, mut val: uint8_t) -> uint8_t {
        let old: bool = val as i32 & 1 as i32 != 0;
        val = (val as i32 >> 1 as i32
            | (old as i32) << 7 as i32) as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.f_szpxy[val as usize] as i32
            | Self::flag_val(nf, 0 as i32 != 0) as i32
            | Self::flag_val(hf, 0 as i32 != 0) as i32
            | Self::flag_val(cf, old) as i32) as uint8_t;
        return val;
    }
    fn cb_rl(&mut self, mut val: uint8_t) -> uint8_t {
        let cfc: bool = self.flag_get(cf);
        let cfn: bool = val as i32 >> 7 as i32 != 0;
        val = ((val as i32) << 1 as i32 | cfc as i32) as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.f_szpxy[val as usize] as i32 | Self::flag_val(cf, cfn) as i32
            | Self::flag_val(nf, 0 as i32 != 0) as i32
            | Self::flag_val(hf, 0 as i32 != 0) as i32) as uint8_t;
        return val;
    }
    fn cb_rr(&mut self, mut val: uint8_t) -> uint8_t {
        let c: bool = self.flag_get(cf);
        let cfn: bool = val as i32 & 1 as i32 != 0;
        val = (val as i32 >> 1 as i32
            | (c as i32) << 7 as i32) as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.f_szpxy[val as usize] as i32 | Self::flag_val(cf, cfn) as i32
            | Self::flag_val(nf, 0 as i32 != 0) as i32
            | Self::flag_val(hf, 0 as i32 != 0) as i32) as uint8_t;
        return val;
    }
    fn cb_sla(&mut self, mut val: uint8_t) -> uint8_t {
        let cfn: bool = val as i32 >> 7 as i32 != 0;
        val = ((val as i32) << 1 as i32) as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.f_szpxy[val as usize] as i32 | Self::flag_val(cf, cfn) as i32
            | Self::flag_val(nf, 0 as i32 != 0) as i32
            | Self::flag_val(hf, 0 as i32 != 0) as i32) as uint8_t;
        return val;
    }
    fn cb_sll(&mut self, mut val: uint8_t) -> uint8_t {
        let cfn: bool = val as i32 >> 7 as i32 != 0;
        val = ((val as i32) << 1 as i32) as uint8_t;
        val = (val as i32 | 1 as i32) as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.f_szpxy[val as usize] as i32 | Self::flag_val(cf, cfn) as i32
            | Self::flag_val(nf, 0 as i32 != 0) as i32
            | Self::flag_val(hf, 0 as i32 != 0) as i32) as uint8_t;
        return val;
    }
    fn cb_sra(&mut self, mut val: uint8_t) -> uint8_t {
        let cfn: bool = val as i32 & 1 as i32 != 0;
        val = (val as i32 >> 1 as i32
            | val as i32 & 0x80 as i32) as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.f_szpxy[val as usize] as i32 | Self::flag_val(cf, cfn) as i32
            | Self::flag_val(nf, 0 as i32 != 0) as i32
            | Self::flag_val(hf, 0 as i32 != 0) as i32) as uint8_t;
        return val;
    }
    fn cb_srl(&mut self, mut val: uint8_t) -> uint8_t {
        let cfn: bool = val as i32 & 1 as i32 != 0;
        val = (val as i32 >> 1 as i32) as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.f_szpxy[val as usize] as i32 | Self::flag_val(cf, cfn) as i32
            | Self::flag_val(nf, 0 as i32 != 0) as i32
            | Self::flag_val(hf, 0 as i32 != 0) as i32) as uint8_t;
        return val;
    }
    fn cb_bit(&mut self, mut val: uint8_t, mut n: uint8_t) -> uint8_t {
        let result: uint8_t = (val as i32 & (1 as i32) << n as i32)
            as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.f_szpxy[result as usize] as i32
            | Self::flag_val(cf, self.flag_get(cf)) as i32
            | Self::flag_val(hf, 1 as i32 != 0) as i32
            | Self::flag_val(nf, 0 as i32 != 0) as i32) as uint8_t;
        return result;
    }
    fn ldi(&mut self) {
        let de: uint16_t = self.c2rust_unnamed_1.de;
        let hl: uint16_t = self.c2rust_unnamed_2.hl;
        let val: uint8_t = self.ctrl.read_byte(hl);
        self.ctrl.write_byte(de, val);
        self.c2rust_unnamed_2.hl = (self.c2rust_unnamed_2.hl).wrapping_add(1);
        self.c2rust_unnamed_1.de = (self.c2rust_unnamed_1.de).wrapping_add(1);
        self.c2rust_unnamed_0.bc = (self.c2rust_unnamed_0.bc).wrapping_sub(1);
        let result: uint8_t = (val as i32
            + self.c2rust_unnamed.c2rust_unnamed.a as i32) as uint8_t;
        self.flag_set(xf, result as i32 >> 3 as i32 & 1 as i32 != 0);
        self.flag_set(yf, result as i32 >> 1 as i32 & 1 as i32 != 0);
        self.flag_set(nf, 0 as i32 != 0);
        self.flag_set(hf, 0 as i32 != 0);
        self.flag_set(pf, self.c2rust_unnamed_0.bc as i32 > 0 as i32);
    }
    fn ldd(&mut self) {
        self.ldi();
        self
            .c2rust_unnamed_2
            .hl = (self.c2rust_unnamed_2.hl as i32 - 2 as i32) as uint16_t;
        self
            .c2rust_unnamed_1
            .de = (self.c2rust_unnamed_1.de as i32 - 2 as i32) as uint16_t;
    }
    fn cpi(&mut self) {
        let mut cfc: bool = self.flag_get(cf);
        let result: uint8_t = self.subb(
            self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
            self.ctrl.read_byte(self.c2rust_unnamed_2.hl) as uint32_t,
            0 as i32 != 0,
        );
        self.c2rust_unnamed_2.hl = (self.c2rust_unnamed_2.hl).wrapping_add(1);
        self.c2rust_unnamed_0.bc = (self.c2rust_unnamed_0.bc).wrapping_sub(1);
        let mut hfc: bool = self.flag_get(hf);
        self.flag_set(
            xf,
            result as i32 - hfc as i32 >> 3 as i32 & 1 as i32
                != 0,
        );
        self.flag_set(
            yf,
            result as i32 - hfc as i32 >> 1 as i32 & 1 as i32
                != 0,
        );
        self.flag_set(pf, self.c2rust_unnamed_0.bc as i32 != 0 as i32);
        self.flag_set(cf, cfc);
        self.mem_ptr = (self.mem_ptr as i32 + 1 as i32) as uint16_t;
    }
    fn cpd(&mut self) {
        self.cpi();
        self
            .c2rust_unnamed_2
            .hl = (self.c2rust_unnamed_2.hl as i32 - 2 as i32) as uint16_t;
        self.mem_ptr = (self.mem_ptr as i32 - 2 as i32) as uint16_t;
    }
    fn in_r_c(&mut self, mut r: *mut uint8_t) {
        *r = self.internal_port_in(self.c2rust_unnamed_0.bc);
        self.flag_set(zf, *r as i32 == 0 as i32);
        self.flag_set(sf, *r as i32 >> 7 as i32 != 0);
        self.flag_set(pf, Self::parity(*r));
        self.flag_set(nf, 0 as i32 != 0);
        self.flag_set(hf, 0 as i32 != 0);
    }
    fn ini(&mut self) {
        let mut tmp: u32 = self.internal_port_in(self.c2rust_unnamed_0.bc)
            as u32;
        let mut tmp2: u32 = tmp
            .wrapping_add(
                (self.c2rust_unnamed_0.c2rust_unnamed.c as i32 + 1 as i32
                    & 0xff as i32) as u32,
            );
        self
            .mem_ptr = (self.c2rust_unnamed_0.bc as i32 + 1 as i32)
            as uint16_t;
        self.ctrl.write_byte(self.c2rust_unnamed_2.hl, tmp as uint8_t);
        self.c2rust_unnamed_2.hl = (self.c2rust_unnamed_2.hl).wrapping_add(1);
        self
            .c2rust_unnamed_0
            .c2rust_unnamed
            .b = (self.c2rust_unnamed_0.c2rust_unnamed.b).wrapping_sub(1);
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.f_szpxy[self.c2rust_unnamed_0.c2rust_unnamed.b as usize] as i32
            & !((1 as i32) << pf as i32)
            | Self::flag_val(nf, tmp >> 7 as i32 & 1 != 0)
                as i32
            | Self::flag_val(
                pf,
                Self::parity(
                    (tmp2 & 7
                        ^ self.c2rust_unnamed_0.c2rust_unnamed.b as u32) as uint8_t,
                ),
            ) as i32
            | Self::flag_val(hf, tmp2 > 255) as i32
            | Self::flag_val(cf, tmp2 > 255) as i32)
            as uint8_t;
    }
    fn ind(&mut self) {
        let mut tmp: u32 = self.internal_port_in(self.c2rust_unnamed_0.bc)
            as u32;
        let mut tmp2: u32 = tmp
            .wrapping_add(
                (self.c2rust_unnamed_0.c2rust_unnamed.c as i32 - 1 as i32
                    & 0xff as i32) as u32,
            );
        self
            .mem_ptr = (self.c2rust_unnamed_0.bc as i32 - 1 as i32)
            as uint16_t;
        self.ctrl.write_byte(self.c2rust_unnamed_2.hl, tmp as uint8_t);
        self.c2rust_unnamed_2.hl = (self.c2rust_unnamed_2.hl).wrapping_sub(1);
        self
            .c2rust_unnamed_0
            .c2rust_unnamed
            .b = (self.c2rust_unnamed_0.c2rust_unnamed.b).wrapping_sub(1);
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.f_szpxy[self.c2rust_unnamed_0.c2rust_unnamed.b as usize] as i32
            & !((1 as i32) << pf as i32)
            | Self::flag_val(nf, tmp >> 7 as i32 & 1 != 0)
                as i32
            | Self::flag_val(
                pf,
                Self::parity(
                    (tmp2 & 7
                        ^ self.c2rust_unnamed_0.c2rust_unnamed.b as u32) as uint8_t,
                ),
            ) as i32
            | Self::flag_val(hf, tmp2 > 255) as i32
            | Self::flag_val(cf, tmp2 > 255) as i32)
            as uint8_t;
    }
    fn outi(&mut self) {
        let mut tmp: u32 = self.ctrl.read_byte(self.c2rust_unnamed_2.hl) as u32;
        let mut tmp2: u32 = 0;
        self.internal_port_out(self.c2rust_unnamed_0.bc, tmp as uint8_t);
        self.c2rust_unnamed_2.hl = (self.c2rust_unnamed_2.hl).wrapping_add(1);
        self
            .c2rust_unnamed_0
            .c2rust_unnamed
            .b = (self.c2rust_unnamed_0.c2rust_unnamed.b as i32 - 1 as i32)
            as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = self.f_szpxy[self.c2rust_unnamed_0.c2rust_unnamed.b as usize];
        self.flag_set(nf, tmp >> 7 as i32 & 1 != 0);
        tmp2 = tmp.wrapping_add(self.c2rust_unnamed_2.c2rust_unnamed.l as u32);
        self.flag_set(
            pf,
            Self::parity(
                (tmp2 & 7
                    ^ self.c2rust_unnamed_0.c2rust_unnamed.b as u32) as uint8_t,
            ),
        );
        self.flag_set(hf, tmp2 > 255);
        self.flag_set(cf, tmp2 > 255);
        self
            .mem_ptr = (self.c2rust_unnamed_0.bc as i32 + 1 as i32)
            as uint16_t;
    }
    fn outd(&mut self) {
        self.outi();
        self
            .c2rust_unnamed_2
            .hl = (self.c2rust_unnamed_2.hl as i32 - 2 as i32) as uint16_t;
        self
            .mem_ptr = (self.c2rust_unnamed_0.bc as i32 - 2 as i32)
            as uint16_t;
    }
    fn outc(&mut self, mut data: uint8_t) {
        self.internal_port_out(self.c2rust_unnamed_0.bc, data);
        self
            .mem_ptr = (self.c2rust_unnamed_0.bc as i32 + 1 as i32)
            as uint16_t;
    }
    fn daa(&mut self) {
        let mut correction: uint8_t = 0 as i32 as uint8_t;
        if self.c2rust_unnamed.c2rust_unnamed.a as i32 & 0xf as i32
            > 0x9 as i32 || self.flag_get(hf) as i32 != 0
        {
            correction = (correction as i32 + 0x6 as i32) as uint8_t;
        }
        if self.c2rust_unnamed.c2rust_unnamed.a as i32 > 0x99 as i32
            || self.flag_get(cf) as i32 != 0
        {
            correction = (correction as i32 + 0x60 as i32) as uint8_t;
            self.flag_set(cf, 1 as i32 != 0);
        }
        let substraction: bool = self.flag_get(nf);
        if substraction {
            self.flag_set(
                hf,
                self.flag_get(hf) as i32 != 0
                    && (self.c2rust_unnamed.c2rust_unnamed.a as i32
                        & 0xf as i32) < 0x6 as i32,
            );
            self
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (self.c2rust_unnamed.c2rust_unnamed.a as i32
                - correction as i32) as uint8_t;
        } else {
            self.flag_set(
                hf,
                self.c2rust_unnamed.c2rust_unnamed.a as i32 & 0xf as i32
                    > 0x9 as i32,
            );
            self
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (self.c2rust_unnamed.c2rust_unnamed.a as i32
                + correction as i32) as uint8_t;
        }
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.c2rust_unnamed.c2rust_unnamed.f as i32
            & !((1 as i32) << sf as i32
                | (1 as i32) << zf as i32
                | (1 as i32) << pf as i32
                | (1 as i32) << xf as i32
                | (1 as i32) << yf as i32)) as uint8_t;
        self
            .c2rust_unnamed
            .c2rust_unnamed
            .f = (self.c2rust_unnamed.c2rust_unnamed.f as i32
            | self.f_szpxy[self.c2rust_unnamed.c2rust_unnamed.a as usize] as i32)
            as uint8_t;
    }
    fn displace(
        &mut self,
        mut base_addr: uint16_t,
        mut displacement: int8_t,
    ) -> uint16_t {
        let addr: uint16_t = (base_addr as i32 + displacement as i32)
            as uint16_t;
        self.mem_ptr = addr;
        return addr;
    }
    fn process_interrupts(&mut self) -> u32 {
        let mut cyc: u32 = 0;
        if self.iff_delay as i32 > 0 as i32 {
            self.iff_delay = (self.iff_delay as i32 - 1 as i32) as uint8_t;
            if self.iff_delay as i32 == 0 as i32 {
                self.iff1 = 1 as i32 != 0;
                self.iff2 = 1 as i32 != 0;
            }
            return cyc;
        }
        if self.nmi_pending != 0 {
            self
                .nmi_pending = (self.nmi_pending as i32
                & !(Z80_PULSE as i32)) as uint8_t;
            self.halted = 0 as i32 != 0;
            self.iff1 = 0 as i32 != 0;
            self.inc_r();
            cyc = cyc.wrapping_add(11);
            self.call(0x66 as i32 as uint16_t);
            return cyc;
        }
        if self.irq_pending as i32 != 0 && self.iff1 as i32 != 0 {
            self
                .irq_pending = (self.irq_pending as i32
                & !(Z80_PULSE as i32)) as uint8_t;
            self.halted = 0 as i32 != 0;
            self.iff1 = 0 as i32 != 0;
            self.iff2 = 0 as i32 != 0;
            self.inc_r();
            match self.interrupt_mode as i32 {
                0 => {
                    cyc = cyc.wrapping_add(11);
                    cyc = cyc.wrapping_add(self.exec_opcode(self.irq_data));
                }
                1 => {
                    cyc = cyc.wrapping_add(13);
                    self.call(0x38 as i32 as uint16_t);
                }
                2 => {
                    cyc = cyc.wrapping_add(19);
                    self.call(
                        self.rw(
                            ((self.i as i32) << 8 as i32
                                | self.irq_data as i32) as uint16_t,
                        ),
                    );
                }
                _ => {}
            }
            return cyc;
        }
        return cyc;
    }
    pub fn z80_reset(&mut self) {
        self.pc = 0 as i32 as uint16_t;
        self.mem_ptr = 0 as i32 as uint16_t;
        self.i = 0 as i32 as uint8_t;
        self.r = 0 as i32 as uint8_t;
        self.interrupt_mode = 0 as i32 as uint8_t;
        self.iff_delay = 0 as i32 as uint8_t;
        self.iff1 = 0 as i32 != 0;
        self.iff2 = 0 as i32 != 0;
        self.halted = 0 as i32 != 0;
        self.nmi_pending = 0 as i32 as uint8_t;
    }
    fn z80_step_s(&mut self) -> u32 {
        let mut cyc: u32 = 0;
        if self.halted {
            cyc = cyc.wrapping_add(self.exec_opcode(0 as i32 as uint8_t));
        } else {
            let opcode: uint8_t = self.nextb();
            cyc = cyc.wrapping_add(self.exec_opcode(opcode));
        }
        cyc = cyc.wrapping_add(self.process_interrupts());
        return cyc;
    }
    pub fn z80_set_pc(&mut self, mut pc: uint16_t) {
        self.pc = pc;
    }
    pub fn z80_set_sp(&mut self, mut sp: uint16_t) {
        self.sp = sp;
    }
    pub fn z80_step_n(
        &mut self,
        mut cycles: u32,
    ) -> u32 {
        let mut cyc: u32 = 0;
        while cyc < cycles {
            cyc = cyc.wrapping_add(self.z80_step_s());
        }
        return cyc;
    }
    pub fn z80_assert_nmi(&mut self) {
        self
            .nmi_pending = (self.nmi_pending as i32 | Z80_ASSERT as i32)
            as uint8_t;
    }
    pub fn z80_pulse_nmi(&mut self) {
        self
            .nmi_pending = (self.nmi_pending as i32 | Z80_PULSE as i32)
            as uint8_t;
    }
    pub fn z80_clr_nmi(&mut self) {
        self.nmi_pending = 0 as i32 as uint8_t;
    }
    pub fn z80_assert_irq(&mut self, mut data: uint8_t) {
        self
            .irq_pending = (self.irq_pending as i32 | Z80_ASSERT as i32)
            as uint8_t;
        self.irq_data = data;
    }
    pub fn z80_pulse_irq(&mut self, mut data: uint8_t) {
        self
            .irq_pending = (self.irq_pending as i32 | Z80_PULSE as i32)
            as uint8_t;
        self.irq_data = data;
    }
    pub fn z80_clr_irq(&mut self) {
        self.irq_pending = 0 as i32 as uint8_t;
    }
    fn exec_opcode(&mut self, mut opcode: uint8_t) -> u32 {
        let mut cyc: u32 = 0;
        self.inc_r();
        match opcode as i32 {
            127 => {
                cyc = cyc.wrapping_add(4);
                self.c2rust_unnamed.c2rust_unnamed.a = self.c2rust_unnamed.c2rust_unnamed.a;
            }
            120 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.c2rust_unnamed_0.c2rust_unnamed.b;
            }
            121 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.c2rust_unnamed_0.c2rust_unnamed.c;
            }
            122 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.c2rust_unnamed_1.c2rust_unnamed.d;
            }
            123 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.c2rust_unnamed_1.c2rust_unnamed.e;
            }
            124 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.c2rust_unnamed_2.c2rust_unnamed.h;
            }
            125 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.c2rust_unnamed_2.c2rust_unnamed.l;
            }
            71 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .b = self.c2rust_unnamed.c2rust_unnamed.a;
            }
            64 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .b = self.c2rust_unnamed_0.c2rust_unnamed.b;
            }
            65 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .b = self.c2rust_unnamed_0.c2rust_unnamed.c;
            }
            66 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .b = self.c2rust_unnamed_1.c2rust_unnamed.d;
            }
            67 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .b = self.c2rust_unnamed_1.c2rust_unnamed.e;
            }
            68 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .b = self.c2rust_unnamed_2.c2rust_unnamed.h;
            }
            69 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .b = self.c2rust_unnamed_2.c2rust_unnamed.l;
            }
            79 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c = self.c2rust_unnamed.c2rust_unnamed.a;
            }
            72 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c = self.c2rust_unnamed_0.c2rust_unnamed.b;
            }
            73 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c = self.c2rust_unnamed_0.c2rust_unnamed.c;
            }
            74 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c = self.c2rust_unnamed_1.c2rust_unnamed.d;
            }
            75 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c = self.c2rust_unnamed_1.c2rust_unnamed.e;
            }
            76 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c = self.c2rust_unnamed_2.c2rust_unnamed.h;
            }
            77 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c = self.c2rust_unnamed_2.c2rust_unnamed.l;
            }
            87 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .d = self.c2rust_unnamed.c2rust_unnamed.a;
            }
            80 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .d = self.c2rust_unnamed_0.c2rust_unnamed.b;
            }
            81 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .d = self.c2rust_unnamed_0.c2rust_unnamed.c;
            }
            82 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .d = self.c2rust_unnamed_1.c2rust_unnamed.d;
            }
            83 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .d = self.c2rust_unnamed_1.c2rust_unnamed.e;
            }
            84 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .d = self.c2rust_unnamed_2.c2rust_unnamed.h;
            }
            85 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .d = self.c2rust_unnamed_2.c2rust_unnamed.l;
            }
            95 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .e = self.c2rust_unnamed.c2rust_unnamed.a;
            }
            88 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .e = self.c2rust_unnamed_0.c2rust_unnamed.b;
            }
            89 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .e = self.c2rust_unnamed_0.c2rust_unnamed.c;
            }
            90 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .e = self.c2rust_unnamed_1.c2rust_unnamed.d;
            }
            91 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .e = self.c2rust_unnamed_1.c2rust_unnamed.e;
            }
            92 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .e = self.c2rust_unnamed_2.c2rust_unnamed.h;
            }
            93 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .e = self.c2rust_unnamed_2.c2rust_unnamed.l;
            }
            103 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_2
                    .c2rust_unnamed
                    .h = self.c2rust_unnamed.c2rust_unnamed.a;
            }
            96 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_2
                    .c2rust_unnamed
                    .h = self.c2rust_unnamed_0.c2rust_unnamed.b;
            }
            97 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_2
                    .c2rust_unnamed
                    .h = self.c2rust_unnamed_0.c2rust_unnamed.c;
            }
            98 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_2
                    .c2rust_unnamed
                    .h = self.c2rust_unnamed_1.c2rust_unnamed.d;
            }
            99 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_2
                    .c2rust_unnamed
                    .h = self.c2rust_unnamed_1.c2rust_unnamed.e;
            }
            100 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_2
                    .c2rust_unnamed
                    .h = self.c2rust_unnamed_2.c2rust_unnamed.h;
            }
            101 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_2
                    .c2rust_unnamed
                    .h = self.c2rust_unnamed_2.c2rust_unnamed.l;
            }
            111 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_2
                    .c2rust_unnamed
                    .l = self.c2rust_unnamed.c2rust_unnamed.a;
            }
            104 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_2
                    .c2rust_unnamed
                    .l = self.c2rust_unnamed_0.c2rust_unnamed.b;
            }
            105 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_2
                    .c2rust_unnamed
                    .l = self.c2rust_unnamed_0.c2rust_unnamed.c;
            }
            106 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_2
                    .c2rust_unnamed
                    .l = self.c2rust_unnamed_1.c2rust_unnamed.d;
            }
            107 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_2
                    .c2rust_unnamed
                    .l = self.c2rust_unnamed_1.c2rust_unnamed.e;
            }
            108 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_2
                    .c2rust_unnamed
                    .l = self.c2rust_unnamed_2.c2rust_unnamed.h;
            }
            109 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_2
                    .c2rust_unnamed
                    .l = self.c2rust_unnamed_2.c2rust_unnamed.l;
            }
            126 => {
                cyc = cyc.wrapping_add(7);
                self.c2rust_unnamed.c2rust_unnamed.a = self.ctrl.read_byte(self.c2rust_unnamed_2.hl);
            }
            70 => {
                cyc = cyc.wrapping_add(7);
                self.c2rust_unnamed_0.c2rust_unnamed.b = self.ctrl.read_byte(self.c2rust_unnamed_2.hl);
            }
            78 => {
                cyc = cyc.wrapping_add(7);
                self.c2rust_unnamed_0.c2rust_unnamed.c = self.ctrl.read_byte(self.c2rust_unnamed_2.hl);
            }
            86 => {
                cyc = cyc.wrapping_add(7);
                self.c2rust_unnamed_1.c2rust_unnamed.d = self.ctrl.read_byte(self.c2rust_unnamed_2.hl);
            }
            94 => {
                cyc = cyc.wrapping_add(7);
                self.c2rust_unnamed_1.c2rust_unnamed.e = self.ctrl.read_byte(self.c2rust_unnamed_2.hl);
            }
            102 => {
                cyc = cyc.wrapping_add(7);
                self.c2rust_unnamed_2.c2rust_unnamed.h = self.ctrl.read_byte(self.c2rust_unnamed_2.hl);
            }
            110 => {
                cyc = cyc.wrapping_add(7);
                self.c2rust_unnamed_2.c2rust_unnamed.l = self.ctrl.read_byte(self.c2rust_unnamed_2.hl);
            }
            119 => {
                cyc = cyc.wrapping_add(7);
                self.ctrl.write_byte(self.c2rust_unnamed_2.hl, self.c2rust_unnamed.c2rust_unnamed.a);
            }
            112 => {
                cyc = cyc.wrapping_add(7);
                self.ctrl.write_byte(self.c2rust_unnamed_2.hl, self.c2rust_unnamed_0.c2rust_unnamed.b);
            }
            113 => {
                cyc = cyc.wrapping_add(7);
                self.ctrl.write_byte(self.c2rust_unnamed_2.hl, self.c2rust_unnamed_0.c2rust_unnamed.c);
            }
            114 => {
                cyc = cyc.wrapping_add(7);
                self.ctrl.write_byte(self.c2rust_unnamed_2.hl, self.c2rust_unnamed_1.c2rust_unnamed.d);
            }
            115 => {
                cyc = cyc.wrapping_add(7);
                self.ctrl.write_byte(self.c2rust_unnamed_2.hl, self.c2rust_unnamed_1.c2rust_unnamed.e);
            }
            116 => {
                cyc = cyc.wrapping_add(7);
                self.ctrl.write_byte(self.c2rust_unnamed_2.hl, self.c2rust_unnamed_2.c2rust_unnamed.h);
            }
            117 => {
                cyc = cyc.wrapping_add(7);
                self.ctrl.write_byte(self.c2rust_unnamed_2.hl, self.c2rust_unnamed_2.c2rust_unnamed.l);
            }
            62 => {
                cyc = cyc.wrapping_add(7);
                self.c2rust_unnamed.c2rust_unnamed.a = self.nextb();
            }
            6 => {
                cyc = cyc.wrapping_add(7);
                self.c2rust_unnamed_0.c2rust_unnamed.b = self.nextb();
            }
            14 => {
                cyc = cyc.wrapping_add(7);
                self.c2rust_unnamed_0.c2rust_unnamed.c = self.nextb();
            }
            22 => {
                cyc = cyc.wrapping_add(7);
                self.c2rust_unnamed_1.c2rust_unnamed.d = self.nextb();
            }
            30 => {
                cyc = cyc.wrapping_add(7);
                self.c2rust_unnamed_1.c2rust_unnamed.e = self.nextb();
            }
            38 => {
                cyc = cyc.wrapping_add(7);
                self.c2rust_unnamed_2.c2rust_unnamed.h = self.nextb();
            }
            46 => {
                cyc = cyc.wrapping_add(7);
                self.c2rust_unnamed_2.c2rust_unnamed.l = self.nextb();
            }
            54 => {
                cyc = cyc.wrapping_add(10);
                self.ctrl.write_byte(self.c2rust_unnamed_2.hl, self.nextb());
            }
            10 => {
                cyc = cyc.wrapping_add(7);
                self.c2rust_unnamed.c2rust_unnamed.a = self.ctrl.read_byte(self.c2rust_unnamed_0.bc);
                self
                    .mem_ptr = (self.c2rust_unnamed_0.bc as i32 + 1 as i32)
                    as uint16_t;
            }
            26 => {
                cyc = cyc.wrapping_add(7);
                self.c2rust_unnamed.c2rust_unnamed.a = self.ctrl.read_byte(self.c2rust_unnamed_1.de);
                self
                    .mem_ptr = (self.c2rust_unnamed_1.de as i32 + 1 as i32)
                    as uint16_t;
            }
            58 => {
                cyc = cyc.wrapping_add(13);
                let addr: uint16_t = self.nextw();
                self.c2rust_unnamed.c2rust_unnamed.a = self.ctrl.read_byte(addr);
                self.mem_ptr = (addr as i32 + 1 as i32) as uint16_t;
            }
            2 => {
                cyc = cyc.wrapping_add(7);
                self.ctrl.write_byte(self.c2rust_unnamed_0.bc, self.c2rust_unnamed.c2rust_unnamed.a);
                self
                    .mem_ptr = ((self.c2rust_unnamed.c2rust_unnamed.a as i32)
                    << 8 as i32
                    | self.c2rust_unnamed_0.bc as i32 + 1 as i32
                        & 0xff as i32) as uint16_t;
            }
            18 => {
                cyc = cyc.wrapping_add(7);
                self.ctrl.write_byte(self.c2rust_unnamed_1.de, self.c2rust_unnamed.c2rust_unnamed.a);
                self
                    .mem_ptr = ((self.c2rust_unnamed.c2rust_unnamed.a as i32)
                    << 8 as i32
                    | self.c2rust_unnamed_1.de as i32 + 1 as i32
                        & 0xff as i32) as uint16_t;
            }
            50 => {
                cyc = cyc.wrapping_add(13);
                let addr_0: uint16_t = self.nextw();
                self.ctrl.write_byte(addr_0, self.c2rust_unnamed.c2rust_unnamed.a);
                self
                    .mem_ptr = ((self.c2rust_unnamed.c2rust_unnamed.a as i32)
                    << 8 as i32
                    | addr_0 as i32 + 1 as i32 & 0xff as i32)
                    as uint16_t;
            }
            1 => {
                cyc = cyc.wrapping_add(10);
                self.c2rust_unnamed_0.bc = self.nextw();
            }
            17 => {
                cyc = cyc.wrapping_add(10);
                self.c2rust_unnamed_1.de = self.nextw();
            }
            33 => {
                cyc = cyc.wrapping_add(10);
                self.c2rust_unnamed_2.hl = self.nextw();
            }
            49 => {
                cyc = cyc.wrapping_add(10);
                self.sp = self.nextw();
            }
            42 => {
                cyc = cyc.wrapping_add(16);
                let addr_1: uint16_t = self.nextw();
                self.c2rust_unnamed_2.hl = self.rw(addr_1);
                self.mem_ptr = (addr_1 as i32 + 1 as i32) as uint16_t;
            }
            34 => {
                cyc = cyc.wrapping_add(16);
                let addr_2: uint16_t = self.nextw();
                self.ww(addr_2, self.c2rust_unnamed_2.hl);
                self.mem_ptr = (addr_2 as i32 + 1 as i32) as uint16_t;
            }
            249 => {
                cyc = cyc.wrapping_add(6);
                self.sp = self.c2rust_unnamed_2.hl;
            }
            235 => {
                cyc = cyc.wrapping_add(4);
                let de: uint16_t = self.c2rust_unnamed_1.de;
                self.c2rust_unnamed_1.de = self.c2rust_unnamed_2.hl;
                self.c2rust_unnamed_2.hl = de;
            }
            227 => {
                cyc = cyc.wrapping_add(19);
                let val: uint16_t = self.rw(self.sp);
                self.ww(self.sp, self.c2rust_unnamed_2.hl);
                self.c2rust_unnamed_2.hl = val;
                self.mem_ptr = val;
            }
            135 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    0 as i32 != 0,
                );
            }
            128 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_0.c2rust_unnamed.b as uint32_t,
                    0 as i32 != 0,
                );
            }
            129 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_0.c2rust_unnamed.c as uint32_t,
                    0 as i32 != 0,
                );
            }
            130 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_1.c2rust_unnamed.d as uint32_t,
                    0 as i32 != 0,
                );
            }
            131 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_1.c2rust_unnamed.e as uint32_t,
                    0 as i32 != 0,
                );
            }
            132 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_2.c2rust_unnamed.h as uint32_t,
                    0 as i32 != 0,
                );
            }
            133 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_2.c2rust_unnamed.l as uint32_t,
                    0 as i32 != 0,
                );
            }
            134 => {
                cyc = cyc.wrapping_add(7);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.ctrl.read_byte(self.c2rust_unnamed_2.hl) as uint32_t,
                    0 as i32 != 0,
                );
            }
            198 => {
                cyc = cyc.wrapping_add(7);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.nextb() as uint32_t,
                    0 as i32 != 0,
                );
            }
            143 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.flag_get(cf),
                );
            }
            136 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_0.c2rust_unnamed.b as uint32_t,
                    self.flag_get(cf),
                );
            }
            137 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_0.c2rust_unnamed.c as uint32_t,
                    self.flag_get(cf),
                );
            }
            138 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_1.c2rust_unnamed.d as uint32_t,
                    self.flag_get(cf),
                );
            }
            139 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_1.c2rust_unnamed.e as uint32_t,
                    self.flag_get(cf),
                );
            }
            140 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_2.c2rust_unnamed.h as uint32_t,
                    self.flag_get(cf),
                );
            }
            141 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_2.c2rust_unnamed.l as uint32_t,
                    self.flag_get(cf),
                );
            }
            142 => {
                cyc = cyc.wrapping_add(7);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.ctrl.read_byte(self.c2rust_unnamed_2.hl) as uint32_t,
                    self.flag_get(cf),
                );
            }
            206 => {
                cyc = cyc.wrapping_add(7);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.nextb() as uint32_t,
                    self.flag_get(cf),
                );
            }
            151 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    0 as i32 != 0,
                );
            }
            144 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_0.c2rust_unnamed.b as uint32_t,
                    0 as i32 != 0,
                );
            }
            145 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_0.c2rust_unnamed.c as uint32_t,
                    0 as i32 != 0,
                );
            }
            146 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_1.c2rust_unnamed.d as uint32_t,
                    0 as i32 != 0,
                );
            }
            147 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_1.c2rust_unnamed.e as uint32_t,
                    0 as i32 != 0,
                );
            }
            148 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_2.c2rust_unnamed.h as uint32_t,
                    0 as i32 != 0,
                );
            }
            149 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_2.c2rust_unnamed.l as uint32_t,
                    0 as i32 != 0,
                );
            }
            150 => {
                cyc = cyc.wrapping_add(7);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.ctrl.read_byte(self.c2rust_unnamed_2.hl) as uint32_t,
                    0 as i32 != 0,
                );
            }
            214 => {
                cyc = cyc.wrapping_add(7);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.nextb() as uint32_t,
                    0 as i32 != 0,
                );
            }
            159 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.flag_get(cf),
                );
            }
            152 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_0.c2rust_unnamed.b as uint32_t,
                    self.flag_get(cf),
                );
            }
            153 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_0.c2rust_unnamed.c as uint32_t,
                    self.flag_get(cf),
                );
            }
            154 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_1.c2rust_unnamed.d as uint32_t,
                    self.flag_get(cf),
                );
            }
            155 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_1.c2rust_unnamed.e as uint32_t,
                    self.flag_get(cf),
                );
            }
            156 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_2.c2rust_unnamed.h as uint32_t,
                    self.flag_get(cf),
                );
            }
            157 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.c2rust_unnamed_2.c2rust_unnamed.l as uint32_t,
                    self.flag_get(cf),
                );
            }
            158 => {
                cyc = cyc.wrapping_add(7);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.ctrl.read_byte(self.c2rust_unnamed_2.hl) as uint32_t,
                    self.flag_get(cf),
                );
            }
            222 => {
                cyc = cyc.wrapping_add(7);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.nextb() as uint32_t,
                    self.flag_get(cf),
                );
            }
            9 => {
                cyc = cyc.wrapping_add(11);
                self.addhl(self.c2rust_unnamed_0.bc);
            }
            25 => {
                cyc = cyc.wrapping_add(11);
                self.addhl(self.c2rust_unnamed_1.de);
            }
            41 => {
                cyc = cyc.wrapping_add(11);
                self.addhl(self.c2rust_unnamed_2.hl);
            }
            57 => {
                cyc = cyc.wrapping_add(11);
                self.addhl(self.sp);
            }
            243 => {
                cyc = cyc.wrapping_add(4);
                self.iff2 = 0 as i32 != 0;
                self.iff1 = self.iff2;
            }
            251 => {
                cyc = cyc.wrapping_add(4);
                self.iff_delay = 1 as i32 as uint8_t;
            }
            0 => {
                cyc = cyc.wrapping_add(4);
            }
            118 => {
                cyc = cyc.wrapping_add(4);
                self.halted = 1 as i32 != 0;
            }
            60 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.inc(self.c2rust_unnamed.c2rust_unnamed.a);
            }
            4 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .b = self.inc(self.c2rust_unnamed_0.c2rust_unnamed.b);
            }
            12 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c = self.inc(self.c2rust_unnamed_0.c2rust_unnamed.c);
            }
            20 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .d = self.inc(self.c2rust_unnamed_1.c2rust_unnamed.d);
            }
            28 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .e = self.inc(self.c2rust_unnamed_1.c2rust_unnamed.e);
            }
            36 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_2
                    .c2rust_unnamed
                    .h = self.inc(self.c2rust_unnamed_2.c2rust_unnamed.h);
            }
            44 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_2
                    .c2rust_unnamed
                    .l = self.inc(self.c2rust_unnamed_2.c2rust_unnamed.l);
            }
            52 => {
                cyc = cyc.wrapping_add(11);
                let mut result: uint8_t = self.inc(self.ctrl.read_byte(self.c2rust_unnamed_2.hl));
                self.ctrl.write_byte(self.c2rust_unnamed_2.hl, result);
            }
            61 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.dec(self.c2rust_unnamed.c2rust_unnamed.a);
            }
            5 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .b = self.dec(self.c2rust_unnamed_0.c2rust_unnamed.b);
            }
            13 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c = self.dec(self.c2rust_unnamed_0.c2rust_unnamed.c);
            }
            21 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .d = self.dec(self.c2rust_unnamed_1.c2rust_unnamed.d);
            }
            29 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .e = self.dec(self.c2rust_unnamed_1.c2rust_unnamed.e);
            }
            37 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_2
                    .c2rust_unnamed
                    .h = self.dec(self.c2rust_unnamed_2.c2rust_unnamed.h);
            }
            45 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed_2
                    .c2rust_unnamed
                    .l = self.dec(self.c2rust_unnamed_2.c2rust_unnamed.l);
            }
            53 => {
                cyc = cyc.wrapping_add(11);
                let mut result_0: uint8_t = self.dec(self.ctrl.read_byte(self.c2rust_unnamed_2.hl));
                self.ctrl.write_byte(self.c2rust_unnamed_2.hl, result_0);
            }
            3 => {
                cyc = cyc.wrapping_add(6);
                self.c2rust_unnamed_0.bc = (self.c2rust_unnamed_0.bc).wrapping_add(1);
            }
            19 => {
                cyc = cyc.wrapping_add(6);
                self.c2rust_unnamed_1.de = (self.c2rust_unnamed_1.de).wrapping_add(1);
            }
            35 => {
                cyc = cyc.wrapping_add(6);
                self.c2rust_unnamed_2.hl = (self.c2rust_unnamed_2.hl).wrapping_add(1);
            }
            51 => {
                cyc = cyc.wrapping_add(6);
                self.sp = (self.sp).wrapping_add(1);
            }
            11 => {
                cyc = cyc.wrapping_add(6);
                self.c2rust_unnamed_0.bc = (self.c2rust_unnamed_0.bc).wrapping_sub(1);
            }
            27 => {
                cyc = cyc.wrapping_add(6);
                self.c2rust_unnamed_1.de = (self.c2rust_unnamed_1.de).wrapping_sub(1);
            }
            43 => {
                cyc = cyc.wrapping_add(6);
                self.c2rust_unnamed_2.hl = (self.c2rust_unnamed_2.hl).wrapping_sub(1);
            }
            59 => {
                cyc = cyc.wrapping_add(6);
                self.sp = (self.sp).wrapping_sub(1);
            }
            39 => {
                cyc = cyc.wrapping_add(4);
                self.daa();
            }
            47 => {
                cyc = cyc.wrapping_add(4);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = !(self.c2rust_unnamed.c2rust_unnamed.a as i32) as uint8_t;
                self.flag_set(nf, 1 as i32 != 0);
                self.flag_set(hf, 1 as i32 != 0);
                self.flag_set(
                    xf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 >> 3 as i32
                        & 1 as i32 != 0,
                );
                self.flag_set(
                    yf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 >> 5 as i32
                        & 1 as i32 != 0,
                );
            }
            55 => {
                cyc = cyc.wrapping_add(4);
                self.flag_set(cf, 1 as i32 != 0);
                self.flag_set(nf, 0 as i32 != 0);
                self.flag_set(hf, 0 as i32 != 0);
                self.flag_set(
                    xf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 >> 3 as i32
                        & 1 as i32 != 0,
                );
                self.flag_set(
                    yf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 >> 5 as i32
                        & 1 as i32 != 0,
                );
            }
            63 => {
                cyc = cyc.wrapping_add(4);
                self.flag_set(hf, self.flag_get(cf));
                self.flag_set(cf, !self.flag_get(cf));
                self.flag_set(nf, 0 as i32 != 0);
                self.flag_set(
                    xf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 >> 3 as i32
                        & 1 as i32 != 0,
                );
                self.flag_set(
                    yf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 >> 5 as i32
                        & 1 as i32 != 0,
                );
            }
            7 => {
                cyc = cyc.wrapping_add(4);
                self.flag_set(
                    cf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 >> 7 as i32
                        != 0,
                );
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = ((self.c2rust_unnamed.c2rust_unnamed.a as i32)
                    << 1 as i32 | self.flag_get(cf) as i32) as uint8_t;
                self.flag_set(nf, 0 as i32 != 0);
                self.flag_set(hf, 0 as i32 != 0);
                self.flag_set(
                    xf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 >> 3 as i32
                        & 1 as i32 != 0,
                );
                self.flag_set(
                    yf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 >> 5 as i32
                        & 1 as i32 != 0,
                );
            }
            15 => {
                cyc = cyc.wrapping_add(4);
                self.flag_set(
                    cf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 & 1 as i32
                        != 0,
                );
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = (self.c2rust_unnamed.c2rust_unnamed.a as i32
                    >> 1 as i32
                    | (self.flag_get(cf) as i32) << 7 as i32) as uint8_t;
                self.flag_set(nf, 0 as i32 != 0);
                self.flag_set(hf, 0 as i32 != 0);
                self.flag_set(
                    xf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 >> 3 as i32
                        & 1 as i32 != 0,
                );
                self.flag_set(
                    yf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 >> 5 as i32
                        & 1 as i32 != 0,
                );
            }
            23 => {
                cyc = cyc.wrapping_add(4);
                let cy: bool = self.flag_get(cf);
                self.flag_set(
                    cf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 >> 7 as i32
                        != 0,
                );
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = ((self.c2rust_unnamed.c2rust_unnamed.a as i32)
                    << 1 as i32 | cy as i32) as uint8_t;
                self.flag_set(nf, 0 as i32 != 0);
                self.flag_set(hf, 0 as i32 != 0);
                self.flag_set(
                    xf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 >> 3 as i32
                        & 1 as i32 != 0,
                );
                self.flag_set(
                    yf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 >> 5 as i32
                        & 1 as i32 != 0,
                );
            }
            31 => {
                cyc = cyc.wrapping_add(4);
                let cy_0: bool = self.flag_get(cf);
                self.flag_set(
                    cf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 & 1 as i32
                        != 0,
                );
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = (self.c2rust_unnamed.c2rust_unnamed.a as i32
                    >> 1 as i32 | (cy_0 as i32) << 7 as i32)
                    as uint8_t;
                self.flag_set(nf, 0 as i32 != 0);
                self.flag_set(hf, 0 as i32 != 0);
                self.flag_set(
                    xf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 >> 3 as i32
                        & 1 as i32 != 0,
                );
                self.flag_set(
                    yf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 >> 5 as i32
                        & 1 as i32 != 0,
                );
            }
            167 => {
                cyc = cyc.wrapping_add(4);
                self.land(self.c2rust_unnamed.c2rust_unnamed.a);
            }
            160 => {
                cyc = cyc.wrapping_add(4);
                self.land(self.c2rust_unnamed_0.c2rust_unnamed.b);
            }
            161 => {
                cyc = cyc.wrapping_add(4);
                self.land(self.c2rust_unnamed_0.c2rust_unnamed.c);
            }
            162 => {
                cyc = cyc.wrapping_add(4);
                self.land(self.c2rust_unnamed_1.c2rust_unnamed.d);
            }
            163 => {
                cyc = cyc.wrapping_add(4);
                self.land(self.c2rust_unnamed_1.c2rust_unnamed.e);
            }
            164 => {
                cyc = cyc.wrapping_add(4);
                self.land(self.c2rust_unnamed_2.c2rust_unnamed.h);
            }
            165 => {
                cyc = cyc.wrapping_add(4);
                self.land(self.c2rust_unnamed_2.c2rust_unnamed.l);
            }
            166 => {
                cyc = cyc.wrapping_add(7);
                self.land(self.ctrl.read_byte(self.c2rust_unnamed_2.hl));
            }
            230 => {
                cyc = cyc.wrapping_add(7);
                self.land(self.nextb());
            }
            175 => {
                cyc = cyc.wrapping_add(4);
                self.lxor(self.c2rust_unnamed.c2rust_unnamed.a);
            }
            168 => {
                cyc = cyc.wrapping_add(4);
                self.lxor(self.c2rust_unnamed_0.c2rust_unnamed.b);
            }
            169 => {
                cyc = cyc.wrapping_add(4);
                self.lxor(self.c2rust_unnamed_0.c2rust_unnamed.c);
            }
            170 => {
                cyc = cyc.wrapping_add(4);
                self.lxor(self.c2rust_unnamed_1.c2rust_unnamed.d);
            }
            171 => {
                cyc = cyc.wrapping_add(4);
                self.lxor(self.c2rust_unnamed_1.c2rust_unnamed.e);
            }
            172 => {
                cyc = cyc.wrapping_add(4);
                self.lxor(self.c2rust_unnamed_2.c2rust_unnamed.h);
            }
            173 => {
                cyc = cyc.wrapping_add(4);
                self.lxor(self.c2rust_unnamed_2.c2rust_unnamed.l);
            }
            174 => {
                cyc = cyc.wrapping_add(7);
                self.lxor(self.ctrl.read_byte(self.c2rust_unnamed_2.hl));
            }
            238 => {
                cyc = cyc.wrapping_add(7);
                self.lxor(self.nextb());
            }
            183 => {
                cyc = cyc.wrapping_add(4);
                self.lor(self.c2rust_unnamed.c2rust_unnamed.a);
            }
            176 => {
                cyc = cyc.wrapping_add(4);
                self.lor(self.c2rust_unnamed_0.c2rust_unnamed.b);
            }
            177 => {
                cyc = cyc.wrapping_add(4);
                self.lor(self.c2rust_unnamed_0.c2rust_unnamed.c);
            }
            178 => {
                cyc = cyc.wrapping_add(4);
                self.lor(self.c2rust_unnamed_1.c2rust_unnamed.d);
            }
            179 => {
                cyc = cyc.wrapping_add(4);
                self.lor(self.c2rust_unnamed_1.c2rust_unnamed.e);
            }
            180 => {
                cyc = cyc.wrapping_add(4);
                self.lor(self.c2rust_unnamed_2.c2rust_unnamed.h);
            }
            181 => {
                cyc = cyc.wrapping_add(4);
                self.lor(self.c2rust_unnamed_2.c2rust_unnamed.l);
            }
            182 => {
                cyc = cyc.wrapping_add(7);
                self.lor(self.ctrl.read_byte(self.c2rust_unnamed_2.hl));
            }
            246 => {
                cyc = cyc.wrapping_add(7);
                self.lor(self.nextb());
            }
            191 => {
                cyc = cyc.wrapping_add(4);
                self.cp(self.c2rust_unnamed.c2rust_unnamed.a as uint32_t);
            }
            184 => {
                cyc = cyc.wrapping_add(4);
                self.cp(self.c2rust_unnamed_0.c2rust_unnamed.b as uint32_t);
            }
            185 => {
                cyc = cyc.wrapping_add(4);
                self.cp(self.c2rust_unnamed_0.c2rust_unnamed.c as uint32_t);
            }
            186 => {
                cyc = cyc.wrapping_add(4);
                self.cp(self.c2rust_unnamed_1.c2rust_unnamed.d as uint32_t);
            }
            187 => {
                cyc = cyc.wrapping_add(4);
                self.cp(self.c2rust_unnamed_1.c2rust_unnamed.e as uint32_t);
            }
            188 => {
                cyc = cyc.wrapping_add(4);
                self.cp(self.c2rust_unnamed_2.c2rust_unnamed.h as uint32_t);
            }
            189 => {
                cyc = cyc.wrapping_add(4);
                self.cp(self.c2rust_unnamed_2.c2rust_unnamed.l as uint32_t);
            }
            190 => {
                cyc = cyc.wrapping_add(7);
                self.cp(self.ctrl.read_byte(self.c2rust_unnamed_2.hl) as uint32_t);
            }
            254 => {
                cyc = cyc.wrapping_add(7);
                self.cp(self.nextb() as uint32_t);
            }
            195 => {
                cyc = cyc.wrapping_add(10);
                self.jump(self.nextw());
            }
            194 => {
                cyc = cyc.wrapping_add(10);
                self.cond_jump(self.flag_get(zf) as i32 == 0 as i32);
            }
            202 => {
                cyc = cyc.wrapping_add(10);
                self.cond_jump(self.flag_get(zf) as i32 == 1 as i32);
            }
            210 => {
                cyc = cyc.wrapping_add(10);
                self.cond_jump(self.flag_get(cf) as i32 == 0 as i32);
            }
            218 => {
                cyc = cyc.wrapping_add(10);
                self.cond_jump(self.flag_get(cf) as i32 == 1 as i32);
            }
            226 => {
                cyc = cyc.wrapping_add(10);
                self.cond_jump(self.flag_get(pf) as i32 == 0 as i32);
            }
            234 => {
                cyc = cyc.wrapping_add(10);
                self.cond_jump(self.flag_get(pf) as i32 == 1 as i32);
            }
            242 => {
                cyc = cyc.wrapping_add(10);
                self.cond_jump(self.flag_get(sf) as i32 == 0 as i32);
            }
            250 => {
                cyc = cyc.wrapping_add(10);
                self.cond_jump(self.flag_get(sf) as i32 == 1 as i32);
            }
            16 => {
                cyc = cyc.wrapping_add(8);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .b = (self.c2rust_unnamed_0.c2rust_unnamed.b).wrapping_sub(1);
                cyc = cyc
                    .wrapping_add(
                        self.cond_jr(
                            self.c2rust_unnamed_0.c2rust_unnamed.b as i32
                                != 0 as i32,
                        ),
                    );
            }
            24 => {
                cyc = cyc.wrapping_add(12);
                self.jr(self.nextb() as int8_t);
            }
            32 => {
                cyc = cyc.wrapping_add(7);
                cyc = cyc
                    .wrapping_add(
                        self.cond_jr(self.flag_get(zf) as i32 == 0 as i32),
                    );
            }
            40 => {
                cyc = cyc.wrapping_add(7);
                cyc = cyc
                    .wrapping_add(
                        self.cond_jr(self.flag_get(zf) as i32 == 1 as i32),
                    );
            }
            48 => {
                cyc = cyc.wrapping_add(7);
                cyc = cyc
                    .wrapping_add(
                        self.cond_jr(self.flag_get(cf) as i32 == 0 as i32),
                    );
            }
            56 => {
                cyc = cyc.wrapping_add(7);
                cyc = cyc
                    .wrapping_add(
                        self.cond_jr(self.flag_get(cf) as i32 == 1 as i32),
                    );
            }
            233 => {
                cyc = cyc.wrapping_add(4);
                self.pc = self.c2rust_unnamed_2.hl;
            }
            205 => {
                cyc = cyc.wrapping_add(17);
                self.call(self.nextw());
            }
            196 => {
                cyc = cyc.wrapping_add(10);
                cyc = cyc
                    .wrapping_add(
                        self.cond_call(self.flag_get(zf) as i32 == 0 as i32),
                    );
            }
            204 => {
                cyc = cyc.wrapping_add(10);
                cyc = cyc
                    .wrapping_add(
                        self.cond_call(self.flag_get(zf) as i32 == 1 as i32),
                    );
            }
            212 => {
                cyc = cyc.wrapping_add(10);
                cyc = cyc
                    .wrapping_add(
                        self.cond_call(self.flag_get(cf) as i32 == 0 as i32),
                    );
            }
            220 => {
                cyc = cyc.wrapping_add(10);
                cyc = cyc
                    .wrapping_add(
                        self.cond_call(self.flag_get(cf) as i32 == 1 as i32),
                    );
            }
            228 => {
                cyc = cyc.wrapping_add(10);
                cyc = cyc
                    .wrapping_add(
                        self.cond_call(self.flag_get(pf) as i32 == 0 as i32),
                    );
            }
            236 => {
                cyc = cyc.wrapping_add(10);
                cyc = cyc
                    .wrapping_add(
                        self.cond_call(self.flag_get(pf) as i32 == 1 as i32),
                    );
            }
            244 => {
                cyc = cyc.wrapping_add(10);
                cyc = cyc
                    .wrapping_add(
                        self.cond_call(self.flag_get(sf) as i32 == 0 as i32),
                    );
            }
            252 => {
                cyc = cyc.wrapping_add(10);
                cyc = cyc
                    .wrapping_add(
                        self.cond_call(self.flag_get(sf) as i32 == 1 as i32),
                    );
            }
            201 => {
                cyc = cyc.wrapping_add(10);
                self.ret();
            }
            192 => {
                cyc = cyc.wrapping_add(5);
                cyc = cyc
                    .wrapping_add(
                        self.cond_ret(self.flag_get(zf) as i32 == 0 as i32),
                    );
            }
            200 => {
                cyc = cyc.wrapping_add(5);
                cyc = cyc
                    .wrapping_add(
                        self.cond_ret(self.flag_get(zf) as i32 == 1 as i32),
                    );
            }
            208 => {
                cyc = cyc.wrapping_add(5);
                cyc = cyc
                    .wrapping_add(
                        self.cond_ret(self.flag_get(cf) as i32 == 0 as i32),
                    );
            }
            216 => {
                cyc = cyc.wrapping_add(5);
                cyc = cyc
                    .wrapping_add(
                        self.cond_ret(self.flag_get(cf) as i32 == 1 as i32),
                    );
            }
            224 => {
                cyc = cyc.wrapping_add(5);
                cyc = cyc
                    .wrapping_add(
                        self.cond_ret(self.flag_get(pf) as i32 == 0 as i32),
                    );
            }
            232 => {
                cyc = cyc.wrapping_add(5);
                cyc = cyc
                    .wrapping_add(
                        self.cond_ret(self.flag_get(pf) as i32 == 1 as i32),
                    );
            }
            240 => {
                cyc = cyc.wrapping_add(5);
                cyc = cyc
                    .wrapping_add(
                        self.cond_ret(self.flag_get(sf) as i32 == 0 as i32),
                    );
            }
            248 => {
                cyc = cyc.wrapping_add(5);
                cyc = cyc
                    .wrapping_add(
                        self.cond_ret(self.flag_get(sf) as i32 == 1 as i32),
                    );
            }
            199 => {
                cyc = cyc.wrapping_add(11);
                self.call(0 as i32 as uint16_t);
            }
            207 => {
                cyc = cyc.wrapping_add(11);
                self.call(0x8 as i32 as uint16_t);
            }
            215 => {
                cyc = cyc.wrapping_add(11);
                self.call(0x10 as i32 as uint16_t);
            }
            223 => {
                cyc = cyc.wrapping_add(11);
                self.call(0x18 as i32 as uint16_t);
            }
            231 => {
                cyc = cyc.wrapping_add(11);
                self.call(0x20 as i32 as uint16_t);
            }
            239 => {
                cyc = cyc.wrapping_add(11);
                self.call(0x28 as i32 as uint16_t);
            }
            247 => {
                cyc = cyc.wrapping_add(11);
                self.call(0x30 as i32 as uint16_t);
            }
            255 => {
                cyc = cyc.wrapping_add(11);
                self.call(0x38 as i32 as uint16_t);
            }
            197 => {
                cyc = cyc.wrapping_add(11);
                self.pushw(self.c2rust_unnamed_0.bc);
            }
            213 => {
                cyc = cyc.wrapping_add(11);
                self.pushw(self.c2rust_unnamed_1.de);
            }
            229 => {
                cyc = cyc.wrapping_add(11);
                self.pushw(self.c2rust_unnamed_2.hl);
            }
            245 => {
                cyc = cyc.wrapping_add(11);
                self.pushw(self.c2rust_unnamed.af);
            }
            193 => {
                cyc = cyc.wrapping_add(10);
                self.c2rust_unnamed_0.bc = self.popw();
            }
            209 => {
                cyc = cyc.wrapping_add(10);
                self.c2rust_unnamed_1.de = self.popw();
            }
            225 => {
                cyc = cyc.wrapping_add(10);
                self.c2rust_unnamed_2.hl = self.popw();
            }
            241 => {
                cyc = cyc.wrapping_add(10);
                self.c2rust_unnamed.af = self.popw();
            }
            219 => {
                cyc = cyc.wrapping_add(11);
                let port: uint16_t = (self.nextb() as i32
                    | (self.c2rust_unnamed.c2rust_unnamed.a as i32)
                        << 8 as i32) as uint16_t;
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.internal_port_in(port);
                self.mem_ptr = (port as i32 + 1 as i32) as uint16_t;
            }
            211 => {
                cyc = cyc.wrapping_add(11);
                let port_0: uint16_t = (self.nextb() as i32
                    | (self.c2rust_unnamed.c2rust_unnamed.a as i32)
                        << 8 as i32) as uint16_t;
                self.internal_port_out(port_0, self.c2rust_unnamed.c2rust_unnamed.a);
                self
                    .mem_ptr = (port_0 as i32 + 1 as i32
                    & 0xff as i32
                    | (self.c2rust_unnamed.c2rust_unnamed.a as i32)
                        << 8 as i32) as uint16_t;
            }
            8 => {
                cyc = cyc.wrapping_add(4);
                let mut af: uint16_t = self.c2rust_unnamed.af;
                self.c2rust_unnamed.af = self.c2rust_unnamed_3.a_f_;
                self.c2rust_unnamed_3.a_f_ = af;
            }
            217 => {
                cyc = cyc.wrapping_add(4);
                let mut bc: uint16_t = self.c2rust_unnamed_0.bc;
                let mut de_0: uint16_t = self.c2rust_unnamed_1.de;
                let mut hl: uint16_t = self.c2rust_unnamed_2.hl;
                self.c2rust_unnamed_0.bc = self.c2rust_unnamed_4.b_c_;
                self.c2rust_unnamed_1.de = self.c2rust_unnamed_5.d_e_;
                self.c2rust_unnamed_2.hl = self.c2rust_unnamed_6.h_l_;
                self.c2rust_unnamed_4.b_c_ = bc;
                self.c2rust_unnamed_5.d_e_ = de_0;
                self.c2rust_unnamed_6.h_l_ = hl;
            }
            203 => {
                cyc = cyc.wrapping_add(0);
                cyc = cyc.wrapping_add(self.exec_opcode_cb(self.nextb()));
            }
            237 => {
                cyc = cyc.wrapping_add(0);
                cyc = cyc.wrapping_add(self.exec_opcode_ed(self.nextb()));
            }
            221 => {
                cyc = cyc.wrapping_add(0);
                cyc = cyc.wrapping_add(self.exec_opcode_ddfd(self.nextb(), &mut self.ix));
            }
            253 => {
                cyc = cyc.wrapping_add(0);
                cyc = cyc.wrapping_add(self.exec_opcode_ddfd(self.nextb(), &mut self.iy));
            }
            _ => {}
        }
        return cyc;
    }
    fn exec_opcode_ddfd(
        &mut self,
        mut opcode: uint8_t,
        iz: *mut uint16_t,
    ) -> u32 {
        let mut cyc: u32 = 0;
        self.inc_r();
        match opcode as i32 {
            225 => {
                cyc = cyc.wrapping_add(14);
                *iz = self.popw();
            }
            229 => {
                cyc = cyc.wrapping_add(15);
                self.pushw(*iz);
            }
            233 => {
                cyc = cyc.wrapping_add(8);
                self.jump(*iz);
            }
            9 => {
                cyc = cyc.wrapping_add(15);
                self.addiz(iz, self.c2rust_unnamed_0.bc);
            }
            25 => {
                cyc = cyc.wrapping_add(15);
                self.addiz(iz, self.c2rust_unnamed_1.de);
            }
            41 => {
                cyc = cyc.wrapping_add(15);
                self.addiz(iz, *iz);
            }
            57 => {
                cyc = cyc.wrapping_add(15);
                self.addiz(iz, self.sp);
            }
            132 => {
                cyc = cyc.wrapping_add(8);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    (*iz as i32 >> 8 as i32) as uint32_t,
                    0 as i32 != 0,
                );
            }
            133 => {
                cyc = cyc.wrapping_add(8);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    (*iz as i32 & 0xff as i32) as uint32_t,
                    0 as i32 != 0,
                );
            }
            140 => {
                cyc = cyc.wrapping_add(8);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    (*iz as i32 >> 8 as i32) as uint32_t,
                    self.flag_get(cf),
                );
            }
            141 => {
                cyc = cyc.wrapping_add(8);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    (*iz as i32 & 0xff as i32) as uint32_t,
                    self.flag_get(cf),
                );
            }
            134 => {
                cyc = cyc.wrapping_add(19);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.ctrl.read_byte(self.displace(*iz, self.nextb() as int8_t)) as uint32_t,
                    0 as i32 != 0,
                );
            }
            142 => {
                cyc = cyc.wrapping_add(19);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.addb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.ctrl.read_byte(self.displace(*iz, self.nextb() as int8_t)) as uint32_t,
                    self.flag_get(cf),
                );
            }
            150 => {
                cyc = cyc.wrapping_add(19);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.ctrl.read_byte(self.displace(*iz, self.nextb() as int8_t)) as uint32_t,
                    0 as i32 != 0,
                );
            }
            158 => {
                cyc = cyc.wrapping_add(19);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    self.ctrl.read_byte(self.displace(*iz, self.nextb() as int8_t)) as uint32_t,
                    self.flag_get(cf),
                );
            }
            148 => {
                cyc = cyc.wrapping_add(8);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    (*iz as i32 >> 8 as i32) as uint32_t,
                    0 as i32 != 0,
                );
            }
            149 => {
                cyc = cyc.wrapping_add(8);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    (*iz as i32 & 0xff as i32) as uint32_t,
                    0 as i32 != 0,
                );
            }
            156 => {
                cyc = cyc.wrapping_add(8);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    (*iz as i32 >> 8 as i32) as uint32_t,
                    self.flag_get(cf),
                );
            }
            157 => {
                cyc = cyc.wrapping_add(8);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    (*iz as i32 & 0xff as i32) as uint32_t,
                    self.flag_get(cf),
                );
            }
            166 => {
                cyc = cyc.wrapping_add(19);
                self.land(self.ctrl.read_byte(self.displace(*iz, self.nextb() as int8_t)));
            }
            164 => {
                cyc = cyc.wrapping_add(8);
                self.land((*iz as i32 >> 8 as i32) as uint8_t);
            }
            165 => {
                cyc = cyc.wrapping_add(8);
                self.land((*iz as i32 & 0xff as i32) as uint8_t);
            }
            174 => {
                cyc = cyc.wrapping_add(19);
                self.lxor(self.ctrl.read_byte(self.displace(*iz, self.nextb() as int8_t)));
            }
            172 => {
                cyc = cyc.wrapping_add(8);
                self.lxor((*iz as i32 >> 8 as i32) as uint8_t);
            }
            173 => {
                cyc = cyc.wrapping_add(8);
                self.lxor((*iz as i32 & 0xff as i32) as uint8_t);
            }
            182 => {
                cyc = cyc.wrapping_add(19);
                self.lor(self.ctrl.read_byte(self.displace(*iz, self.nextb() as int8_t)));
            }
            180 => {
                cyc = cyc.wrapping_add(8);
                self.lor((*iz as i32 >> 8 as i32) as uint8_t);
            }
            181 => {
                cyc = cyc.wrapping_add(8);
                self.lor((*iz as i32 & 0xff as i32) as uint8_t);
            }
            190 => {
                cyc = cyc.wrapping_add(19);
                self.cp(self.ctrl.read_byte(self.displace(*iz, self.nextb() as int8_t)) as uint32_t);
            }
            188 => {
                cyc = cyc.wrapping_add(8);
                self.cp((*iz as i32 >> 8 as i32) as uint32_t);
            }
            189 => {
                cyc = cyc.wrapping_add(8);
                self.cp((*iz as i32 & 0xff as i32) as uint32_t);
            }
            35 => {
                cyc = cyc.wrapping_add(10);
                *iz = (*iz as i32 + 1 as i32) as uint16_t;
            }
            43 => {
                cyc = cyc.wrapping_add(10);
                *iz = (*iz as i32 - 1 as i32) as uint16_t;
            }
            52 => {
                cyc = cyc.wrapping_add(23);
                let mut addr: uint16_t = self.displace(*iz, self.nextb() as int8_t);
                self.ctrl.write_byte(addr, self.inc(self.ctrl.read_byte(addr)));
            }
            53 => {
                cyc = cyc.wrapping_add(23);
                let mut addr_0: uint16_t = self.displace(*iz, self.nextb() as int8_t);
                self.ctrl.write_byte(addr_0, self.dec(self.ctrl.read_byte(addr_0)));
            }
            36 => {
                cyc = cyc.wrapping_add(8);
                *iz = (*iz as i32 & 0xff as i32
                    | (self.inc((*iz as i32 >> 8 as i32) as uint8_t)
                        as i32) << 8 as i32) as uint16_t;
            }
            37 => {
                cyc = cyc.wrapping_add(8);
                *iz = (*iz as i32 & 0xff as i32
                    | (self.dec((*iz as i32 >> 8 as i32) as uint8_t)
                        as i32) << 8 as i32) as uint16_t;
            }
            44 => {
                cyc = cyc.wrapping_add(8);
                *iz = ((*iz as i32 >> 8 as i32) << 8 as i32
                    | self.inc((*iz as i32 & 0xff as i32) as uint8_t)
                        as i32) as uint16_t;
            }
            45 => {
                cyc = cyc.wrapping_add(8);
                *iz = ((*iz as i32 >> 8 as i32) << 8 as i32
                    | self.dec((*iz as i32 & 0xff as i32) as uint8_t)
                        as i32) as uint16_t;
            }
            42 => {
                cyc = cyc.wrapping_add(20);
                *iz = self.rw(self.nextw());
            }
            34 => {
                cyc = cyc.wrapping_add(20);
                self.ww(self.nextw(), *iz);
            }
            33 => {
                cyc = cyc.wrapping_add(14);
                *iz = self.nextw();
            }
            54 => {
                cyc = cyc.wrapping_add(19);
                let mut addr_1: uint16_t = self.displace(*iz, self.nextb() as int8_t);
                self.ctrl.write_byte(addr_1, self.nextb());
            }
            112 => {
                cyc = cyc.wrapping_add(19);
                self.ctrl.write_byte(
                    self.displace(*iz, self.nextb() as int8_t),
                    self.c2rust_unnamed_0.c2rust_unnamed.b,
                );
            }
            113 => {
                cyc = cyc.wrapping_add(19);
                self.ctrl.write_byte(
                    self.displace(*iz, self.nextb() as int8_t),
                    self.c2rust_unnamed_0.c2rust_unnamed.c,
                );
            }
            114 => {
                cyc = cyc.wrapping_add(19);
                self.ctrl.write_byte(
                    self.displace(*iz, self.nextb() as int8_t),
                    self.c2rust_unnamed_1.c2rust_unnamed.d,
                );
            }
            115 => {
                cyc = cyc.wrapping_add(19);
                self.ctrl.write_byte(
                    self.displace(*iz, self.nextb() as int8_t),
                    self.c2rust_unnamed_1.c2rust_unnamed.e,
                );
            }
            116 => {
                cyc = cyc.wrapping_add(19);
                self.ctrl.write_byte(
                    self.displace(*iz, self.nextb() as int8_t),
                    self.c2rust_unnamed_2.c2rust_unnamed.h,
                );
            }
            117 => {
                cyc = cyc.wrapping_add(19);
                self.ctrl.write_byte(
                    self.displace(*iz, self.nextb() as int8_t),
                    self.c2rust_unnamed_2.c2rust_unnamed.l,
                );
            }
            119 => {
                cyc = cyc.wrapping_add(19);
                self.ctrl.write_byte(
                    self.displace(*iz, self.nextb() as int8_t),
                    self.c2rust_unnamed.c2rust_unnamed.a,
                );
            }
            70 => {
                cyc = cyc.wrapping_add(19);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .b = self.ctrl.read_byte(self.displace(*iz, self.nextb() as int8_t));
            }
            78 => {
                cyc = cyc.wrapping_add(19);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c = self.ctrl.read_byte(self.displace(*iz, self.nextb() as int8_t));
            }
            86 => {
                cyc = cyc.wrapping_add(19);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .d = self.ctrl.read_byte(self.displace(*iz, self.nextb() as int8_t));
            }
            94 => {
                cyc = cyc.wrapping_add(19);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .e = self.ctrl.read_byte(self.displace(*iz, self.nextb() as int8_t));
            }
            102 => {
                cyc = cyc.wrapping_add(19);
                self
                    .c2rust_unnamed_2
                    .c2rust_unnamed
                    .h = self.ctrl.read_byte(self.displace(*iz, self.nextb() as int8_t));
            }
            110 => {
                cyc = cyc.wrapping_add(19);
                self
                    .c2rust_unnamed_2
                    .c2rust_unnamed
                    .l = self.ctrl.read_byte(self.displace(*iz, self.nextb() as int8_t));
            }
            126 => {
                cyc = cyc.wrapping_add(19);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.ctrl.read_byte(self.displace(*iz, self.nextb() as int8_t));
            }
            68 => {
                cyc = cyc.wrapping_add(8);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .b = (*iz as i32 >> 8 as i32) as uint8_t;
            }
            76 => {
                cyc = cyc.wrapping_add(8);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c = (*iz as i32 >> 8 as i32) as uint8_t;
            }
            84 => {
                cyc = cyc.wrapping_add(8);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .d = (*iz as i32 >> 8 as i32) as uint8_t;
            }
            92 => {
                cyc = cyc.wrapping_add(8);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .e = (*iz as i32 >> 8 as i32) as uint8_t;
            }
            124 => {
                cyc = cyc.wrapping_add(8);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = (*iz as i32 >> 8 as i32) as uint8_t;
            }
            69 => {
                cyc = cyc.wrapping_add(8);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .b = (*iz as i32 & 0xff as i32) as uint8_t;
            }
            77 => {
                cyc = cyc.wrapping_add(8);
                self
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c = (*iz as i32 & 0xff as i32) as uint8_t;
            }
            85 => {
                cyc = cyc.wrapping_add(8);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .d = (*iz as i32 & 0xff as i32) as uint8_t;
            }
            93 => {
                cyc = cyc.wrapping_add(8);
                self
                    .c2rust_unnamed_1
                    .c2rust_unnamed
                    .e = (*iz as i32 & 0xff as i32) as uint8_t;
            }
            125 => {
                cyc = cyc.wrapping_add(8);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = (*iz as i32 & 0xff as i32) as uint8_t;
            }
            96 => {
                cyc = cyc.wrapping_add(8);
                *iz = (*iz as i32 & 0xff as i32
                    | (self.c2rust_unnamed_0.c2rust_unnamed.b as i32)
                        << 8 as i32) as uint16_t;
            }
            97 => {
                cyc = cyc.wrapping_add(8);
                *iz = (*iz as i32 & 0xff as i32
                    | (self.c2rust_unnamed_0.c2rust_unnamed.c as i32)
                        << 8 as i32) as uint16_t;
            }
            98 => {
                cyc = cyc.wrapping_add(8);
                *iz = (*iz as i32 & 0xff as i32
                    | (self.c2rust_unnamed_1.c2rust_unnamed.d as i32)
                        << 8 as i32) as uint16_t;
            }
            99 => {
                cyc = cyc.wrapping_add(8);
                *iz = (*iz as i32 & 0xff as i32
                    | (self.c2rust_unnamed_1.c2rust_unnamed.e as i32)
                        << 8 as i32) as uint16_t;
            }
            100 => {
                cyc = cyc.wrapping_add(8);
            }
            101 => {
                cyc = cyc.wrapping_add(8);
                *iz = ((*iz as i32 & 0xff as i32) << 8 as i32
                    | *iz as i32 & 0xff as i32) as uint16_t;
            }
            103 => {
                cyc = cyc.wrapping_add(8);
                *iz = (*iz as i32 & 0xff as i32
                    | (self.c2rust_unnamed.c2rust_unnamed.a as i32)
                        << 8 as i32) as uint16_t;
            }
            38 => {
                cyc = cyc.wrapping_add(11);
                *iz = (*iz as i32 & 0xff as i32
                    | (self.nextb() as i32) << 8 as i32) as uint16_t;
            }
            104 => {
                cyc = cyc.wrapping_add(8);
                *iz = ((*iz as i32 >> 8 as i32) << 8 as i32
                    | self.c2rust_unnamed_0.c2rust_unnamed.b as i32) as uint16_t;
            }
            105 => {
                cyc = cyc.wrapping_add(8);
                *iz = ((*iz as i32 >> 8 as i32) << 8 as i32
                    | self.c2rust_unnamed_0.c2rust_unnamed.c as i32) as uint16_t;
            }
            106 => {
                cyc = cyc.wrapping_add(8);
                *iz = ((*iz as i32 >> 8 as i32) << 8 as i32
                    | self.c2rust_unnamed_1.c2rust_unnamed.d as i32) as uint16_t;
            }
            107 => {
                cyc = cyc.wrapping_add(8);
                *iz = ((*iz as i32 >> 8 as i32) << 8 as i32
                    | self.c2rust_unnamed_1.c2rust_unnamed.e as i32) as uint16_t;
            }
            108 => {
                cyc = cyc.wrapping_add(8);
                *iz = ((*iz as i32 >> 8 as i32) << 8 as i32
                    | *iz as i32 >> 8 as i32) as uint16_t;
            }
            109 => {
                cyc = cyc.wrapping_add(8);
            }
            111 => {
                cyc = cyc.wrapping_add(8);
                *iz = ((*iz as i32 >> 8 as i32) << 8 as i32
                    | self.c2rust_unnamed.c2rust_unnamed.a as i32) as uint16_t;
            }
            46 => {
                cyc = cyc.wrapping_add(11);
                *iz = ((*iz as i32 >> 8 as i32) << 8 as i32
                    | self.nextb() as i32) as uint16_t;
            }
            249 => {
                cyc = cyc.wrapping_add(10);
                self.sp = *iz;
            }
            227 => {
                cyc = cyc.wrapping_add(23);
                let val: uint16_t = self.rw(self.sp);
                self.ww(self.sp, *iz);
                *iz = val;
                self.mem_ptr = val;
            }
            203 => {
                let mut addr_2: uint16_t = self.displace(*iz, self.nextb() as int8_t);
                let mut op: uint8_t = self.nextb();
                cyc = cyc.wrapping_add(self.exec_opcode_dcb(op, addr_2));
            }
            _ => {
                cyc = cyc
                    .wrapping_add(
                        (4_u32).wrapping_add(self.exec_opcode(opcode)),
                    );
                self
                    .r = (self.r as i32 & 0x80 as i32
                    | self.r as i32 - 1 as i32 & 0x7f as i32)
                    as uint8_t;
            }
        }
        return cyc;
    }
    fn exec_opcode_cb(&mut self, mut opcode: uint8_t) -> u32 {
        let mut cyc: u32 = 8;
        self.inc_r();
        let mut x_: uint8_t = (opcode as i32 >> 6 as i32 & 3 as i32)
            as uint8_t;
        let mut y_: uint8_t = (opcode as i32 >> 3 as i32 & 7 as i32)
            as uint8_t;
        let mut z_: uint8_t = (opcode as i32 & 7 as i32) as uint8_t;
        let mut hl: uint8_t = 0 as i32 as uint8_t;
        let mut reg: *mut uint8_t = 0 as *mut uint8_t;
        match z_ as i32 {
            0 => {
                reg = &mut self.c2rust_unnamed_0.c2rust_unnamed.b;
            }
            1 => {
                reg = &mut self.c2rust_unnamed_0.c2rust_unnamed.c;
            }
            2 => {
                reg = &mut self.c2rust_unnamed_1.c2rust_unnamed.d;
            }
            3 => {
                reg = &mut self.c2rust_unnamed_1.c2rust_unnamed.e;
            }
            4 => {
                reg = &mut self.c2rust_unnamed_2.c2rust_unnamed.h;
            }
            5 => {
                reg = &mut self.c2rust_unnamed_2.c2rust_unnamed.l;
            }
            6 => {
                hl = self.ctrl.read_byte(self.c2rust_unnamed_2.hl);
                reg = &mut hl;
            }
            7 => {
                reg = &mut self.c2rust_unnamed.c2rust_unnamed.a;
            }
            _ => {}
        }
        match x_ as i32 {
            0 => {
                match y_ as i32 {
                    0 => {
                        *reg = self.cb_rlc(*reg);
                    }
                    1 => {
                        *reg = self.cb_rrc(*reg);
                    }
                    2 => {
                        *reg = self.cb_rl(*reg);
                    }
                    3 => {
                        *reg = self.cb_rr(*reg);
                    }
                    4 => {
                        *reg = self.cb_sla(*reg);
                    }
                    5 => {
                        *reg = self.cb_sra(*reg);
                    }
                    6 => {
                        *reg = self.cb_sll(*reg);
                    }
                    7 => {
                        *reg = self.cb_srl(*reg);
                    }
                    _ => {}
                }
            }
            1 => {
                self.cb_bit(*reg, y_);
                if z_ as i32 == 6 as i32 {
                    self.flag_set(
                        yf,
                        self.mem_ptr as i32 >> 8 as i32 >> 5 as i32
                            & 1 as i32 != 0,
                    );
                    self.flag_set(
                        xf,
                        self.mem_ptr as i32 >> 8 as i32 >> 3 as i32
                            & 1 as i32 != 0,
                    );
                    cyc = cyc.wrapping_add(4);
                } else {
                    self.flag_set(
                        yf,
                        *reg as i32 >> 5 as i32 & 1 as i32 != 0,
                    );
                    self.flag_set(
                        xf,
                        *reg as i32 >> 3 as i32 & 1 as i32 != 0,
                    );
                }
            }
            2 => {
                *reg = (*reg as i32 & !((1 as i32) << y_ as i32))
                    as uint8_t;
            }
            3 => {
                *reg = (*reg as i32 | (1 as i32) << y_ as i32)
                    as uint8_t;
            }
            _ => {}
        }
        if x_ as i32 != 1 as i32 && z_ as i32 == 6 as i32 {
            self.ctrl.write_byte(self.c2rust_unnamed_2.hl, hl);
            cyc = cyc.wrapping_add(7);
        }
        return cyc;
    }
    fn exec_opcode_dcb(
        &mut self,
        mut opcode: uint8_t,
        mut addr: uint16_t,
    ) -> u32 {
        let mut cyc: u32 = 0;
        let mut val: uint8_t = self.ctrl.read_byte(addr);
        let mut result: uint8_t = 0 as i32 as uint8_t;
        let mut x_: uint8_t = (opcode as i32 >> 6 as i32 & 3 as i32)
            as uint8_t;
        let mut y_: uint8_t = (opcode as i32 >> 3 as i32 & 7 as i32)
            as uint8_t;
        let mut z_: uint8_t = (opcode as i32 & 7 as i32) as uint8_t;
        match x_ as i32 {
            0 => {
                match y_ as i32 {
                    0 => {
                        result = self.cb_rlc(val);
                    }
                    1 => {
                        result = self.cb_rrc(val);
                    }
                    2 => {
                        result = self.cb_rl(val);
                    }
                    3 => {
                        result = self.cb_rr(val);
                    }
                    4 => {
                        result = self.cb_sla(val);
                    }
                    5 => {
                        result = self.cb_sra(val);
                    }
                    6 => {
                        result = self.cb_sll(val);
                    }
                    7 => {
                        result = self.cb_srl(val);
                    }
                    _ => {}
                }
            }
            1 => {
                result = self.cb_bit(val, y_);
                self.flag_set(
                    yf,
                    addr as i32 >> 8 as i32 >> 5 as i32
                        & 1 as i32 != 0,
                );
                self.flag_set(
                    xf,
                    addr as i32 >> 8 as i32 >> 3 as i32
                        & 1 as i32 != 0,
                );
            }
            2 => {
                result = (val as i32 & !((1 as i32) << y_ as i32))
                    as uint8_t;
            }
            3 => {
                result = (val as i32 | (1 as i32) << y_ as i32)
                    as uint8_t;
            }
            _ => {}
        }
        if x_ as i32 != 1 as i32 && z_ as i32 != 6 as i32 {
            match z_ as i32 {
                0 => {
                    self.c2rust_unnamed_0.c2rust_unnamed.b = result;
                }
                1 => {
                    self.c2rust_unnamed_0.c2rust_unnamed.c = result;
                }
                2 => {
                    self.c2rust_unnamed_1.c2rust_unnamed.d = result;
                }
                3 => {
                    self.c2rust_unnamed_1.c2rust_unnamed.e = result;
                }
                4 => {
                    self.c2rust_unnamed_2.c2rust_unnamed.h = result;
                }
                5 => {
                    self.c2rust_unnamed_2.c2rust_unnamed.l = result;
                }
                6 => {
                    self.ctrl.write_byte(self.c2rust_unnamed_2.hl, result);
                }
                7 => {
                    self.c2rust_unnamed.c2rust_unnamed.a = result;
                }
                _ => {}
            }
        }
        if x_ as i32 == 1 as i32 {
            cyc = cyc.wrapping_add(20);
        } else {
            self.ctrl.write_byte(addr, result);
            cyc = cyc.wrapping_add(23);
        }
        return cyc;
    }
    fn exec_opcode_ed(&mut self, mut opcode: uint8_t) -> u32 {
        let mut cyc: u32 = 0;
        self.inc_r();
        match opcode as i32 {
            71 => {
                cyc = cyc.wrapping_add(9);
                self.i = self.c2rust_unnamed.c2rust_unnamed.a;
            }
            79 => {
                cyc = cyc.wrapping_add(9);
                self.r = self.c2rust_unnamed.c2rust_unnamed.a;
            }
            87 => {
                cyc = cyc.wrapping_add(9);
                self.c2rust_unnamed.c2rust_unnamed.a = self.i;
                self.flag_set(
                    sf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 >> 7 as i32
                        != 0,
                );
                self.flag_set(
                    zf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 == 0 as i32,
                );
                self.flag_set(hf, 0 as i32 != 0);
                self.flag_set(nf, 0 as i32 != 0);
                self.flag_set(pf, self.iff2);
            }
            95 => {
                cyc = cyc.wrapping_add(9);
                self.c2rust_unnamed.c2rust_unnamed.a = self.r;
                self.flag_set(
                    sf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 >> 7 as i32
                        != 0,
                );
                self.flag_set(
                    zf,
                    self.c2rust_unnamed.c2rust_unnamed.a as i32 == 0 as i32,
                );
                self.flag_set(hf, 0 as i32 != 0);
                self.flag_set(nf, 0 as i32 != 0);
                self.flag_set(pf, self.iff2);
            }
            69 | 85 | 93 | 101 | 109 | 117 | 125 => {
                cyc = cyc.wrapping_add(14);
                self.iff1 = self.iff2;
                self.ret();
            }
            77 => {
                cyc = cyc.wrapping_add(14);
                self.ret();
            }
            160 => {
                cyc = cyc.wrapping_add(16);
                self.ldi();
            }
            176 => {
                cyc = cyc.wrapping_add(16);
                self.ldi();
                if self.c2rust_unnamed_0.bc as i32 != 0 as i32 {
                    self.pc = (self.pc as i32 - 2 as i32) as uint16_t;
                    cyc = cyc.wrapping_add(5);
                    self.mem_ptr = (self.pc as i32 + 1 as i32) as uint16_t;
                }
            }
            168 => {
                cyc = cyc.wrapping_add(16);
                self.ldd();
            }
            184 => {
                cyc = cyc.wrapping_add(16);
                self.ldd();
                if self.c2rust_unnamed_0.bc as i32 != 0 as i32 {
                    self.pc = (self.pc as i32 - 2 as i32) as uint16_t;
                    cyc = cyc.wrapping_add(5);
                    self.mem_ptr = (self.pc as i32 + 1 as i32) as uint16_t;
                }
            }
            161 => {
                cyc = cyc.wrapping_add(16);
                self.cpi();
            }
            169 => {
                cyc = cyc.wrapping_add(16);
                self.cpd();
            }
            177 => {
                cyc = cyc.wrapping_add(16);
                self.cpi();
                if self.c2rust_unnamed_0.bc as i32 != 0 as i32
                    && !self.flag_get(zf)
                {
                    self.pc = (self.pc as i32 - 2 as i32) as uint16_t;
                    cyc = cyc.wrapping_add(5);
                    self.mem_ptr = (self.pc as i32 + 1 as i32) as uint16_t;
                } else {
                    self
                        .mem_ptr = (self.mem_ptr as i32 + 1 as i32)
                        as uint16_t;
                }
            }
            185 => {
                cyc = cyc.wrapping_add(16);
                self.cpd();
                if self.c2rust_unnamed_0.bc as i32 != 0 as i32
                    && !self.flag_get(zf)
                {
                    self.pc = (self.pc as i32 - 2 as i32) as uint16_t;
                    cyc = cyc.wrapping_add(5);
                } else {
                    self
                        .mem_ptr = (self.mem_ptr as i32 + 1 as i32)
                        as uint16_t;
                }
            }
            64 => {
                cyc = cyc.wrapping_add(12);
                self.in_r_c(&mut self.c2rust_unnamed_0.c2rust_unnamed.b);
            }
            72 => {
                cyc = cyc.wrapping_add(12);
                self.in_r_c(&mut self.c2rust_unnamed_0.c2rust_unnamed.c);
            }
            80 => {
                cyc = cyc.wrapping_add(12);
                self.in_r_c(&mut self.c2rust_unnamed_1.c2rust_unnamed.d);
            }
            88 => {
                cyc = cyc.wrapping_add(12);
                self.in_r_c(&mut self.c2rust_unnamed_1.c2rust_unnamed.e);
            }
            96 => {
                cyc = cyc.wrapping_add(12);
                self.in_r_c(&mut self.c2rust_unnamed_2.c2rust_unnamed.h);
            }
            104 => {
                cyc = cyc.wrapping_add(12);
                self.in_r_c(&mut self.c2rust_unnamed_2.c2rust_unnamed.l);
            }
            112 => {
                cyc = cyc.wrapping_add(12);
                let mut val: uint8_t = 0;
                self.in_r_c(&mut val);
            }
            120 => {
                cyc = cyc.wrapping_add(12);
                self.in_r_c(&mut self.c2rust_unnamed.c2rust_unnamed.a);
                self
                    .mem_ptr = (self.c2rust_unnamed_0.bc as i32 + 1 as i32)
                    as uint16_t;
            }
            162 => {
                cyc = cyc.wrapping_add(16);
                self.ini();
            }
            178 => {
                cyc = cyc.wrapping_add(16);
                self.ini();
                if self.c2rust_unnamed_0.c2rust_unnamed.b as i32 > 0 as i32 {
                    self.pc = (self.pc as i32 - 2 as i32) as uint16_t;
                    cyc = cyc.wrapping_add(5);
                    self.mem_ptr = (self.pc as i32 + 1 as i32) as uint16_t;
                }
            }
            170 => {
                cyc = cyc.wrapping_add(16);
                self.ind();
            }
            186 => {
                cyc = cyc.wrapping_add(16);
                self.ind();
                if self.c2rust_unnamed_0.c2rust_unnamed.b as i32 > 0 as i32 {
                    self.pc = (self.pc as i32 - 2 as i32) as uint16_t;
                    cyc = cyc.wrapping_add(5);
                    self.mem_ptr = (self.pc as i32 + 1 as i32) as uint16_t;
                }
            }
            121 => {
                cyc = cyc.wrapping_add(12);
                self.outc(self.c2rust_unnamed.c2rust_unnamed.a);
            }
            65 => {
                cyc = cyc.wrapping_add(12);
                self.outc(self.c2rust_unnamed_0.c2rust_unnamed.b);
            }
            73 => {
                cyc = cyc.wrapping_add(12);
                self.outc(self.c2rust_unnamed_0.c2rust_unnamed.c);
            }
            81 => {
                cyc = cyc.wrapping_add(12);
                self.outc(self.c2rust_unnamed_1.c2rust_unnamed.d);
            }
            89 => {
                cyc = cyc.wrapping_add(12);
                self.outc(self.c2rust_unnamed_1.c2rust_unnamed.e);
            }
            97 => {
                cyc = cyc.wrapping_add(12);
                self.outc(self.c2rust_unnamed_2.c2rust_unnamed.h);
            }
            105 => {
                cyc = cyc.wrapping_add(12);
                self.outc(self.c2rust_unnamed_2.c2rust_unnamed.l);
            }
            113 => {
                cyc = cyc.wrapping_add(12);
                self.outc(0 as i32 as uint8_t);
            }
            163 => {
                cyc = cyc.wrapping_add(16);
                self.outi();
            }
            179 => {
                cyc = cyc.wrapping_add(16);
                self.outi();
                if self.c2rust_unnamed_0.c2rust_unnamed.b as i32 > 0 as i32 {
                    self.pc = (self.pc as i32 - 2 as i32) as uint16_t;
                    cyc = cyc.wrapping_add(5);
                    self.mem_ptr = (self.pc as i32 + 1 as i32) as uint16_t;
                }
            }
            171 => {
                cyc = cyc.wrapping_add(16);
                self.outd();
            }
            187 => {
                cyc = cyc.wrapping_add(16);
                self.outd();
                if self.c2rust_unnamed_0.c2rust_unnamed.b as i32 > 0 as i32 {
                    self.pc = (self.pc as i32 - 2 as i32) as uint16_t;
                    cyc = cyc.wrapping_add(5);
                    self.mem_ptr = (self.pc as i32 + 1 as i32) as uint16_t;
                }
            }
            66 => {
                cyc = cyc.wrapping_add(15);
                self.sbchl(self.c2rust_unnamed_0.bc);
            }
            82 => {
                cyc = cyc.wrapping_add(15);
                self.sbchl(self.c2rust_unnamed_1.de);
            }
            98 => {
                cyc = cyc.wrapping_add(15);
                self.sbchl(self.c2rust_unnamed_2.hl);
            }
            114 => {
                cyc = cyc.wrapping_add(15);
                self.sbchl(self.sp);
            }
            74 => {
                cyc = cyc.wrapping_add(15);
                self.adchl(self.c2rust_unnamed_0.bc);
            }
            90 => {
                cyc = cyc.wrapping_add(15);
                self.adchl(self.c2rust_unnamed_1.de);
            }
            106 => {
                cyc = cyc.wrapping_add(15);
                self.adchl(self.c2rust_unnamed_2.hl);
            }
            122 => {
                cyc = cyc.wrapping_add(15);
                self.adchl(self.sp);
            }
            67 => {
                cyc = cyc.wrapping_add(20);
                let addr: uint16_t = self.nextw();
                self.ww(addr, self.c2rust_unnamed_0.bc);
                self.mem_ptr = (addr as i32 + 1 as i32) as uint16_t;
            }
            83 => {
                cyc = cyc.wrapping_add(20);
                let addr_0: uint16_t = self.nextw();
                self.ww(addr_0, self.c2rust_unnamed_1.de);
                self.mem_ptr = (addr_0 as i32 + 1 as i32) as uint16_t;
            }
            99 => {
                cyc = cyc.wrapping_add(20);
                let addr_1: uint16_t = self.nextw();
                self.ww(addr_1, self.c2rust_unnamed_2.hl);
                self.mem_ptr = (addr_1 as i32 + 1 as i32) as uint16_t;
            }
            115 => {
                cyc = cyc.wrapping_add(20);
                let addr_2: uint16_t = self.nextw();
                self.ww(addr_2, self.sp);
                self.mem_ptr = (addr_2 as i32 + 1 as i32) as uint16_t;
            }
            75 => {
                cyc = cyc.wrapping_add(20);
                let addr_3: uint16_t = self.nextw();
                self.c2rust_unnamed_0.bc = self.rw(addr_3);
                self.mem_ptr = (addr_3 as i32 + 1 as i32) as uint16_t;
            }
            91 => {
                cyc = cyc.wrapping_add(20);
                let addr_4: uint16_t = self.nextw();
                self.c2rust_unnamed_1.de = self.rw(addr_4);
                self.mem_ptr = (addr_4 as i32 + 1 as i32) as uint16_t;
            }
            107 => {
                cyc = cyc.wrapping_add(20);
                let addr_5: uint16_t = self.nextw();
                self.c2rust_unnamed_2.hl = self.rw(addr_5);
                self.mem_ptr = (addr_5 as i32 + 1 as i32) as uint16_t;
            }
            123 => {
                cyc = cyc.wrapping_add(20);
                let addr_6: uint16_t = self.nextw();
                self.sp = self.rw(addr_6);
                self.mem_ptr = (addr_6 as i32 + 1 as i32) as uint16_t;
            }
            68 | 84 | 100 | 116 | 76 | 92 | 108 | 124 => {
                cyc = cyc.wrapping_add(8);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = self.subb(
                    0 as i32 as uint32_t,
                    self.c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                    0 as i32 != 0,
                );
            }
            70 | 102 => {
                cyc = cyc.wrapping_add(8);
                self.interrupt_mode = 0 as i32 as uint8_t;
            }
            86 | 118 => {
                cyc = cyc.wrapping_add(8);
                self.interrupt_mode = 1 as i32 as uint8_t;
            }
            94 | 126 => {
                cyc = cyc.wrapping_add(8);
                self.interrupt_mode = 2 as i32 as uint8_t;
            }
            103 => {
                cyc = cyc.wrapping_add(18);
                let mut a: uint8_t = self.c2rust_unnamed.c2rust_unnamed.a;
                let mut val_0: uint8_t = self.ctrl.read_byte(self.c2rust_unnamed_2.hl);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = (a as i32 & 0xf0 as i32
                    | val_0 as i32 & 0xf as i32) as uint8_t;
                self.ctrl.write_byte(
                    self.c2rust_unnamed_2.hl,
                    (val_0 as i32 >> 4 as i32
                        | (a as i32) << 4 as i32) as uint8_t,
                );
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .f = (self.f_szpxy[self.c2rust_unnamed.c2rust_unnamed.a as usize]
                    as i32 | Self::flag_val(cf, self.flag_get(cf)) as i32
                    | Self::flag_val(nf, 0 as i32 != 0) as i32
                    | Self::flag_val(hf, 0 as i32 != 0) as i32) as uint8_t;
                self
                    .mem_ptr = (self.c2rust_unnamed_2.hl as i32 + 1 as i32)
                    as uint16_t;
            }
            111 => {
                cyc = cyc.wrapping_add(18);
                let mut a_0: uint8_t = self.c2rust_unnamed.c2rust_unnamed.a;
                let mut val_1: uint8_t = self.ctrl.read_byte(self.c2rust_unnamed_2.hl);
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .a = (a_0 as i32 & 0xf0 as i32
                    | val_1 as i32 >> 4 as i32) as uint8_t;
                self.ctrl.write_byte(
                    self.c2rust_unnamed_2.hl,
                    ((val_1 as i32) << 4 as i32
                        | a_0 as i32 & 0xf as i32) as uint8_t,
                );
                self
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .f = (self.f_szpxy[self.c2rust_unnamed.c2rust_unnamed.a as usize]
                    as i32 | Self::flag_val(cf, self.flag_get(cf)) as i32
                    | Self::flag_val(nf, 0 as i32 != 0) as i32
                    | Self::flag_val(hf, 0 as i32 != 0) as i32) as uint8_t;
                self
                    .mem_ptr = (self.c2rust_unnamed_2.hl as i32 + 1 as i32)
                    as uint16_t;
            }
            _ => {}
        }
        return cyc;
    }
}

#[derive(Copy, Clone)]
pub union C2RustUnnamed_0 {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub h_l_: uint16_t,
}
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub l_: uint8_t,
    pub h_: uint8_t,
}
#[derive(Copy, Clone)]
pub union C2RustUnnamed_2 {
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub d_e_: uint16_t,
}
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub e_: uint8_t,
    pub d_: uint8_t,
}
#[derive(Copy, Clone)]
pub union C2RustUnnamed_4 {
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub b_c_: uint16_t,
}
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_5 {
    pub c_: uint8_t,
    pub b_: uint8_t,
}
#[derive(Copy, Clone)]
pub union C2RustUnnamed_6 {
    pub c2rust_unnamed: C2RustUnnamed_7,
    pub a_f_: uint16_t,
}
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_7 {
    pub f_: uint8_t,
    pub a_: uint8_t,
}
#[derive(Copy, Clone)]
pub union C2RustUnnamed_8 {
    pub c2rust_unnamed: C2RustUnnamed_9,
    pub hl: uint16_t,
}
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_9 {
    pub l: uint8_t,
    pub h: uint8_t,
}
#[derive(Copy, Clone)]
pub union C2RustUnnamed_10 {
    pub c2rust_unnamed: C2RustUnnamed_11,
    pub de: uint16_t,
}
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_11 {
    pub e: uint8_t,
    pub d: uint8_t,
}
#[derive(Copy, Clone)]
pub union C2RustUnnamed_12 {
    pub c2rust_unnamed: C2RustUnnamed_13,
    pub bc: uint16_t,
}
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_13 {
    pub c: uint8_t,
    pub b: uint8_t,
}
#[derive(Copy, Clone)]
pub union C2RustUnnamed_14 {
    pub c2rust_unnamed: C2RustUnnamed_15,
    pub af: uint16_t,
}
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_15 {
    pub f: uint8_t,
    pub a: uint8_t,
}
