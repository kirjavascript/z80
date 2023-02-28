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

type int8_t = i8;
type int32_t = i32;
type uint8_t = u8;
type uint16_t = u16;
type uint32_t = u32;
const ASSERT: u32 = 2;
const PULSE: u32 = 1;

pub trait Z80Ctrl {
    fn read_byte(&self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, value: u8);
    fn port_in(&self, addr: u16) -> u8;
    fn port_out(&mut self, addr: u16, value: u8);
}

// pub struct Z80 <T: z80Ctrl>{
//     pub ctrl: T,

pub struct Z80 {
    pub ctrl: Box<dyn Z80Ctrl>,
    pub pc: uint16_t,
    pub sp: uint16_t,
    pub ix: uint16_t,
    pub iy: uint16_t,
    pub mem_ptr: uint16_t,
    c2rust_unnamed: C2RustUnnamed_14,
    c2rust_unnamed_0: C2RustUnnamed_12,
    c2rust_unnamed_1: C2RustUnnamed_10,
    c2rust_unnamed_2: C2RustUnnamed_8,
    c2rust_unnamed_3: C2RustUnnamed_6,
    c2rust_unnamed_4: C2RustUnnamed_4,
    c2rust_unnamed_5: C2RustUnnamed_2,
    c2rust_unnamed_6: C2RustUnnamed_0,
    pub i: uint8_t,
    pub r: uint8_t,
    pub iff_delay: uint8_t,
    pub interrupt_mode: uint8_t,
    pub irq_data: uint8_t,
    pub irq_pending: uint8_t,
    pub nmi_pending: uint8_t,
    pub halted: bool,
    pub iff1: bool,
    pub iff2: bool,
    #[cfg(test)]
    pub test_finished: bool,
}

impl Z80 {
    pub fn new(ctrl: Box<dyn Z80Ctrl>) -> Self {
        Z80 {
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
        unsafe { step_s(self) }
    }
    fn internal_port_in(&self, addr: u16) -> u8 {
        #[cfg(test)]
        unsafe {
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
}

#[derive(Copy, Clone)]
#[repr(C)]
union C2RustUnnamed_0 {
    c2rust_unnamed: C2RustUnnamed_1,
    h_l_: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct C2RustUnnamed_1 {
    l_: uint8_t,
    h_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
union C2RustUnnamed_2 {
    c2rust_unnamed: C2RustUnnamed_3,
    d_e_: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct C2RustUnnamed_3 {
    e_: uint8_t,
    d_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
union C2RustUnnamed_4 {
    c2rust_unnamed: C2RustUnnamed_5,
    b_c_: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct C2RustUnnamed_5 {
    c_: uint8_t,
    b_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
union C2RustUnnamed_6 {
    c2rust_unnamed: C2RustUnnamed_7,
    a_f_: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct C2RustUnnamed_7 {
    f_: uint8_t,
    a_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
union C2RustUnnamed_8 {
    c2rust_unnamed: C2RustUnnamed_9,
    hl: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct C2RustUnnamed_9 {
    l: uint8_t,
    h: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
union C2RustUnnamed_10 {
    c2rust_unnamed: C2RustUnnamed_11,
    de: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct C2RustUnnamed_11 {
    e: uint8_t,
    d: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
union C2RustUnnamed_12 {
    c2rust_unnamed: C2RustUnnamed_13,
    bc: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct C2RustUnnamed_13 {
    c: uint8_t,
    b: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
union C2RustUnnamed_14 {
    c2rust_unnamed: C2RustUnnamed_15,
    af: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct C2RustUnnamed_15 {
    f: uint8_t,
    a: uint8_t,
}
type Flagbit = u32;
const sf: Flagbit = 7;
const zf: Flagbit = 6;
const yf: Flagbit = 5;
const hf: Flagbit = 4;
const xf: Flagbit = 3;
const pf: Flagbit = 2;
const nf: Flagbit = 1;
const cf: Flagbit = 0;
static mut f_szpxy: [uint8_t; 256] = [
    0x44, 0, 0, 0x4, 0, 0x4, 0x4, 0, 0x8, 0xc, 0xc, 0x8, 0xc, 0x8, 0x8, 0xc, 0, 0x4, 0x4, 0, 0x4, 0, 0, 0x4, 0xc, 0x8, 0x8, 0xc, 0x8, 0xc, 0xc, 0x8, 0x20, 0x24, 0x24, 0x20, 0x24, 0x20, 0x20, 0x24, 0x2c, 0x28, 0x28, 0x2c, 0x28, 0x2c, 0x2c, 0x28, 0x24, 0x20, 0x20, 0x24, 0x20, 0x24, 0x24, 0x20, 0x28, 0x2c, 0x2c, 0x28, 0x2c, 0x28, 0x28, 0x2c, 0, 0x4, 0x4, 0, 0x4, 0, 0, 0x4, 0xc, 0x8, 0x8, 0xc, 0x8, 0xc, 0xc, 0x8, 0x4, 0, 0, 0x4, 0, 0x4, 0x4, 0, 0x8, 0xc, 0xc, 0x8, 0xc, 0x8, 0x8, 0xc, 0x24, 0x20, 0x20, 0x24, 0x20, 0x24, 0x24, 0x20, 0x28, 0x2c, 0x2c, 0x28, 0x2c, 0x28, 0x28, 0x2c, 0x20, 0x24, 0x24, 0x20, 0x24, 0x20, 0x20, 0x24, 0x2c, 0x28, 0x28, 0x2c, 0x28, 0x2c, 0x2c, 0x28, 0x80, 0x84, 0x84, 0x80, 0x84, 0x80, 0x80, 0x84, 0x8c, 0x88, 0x88, 0x8c, 0x88, 0x8c, 0x8c, 0x88, 0x84, 0x80, 0x80, 0x84, 0x80, 0x84, 0x84, 0x80, 0x88, 0x8c, 0x8c, 0x88, 0x8c, 0x88, 0x88, 0x8c, 0xa4, 0xa0, 0xa0, 0xa4, 0xa0, 0xa4, 0xa4, 0xa0, 0xa8, 0xac, 0xac, 0xa8, 0xac, 0xa8, 0xa8, 0xac, 0xa0, 0xa4, 0xa4, 0xa0, 0xa4, 0xa0, 0xa0, 0xa4, 0xac, 0xa8, 0xa8, 0xac, 0xa8, 0xac, 0xac, 0xa8, 0x84, 0x80, 0x80, 0x84, 0x80, 0x84, 0x84, 0x80, 0x88, 0x8c, 0x8c, 0x88, 0x8c, 0x88, 0x88, 0x8c, 0x80, 0x84, 0x84, 0x80, 0x84, 0x80, 0x80, 0x84, 0x8c, 0x88, 0x88, 0x8c, 0x88, 0x8c, 0x8c, 0x88, 0xa0, 0xa4, 0xa4, 0xa0, 0xa4, 0xa0, 0xa0, 0xa4, 0xac, 0xa8, 0xa8, 0xac, 0xa8, 0xac, 0xac, 0xa8, 0xa4, 0xa0, 0xa0, 0xa4, 0xa0, 0xa4, 0xa4, 0xa0, 0xa8, 0xac, 0xac, 0xa8, 0xac, 0xa8, 0xa8, 0xac,
];
#[inline]
unsafe fn flag_val(mut bit: Flagbit, mut cond: bool) -> uint8_t {
    return ((cond as i32) << bit as u32) as uint8_t;
}
#[inline]
unsafe fn flag_get(z: *mut Z80, mut bit: Flagbit) -> bool {
    return (*z).c2rust_unnamed.c2rust_unnamed.f as i32
        & (1 as i32) << bit as u32 != 0;
}
#[inline]
unsafe fn flag_set(z: *mut Z80, mut bit: Flagbit, mut val: bool) {
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as i32
        & !((1 as i32) << bit as u32)) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as i32
        | (val as i32) << bit as u32) as uint8_t;
}
#[inline]
unsafe fn rb(z: *mut Z80, mut addr: uint16_t) -> uint8_t {
    return (*z).ctrl.read_byte(addr);
}
#[inline]
unsafe fn wb(z: *mut Z80, mut addr: uint16_t, mut val: uint8_t) {
    (*z).ctrl.write_byte(addr, val);
}
#[inline]
unsafe fn rw(z: *mut Z80, mut addr: uint16_t) -> uint16_t {
    return (
        ((*z).ctrl.read_byte((addr as i32 + 1 as i32) as uint16_t) as i32) << 8 as i32
        | (*z).ctrl.read_byte(addr) as i32
    ) as uint16_t;
}
#[inline]
unsafe fn ww(z: *mut Z80, mut addr: uint16_t, mut val: uint16_t) {
    (*z).ctrl.write_byte(addr, (val as i32 & 0xff as i32) as uint8_t);
    (*z).ctrl.write_byte(
        (addr as i32 + 1 as i32) as uint16_t,
        (val as i32 >> 8 as i32) as uint8_t,
    );
}
#[inline]
unsafe fn pushw(z: *mut Z80, mut val: uint16_t) {
    (*z).sp = ((*z).sp as i32 - 2 as i32) as uint16_t;
    ww(z, (*z).sp, val);
}
#[inline]
unsafe fn popw(z: *mut Z80) -> uint16_t {
    (*z).sp = ((*z).sp as i32 + 2 as i32) as uint16_t;
    return rw(z, ((*z).sp as i32 - 2 as i32) as uint16_t);
}
#[inline]
unsafe fn nextb(z: *mut Z80) -> uint8_t {
    let fresh0 = (*z).pc;
    (*z).pc = ((*z).pc).wrapping_add(1);
    return rb(z, fresh0);
}
#[inline]
unsafe fn nextw(z: *mut Z80) -> uint16_t {
    (*z).pc = ((*z).pc as i32 + 2 as i32) as uint16_t;
    return rw(z, ((*z).pc as i32 - 2 as i32) as uint16_t);
}
#[inline]
unsafe fn inc_r(z: *mut Z80) {
    (*z)
        .r = ((*z).r as i32 & 0x80 as i32
        | (*z).r as i32 + 1 as i32 & 0x7f as i32) as uint8_t;
}
#[inline]
unsafe fn parity(mut v: uint8_t) -> bool {
    v = (v as i32 ^ v as i32 >> 4 as i32) as uint8_t;
    v = (v as i32 & 0xf as i32) as uint8_t;
    return 0x6996 as i32 >> v as i32 & 1 as i32 == 0;
}
#[inline]
unsafe fn jump(z: *mut Z80, mut addr: uint16_t) {
    (*z).pc = addr;
    (*z).mem_ptr = addr;
}
#[inline]
unsafe fn cond_jump(z: *mut Z80, mut condition: bool) {
    let addr: uint16_t = nextw(z);
    if condition {
        jump(z, addr);
    }
    (*z).mem_ptr = addr;
}
#[inline]
unsafe fn call(z: *mut Z80, mut addr: uint16_t) {
    pushw(z, (*z).pc);
    (*z).pc = addr;
    (*z).mem_ptr = addr;
}
#[inline]
unsafe fn cond_call(z: *mut Z80, mut condition: bool) -> u32 {
    let addr: uint16_t = nextw(z);
    let mut cyc: u32 = 0;
    if condition {
        call(z, addr);
        cyc = 7;
    }
    (*z).mem_ptr = addr;
    return cyc;
}
#[inline]
unsafe fn ret(z: *mut Z80) {
    (*z).pc = popw(z);
    (*z).mem_ptr = (*z).pc;
}
#[inline]
unsafe fn cond_ret(z: *mut Z80, mut condition: bool) -> u32 {
    if condition {
        ret(z);
        return 6;
    }
    return 0;
}
#[inline]
unsafe fn jr(z: *mut Z80, mut displacement: int8_t) {
    (*z).pc = ((*z).pc as i32 + displacement as i32) as uint16_t;
    (*z).mem_ptr = (*z).pc;
}
#[inline]
unsafe fn cond_jr(z: *mut Z80, mut condition: bool) -> u32 {
    let b: int8_t = nextb(z) as int8_t;
    if condition {
        jr(z, b);
        return 5;
    }
    return 0;
}
#[inline]
unsafe fn addb(
    z: *mut Z80,
    mut a: uint32_t,
    mut b: uint32_t,
    mut cy: bool,
) -> uint8_t {
    let mut result: int32_t = a.wrapping_add(b).wrapping_add(cy as u32)
        as int32_t;
    let mut carry: int32_t = (result as u32 ^ a ^ b) as int32_t;
    result &= 0xff as i32;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[result as usize] as i32
        & !((1 as i32) << pf as i32)) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as i32
        | carry & (1 as i32) << hf as i32) as uint8_t;
    carry >>= 6 as i32;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as i32
        | carry + 2 as i32 & 4 as i32) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as i32
        | carry >> 2 as i32) as uint8_t;
    return result as uint8_t;
}
#[inline]
unsafe fn subb(
    z: *mut Z80,
    mut a: uint32_t,
    mut b: uint32_t,
    mut cy: bool,
) -> uint8_t {
    let mut result: int32_t = a.wrapping_sub(b).wrapping_sub(cy as u32)
        as int32_t;
    let mut carry: int32_t = (result as u32 ^ a ^ b) as int32_t;
    result &= 0xff as i32;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((1 as i32) << nf as i32
        | f_szpxy[result as usize] as i32
            & !((1 as i32) << pf as i32)) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as i32
        | carry & (1 as i32) << hf as i32) as uint8_t;
    carry >>= 6 as i32;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as i32
        | carry + 2 as i32 & 4 as i32) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as i32
        | carry >> 2 as i32 & 1 as i32) as uint8_t;
    return result as uint8_t;
}
#[inline]
unsafe fn addw(
    z: *mut Z80,
    mut a: uint16_t,
    mut b: uint16_t,
    mut cy: bool,
) -> uint16_t {
    let mut lsb: uint8_t = addb(z, a as uint32_t, b as uint32_t, cy);
    let mut msb: uint8_t = addb(
        z,
        (a as i32 >> 8 as i32) as uint32_t,
        (b as i32 >> 8 as i32) as uint32_t,
        flag_get(z, cf),
    );
    let mut result: uint16_t = ((msb as i32) << 8 as i32
        | lsb as i32) as uint16_t;
    flag_set(z, zf, result as i32 == 0 as i32);
    (*z).mem_ptr = (a as i32 + 1 as i32) as uint16_t;
    return result;
}
#[inline]
unsafe fn subw(
    z: *mut Z80,
    mut a: uint16_t,
    mut b: uint16_t,
    mut cy: bool,
) -> uint16_t {
    let mut lsb: uint8_t = subb(z, a as uint32_t, b as uint32_t, cy);
    let mut msb: uint8_t = subb(
        z,
        (a as i32 >> 8 as i32) as uint32_t,
        (b as i32 >> 8 as i32) as uint32_t,
        flag_get(z, cf),
    );
    let mut result: uint16_t = ((msb as i32) << 8 as i32
        | lsb as i32) as uint16_t;
    flag_set(z, zf, result as i32 == 0 as i32);
    (*z).mem_ptr = (a as i32 + 1 as i32) as uint16_t;
    return result;
}
#[inline]
unsafe fn addhl(z: *mut Z80, mut val: uint16_t) {
    let mut sfc: bool = flag_get(z, sf);
    let mut zfc: bool = flag_get(z, zf);
    let mut pfc: bool = flag_get(z, pf);
    let mut result: uint16_t = addw(
        z,
        (*z).c2rust_unnamed_2.hl,
        val,
        0 as i32 != 0,
    );
    (*z).c2rust_unnamed_2.hl = result;
    flag_set(z, sf, sfc);
    flag_set(z, zf, zfc);
    flag_set(z, pf, pfc);
}
#[inline]
unsafe fn addiz(z: *mut Z80, mut reg: *mut uint16_t, mut val: uint16_t) {
    let mut sfc: bool = flag_get(z, sf);
    let mut zfc: bool = flag_get(z, zf);
    let mut pfc: bool = flag_get(z, pf);
    let mut result: uint16_t = addw(z, *reg, val, 0 as i32 != 0);
    *reg = result;
    flag_set(z, sf, sfc);
    flag_set(z, zf, zfc);
    flag_set(z, pf, pfc);
}
#[inline]
unsafe fn adchl(z: *mut Z80, mut val: uint16_t) {
    let mut result: uint16_t = addw(z, (*z).c2rust_unnamed_2.hl, val, flag_get(z, cf));
    flag_set(z, sf, result as i32 >> 15 as i32 != 0);
    flag_set(z, zf, result as i32 == 0 as i32);
    (*z).c2rust_unnamed_2.hl = result;
}
#[inline]
unsafe fn sbchl(z: *mut Z80, mut val: uint16_t) {
    let result: uint16_t = subw(z, (*z).c2rust_unnamed_2.hl, val, flag_get(z, cf));
    flag_set(z, sf, result as i32 >> 15 as i32 != 0);
    flag_set(z, zf, result as i32 == 0 as i32);
    (*z).c2rust_unnamed_2.hl = result;
}
#[inline]
unsafe fn inc(z: *mut Z80, mut a: uint8_t) -> uint8_t {
    let mut cfc: bool = flag_get(z, cf);
    let mut result: uint8_t = addb(
        z,
        a as uint32_t,
        1 as i32 as uint32_t,
        0 as i32 != 0,
    );
    flag_set(z, cf, cfc);
    return result;
}
#[inline]
unsafe fn dec(z: *mut Z80, mut a: uint8_t) -> uint8_t {
    let mut cfc: bool = flag_get(z, cf);
    let mut result: uint8_t = subb(
        z,
        a as uint32_t,
        1 as i32 as uint32_t,
        0 as i32 != 0,
    );
    flag_set(z, cf, cfc);
    return result;
}
#[inline]
unsafe fn land(z: *mut Z80, mut val: uint8_t) {
    let result: uint8_t = ((*z).c2rust_unnamed.c2rust_unnamed.a as i32
        & val as i32) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[result as usize] as i32
        | flag_val(hf, 1 as i32 != 0) as i32
        | flag_val(nf, 0 as i32 != 0) as i32
        | flag_val(cf, 0 as i32 != 0) as i32) as uint8_t;
    (*z).c2rust_unnamed.c2rust_unnamed.a = result;
}
#[inline]
unsafe fn lxor(z: *mut Z80, val: uint8_t) {
    let result: uint8_t = ((*z).c2rust_unnamed.c2rust_unnamed.a as i32
        ^ val as i32) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[result as usize] as i32
        | flag_val(hf, 0 as i32 != 0) as i32
        | flag_val(nf, 0 as i32 != 0) as i32
        | flag_val(cf, 0 as i32 != 0) as i32) as uint8_t;
    (*z).c2rust_unnamed.c2rust_unnamed.a = result;
}
#[inline]
unsafe fn lor(z: *mut Z80, val: uint8_t) {
    let result: uint8_t = ((*z).c2rust_unnamed.c2rust_unnamed.a as i32
        | val as i32) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[result as usize] as i32
        | flag_val(hf, 0 as i32 != 0) as i32
        | flag_val(nf, 0 as i32 != 0) as i32
        | flag_val(cf, 0 as i32 != 0) as i32) as uint8_t;
    (*z).c2rust_unnamed.c2rust_unnamed.a = result;
}
#[inline]
unsafe fn cp(z: *mut Z80, val: uint32_t) {
    let mut result: int32_t = ((*z).c2rust_unnamed.c2rust_unnamed.a as u32)
        .wrapping_sub(val) as int32_t;
    let mut carry: int32_t = ((result
        ^ (*z).c2rust_unnamed.c2rust_unnamed.a as i32) as u32 ^ val)
        as int32_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (((1 as i32) << nf as i32) as u32
        | val
            & ((1 as i32) << xf as i32
                | (1 as i32) << yf as i32) as u32
        | (result & (1 as i32) << sf as i32) as u32
        | (((result & 0xff as i32 == 0) as i32) << zf as i32)
            as u32) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as i32
        | carry & (1 as i32) << hf as i32) as uint8_t;
    carry >>= 6 as i32;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as i32
        | carry + 2 as i32 & 4 as i32) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as i32
        | carry >> 2 as i32 & 1 as i32) as uint8_t;
}
#[inline]
unsafe fn cb_rlc(z: *mut Z80, mut val: uint8_t) -> uint8_t {
    let old: bool = val as i32 >> 7 as i32 != 0;
    val = ((val as i32) << 1 as i32 | old as i32) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[val as usize] as i32
        | flag_val(nf, 0 as i32 != 0) as i32
        | flag_val(hf, 0 as i32 != 0) as i32
        | flag_val(cf, old) as i32) as uint8_t;
    return val;
}
#[inline]
unsafe fn cb_rrc(z: *mut Z80, mut val: uint8_t) -> uint8_t {
    let old: bool = val as i32 & 1 as i32 != 0;
    val = (val as i32 >> 1 as i32
        | (old as i32) << 7 as i32) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[val as usize] as i32
        | flag_val(nf, 0 as i32 != 0) as i32
        | flag_val(hf, 0 as i32 != 0) as i32
        | flag_val(cf, old) as i32) as uint8_t;
    return val;
}
#[inline]
unsafe fn cb_rl(z: *mut Z80, mut val: uint8_t) -> uint8_t {
    let cfc: bool = flag_get(z, cf);
    let cfn: bool = val as i32 >> 7 as i32 != 0;
    val = ((val as i32) << 1 as i32 | cfc as i32) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[val as usize] as i32 | flag_val(cf, cfn) as i32
        | flag_val(nf, 0 as i32 != 0) as i32
        | flag_val(hf, 0 as i32 != 0) as i32) as uint8_t;
    return val;
}
#[inline]
unsafe fn cb_rr(z: *mut Z80, mut val: uint8_t) -> uint8_t {
    let c: bool = flag_get(z, cf);
    let cfn: bool = val as i32 & 1 as i32 != 0;
    val = (val as i32 >> 1 as i32
        | (c as i32) << 7 as i32) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[val as usize] as i32 | flag_val(cf, cfn) as i32
        | flag_val(nf, 0 as i32 != 0) as i32
        | flag_val(hf, 0 as i32 != 0) as i32) as uint8_t;
    return val;
}
#[inline]
unsafe fn cb_sla(z: *mut Z80, mut val: uint8_t) -> uint8_t {
    let cfn: bool = val as i32 >> 7 as i32 != 0;
    val = ((val as i32) << 1 as i32) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[val as usize] as i32 | flag_val(cf, cfn) as i32
        | flag_val(nf, 0 as i32 != 0) as i32
        | flag_val(hf, 0 as i32 != 0) as i32) as uint8_t;
    return val;
}
#[inline]
unsafe fn cb_sll(z: *mut Z80, mut val: uint8_t) -> uint8_t {
    let cfn: bool = val as i32 >> 7 as i32 != 0;
    val = ((val as i32) << 1 as i32) as uint8_t;
    val = (val as i32 | 1 as i32) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[val as usize] as i32 | flag_val(cf, cfn) as i32
        | flag_val(nf, 0 as i32 != 0) as i32
        | flag_val(hf, 0 as i32 != 0) as i32) as uint8_t;
    return val;
}
#[inline]
unsafe fn cb_sra(z: *mut Z80, mut val: uint8_t) -> uint8_t {
    let cfn: bool = val as i32 & 1 as i32 != 0;
    val = (val as i32 >> 1 as i32
        | val as i32 & 0x80 as i32) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[val as usize] as i32 | flag_val(cf, cfn) as i32
        | flag_val(nf, 0 as i32 != 0) as i32
        | flag_val(hf, 0 as i32 != 0) as i32) as uint8_t;
    return val;
}
#[inline]
unsafe fn cb_srl(z: *mut Z80, mut val: uint8_t) -> uint8_t {
    let cfn: bool = val as i32 & 1 as i32 != 0;
    val = (val as i32 >> 1 as i32) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[val as usize] as i32 | flag_val(cf, cfn) as i32
        | flag_val(nf, 0 as i32 != 0) as i32
        | flag_val(hf, 0 as i32 != 0) as i32) as uint8_t;
    return val;
}
#[inline]
unsafe fn cb_bit(z: *mut Z80, mut val: uint8_t, mut n: uint8_t) -> uint8_t {
    let result: uint8_t = (val as i32 & (1 as i32) << n as i32)
        as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[result as usize] as i32
        | flag_val(cf, flag_get(z, cf)) as i32
        | flag_val(hf, 1 as i32 != 0) as i32
        | flag_val(nf, 0 as i32 != 0) as i32) as uint8_t;
    return result;
}
#[inline]
unsafe fn ldi(z: *mut Z80) {
    let de: uint16_t = (*z).c2rust_unnamed_1.de;
    let hl: uint16_t = (*z).c2rust_unnamed_2.hl;
    let val: uint8_t = rb(z, hl);
    wb(z, de, val);
    (*z).c2rust_unnamed_2.hl = ((*z).c2rust_unnamed_2.hl).wrapping_add(1);
    (*z).c2rust_unnamed_1.de = ((*z).c2rust_unnamed_1.de).wrapping_add(1);
    (*z).c2rust_unnamed_0.bc = ((*z).c2rust_unnamed_0.bc).wrapping_sub(1);
    let result: uint8_t = (val as i32
        + (*z).c2rust_unnamed.c2rust_unnamed.a as i32) as uint8_t;
    flag_set(z, xf, result as i32 >> 3 as i32 & 1 as i32 != 0);
    flag_set(z, yf, result as i32 >> 1 as i32 & 1 as i32 != 0);
    flag_set(z, nf, 0 as i32 != 0);
    flag_set(z, hf, 0 as i32 != 0);
    flag_set(z, pf, (*z).c2rust_unnamed_0.bc as i32 > 0 as i32);
}
#[inline]
unsafe fn ldd(z: *mut Z80) {
    ldi(z);
    (*z)
        .c2rust_unnamed_2
        .hl = ((*z).c2rust_unnamed_2.hl as i32 - 2 as i32) as uint16_t;
    (*z)
        .c2rust_unnamed_1
        .de = ((*z).c2rust_unnamed_1.de as i32 - 2 as i32) as uint16_t;
}
#[inline]
unsafe fn cpi(z: *mut Z80) {
    let mut cfc: bool = flag_get(z, cf);
    let result: uint8_t = subb(
        z,
        (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
        rb(z, (*z).c2rust_unnamed_2.hl) as uint32_t,
        0 as i32 != 0,
    );
    (*z).c2rust_unnamed_2.hl = ((*z).c2rust_unnamed_2.hl).wrapping_add(1);
    (*z).c2rust_unnamed_0.bc = ((*z).c2rust_unnamed_0.bc).wrapping_sub(1);
    let mut hfc: bool = flag_get(z, hf);
    flag_set(
        z,
        xf,
        result as i32 - hfc as i32 >> 3 as i32 & 1 as i32
            != 0,
    );
    flag_set(
        z,
        yf,
        result as i32 - hfc as i32 >> 1 as i32 & 1 as i32
            != 0,
    );
    flag_set(z, pf, (*z).c2rust_unnamed_0.bc as i32 != 0 as i32);
    flag_set(z, cf, cfc);
    (*z).mem_ptr = ((*z).mem_ptr as i32 + 1 as i32) as uint16_t;
}
#[inline]
unsafe fn cpd(z: *mut Z80) {
    cpi(z);
    (*z)
        .c2rust_unnamed_2
        .hl = ((*z).c2rust_unnamed_2.hl as i32 - 2 as i32) as uint16_t;
    (*z).mem_ptr = ((*z).mem_ptr as i32 - 2 as i32) as uint16_t;
}
unsafe fn in_r_c(z: *mut Z80, mut r: *mut uint8_t) {
    *r = (*z).internal_port_in((*z).c2rust_unnamed_0.bc);
    flag_set(z, zf, *r as i32 == 0 as i32);
    flag_set(z, sf, *r as i32 >> 7 as i32 != 0);
    flag_set(z, pf, parity(*r));
    flag_set(z, nf, 0 as i32 != 0);
    flag_set(z, hf, 0 as i32 != 0);
}
unsafe fn ini(z: *mut Z80) {
    let mut tmp: u32 = (*z).internal_port_in((*z).c2rust_unnamed_0.bc)
        as u32;
    let mut tmp2: u32 = tmp
        .wrapping_add(
            ((*z).c2rust_unnamed_0.c2rust_unnamed.c as i32 + 1 as i32
                & 0xff as i32) as u32,
        );
    (*z)
        .mem_ptr = ((*z).c2rust_unnamed_0.bc as i32 + 1 as i32)
        as uint16_t;
    wb(z, (*z).c2rust_unnamed_2.hl, tmp as uint8_t);
    (*z).c2rust_unnamed_2.hl = ((*z).c2rust_unnamed_2.hl).wrapping_add(1);
    (*z)
        .c2rust_unnamed_0
        .c2rust_unnamed
        .b = ((*z).c2rust_unnamed_0.c2rust_unnamed.b).wrapping_sub(1);
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[(*z).c2rust_unnamed_0.c2rust_unnamed.b as usize] as i32
        & !((1 as i32) << pf as i32)
        | flag_val(nf, tmp >> 7 as i32 & 1 != 0)
            as i32
        | flag_val(
            pf,
            parity(
                (tmp2 & 7
                    ^ (*z).c2rust_unnamed_0.c2rust_unnamed.b as u32) as uint8_t,
            ),
        ) as i32
        | flag_val(hf, tmp2 > 255) as i32
        | flag_val(cf, tmp2 > 255) as i32)
        as uint8_t;
}
unsafe fn ind(z: *mut Z80) {
    let mut tmp: u32 = (*z).internal_port_in((*z).c2rust_unnamed_0.bc)
        as u32;
    let mut tmp2: u32 = tmp
        .wrapping_add(
            ((*z).c2rust_unnamed_0.c2rust_unnamed.c as i32 - 1 as i32
                & 0xff as i32) as u32,
        );
    (*z)
        .mem_ptr = ((*z).c2rust_unnamed_0.bc as i32 - 1 as i32)
        as uint16_t;
    wb(z, (*z).c2rust_unnamed_2.hl, tmp as uint8_t);
    (*z).c2rust_unnamed_2.hl = ((*z).c2rust_unnamed_2.hl).wrapping_sub(1);
    (*z)
        .c2rust_unnamed_0
        .c2rust_unnamed
        .b = ((*z).c2rust_unnamed_0.c2rust_unnamed.b).wrapping_sub(1);
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[(*z).c2rust_unnamed_0.c2rust_unnamed.b as usize] as i32
        & !((1 as i32) << pf as i32)
        | flag_val(nf, tmp >> 7 as i32 & 1 != 0)
            as i32
        | flag_val(
            pf,
            parity(
                (tmp2 & 7
                    ^ (*z).c2rust_unnamed_0.c2rust_unnamed.b as u32) as uint8_t,
            ),
        ) as i32
        | flag_val(hf, tmp2 > 255) as i32
        | flag_val(cf, tmp2 > 255) as i32)
        as uint8_t;
}
unsafe fn outi(z: *mut Z80) {
    let mut tmp: u32 = rb(z, (*z).c2rust_unnamed_2.hl) as u32;
    let mut tmp2: u32 = 0;
    (*z).internal_port_out((*z).c2rust_unnamed_0.bc, tmp as uint8_t);
    (*z).c2rust_unnamed_2.hl = ((*z).c2rust_unnamed_2.hl).wrapping_add(1);
    (*z)
        .c2rust_unnamed_0
        .c2rust_unnamed
        .b = ((*z).c2rust_unnamed_0.c2rust_unnamed.b as i32 - 1 as i32)
        as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = f_szpxy[(*z).c2rust_unnamed_0.c2rust_unnamed.b as usize];
    flag_set(z, nf, tmp >> 7 as i32 & 1 != 0);
    tmp2 = tmp.wrapping_add((*z).c2rust_unnamed_2.c2rust_unnamed.l as u32);
    flag_set(
        z,
        pf,
        parity(
            (tmp2 & 7
                ^ (*z).c2rust_unnamed_0.c2rust_unnamed.b as u32) as uint8_t,
        ),
    );
    flag_set(z, hf, tmp2 > 255);
    flag_set(z, cf, tmp2 > 255);
    (*z)
        .mem_ptr = ((*z).c2rust_unnamed_0.bc as i32 + 1 as i32)
        as uint16_t;
}
unsafe fn outd(z: *mut Z80) {
    outi(z);
    (*z)
        .c2rust_unnamed_2
        .hl = ((*z).c2rust_unnamed_2.hl as i32 - 2 as i32) as uint16_t;
    (*z)
        .mem_ptr = ((*z).c2rust_unnamed_0.bc as i32 - 2 as i32)
        as uint16_t;
}
unsafe fn outc(z: *mut Z80, mut data: uint8_t) {
    (*z).internal_port_out((*z).c2rust_unnamed_0.bc, data);
    (*z)
        .mem_ptr = ((*z).c2rust_unnamed_0.bc as i32 + 1 as i32)
        as uint16_t;
}
unsafe fn daa(z: *mut Z80) {
    let mut correction: uint8_t = 0 as i32 as uint8_t;
    if (*z).c2rust_unnamed.c2rust_unnamed.a as i32 & 0xf as i32
        > 0x9 as i32 || flag_get(z, hf) as i32 != 0
    {
        correction = (correction as i32 + 0x6 as i32) as uint8_t;
    }
    if (*z).c2rust_unnamed.c2rust_unnamed.a as i32 > 0x99 as i32
        || flag_get(z, cf) as i32 != 0
    {
        correction = (correction as i32 + 0x60 as i32) as uint8_t;
        flag_set(z, cf, 1 as i32 != 0);
    }
    let substraction: bool = flag_get(z, nf);
    if substraction {
        flag_set(
            z,
            hf,
            flag_get(z, hf) as i32 != 0
                && ((*z).c2rust_unnamed.c2rust_unnamed.a as i32
                    & 0xf as i32) < 0x6 as i32,
        );
        (*z)
            .c2rust_unnamed
            .c2rust_unnamed
            .a = ((*z).c2rust_unnamed.c2rust_unnamed.a as i32
            - correction as i32) as uint8_t;
    } else {
        flag_set(
            z,
            hf,
            (*z).c2rust_unnamed.c2rust_unnamed.a as i32 & 0xf as i32
                > 0x9 as i32,
        );
        (*z)
            .c2rust_unnamed
            .c2rust_unnamed
            .a = ((*z).c2rust_unnamed.c2rust_unnamed.a as i32
            + correction as i32) as uint8_t;
    }
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as i32
        & !((1 as i32) << sf as i32
            | (1 as i32) << zf as i32
            | (1 as i32) << pf as i32
            | (1 as i32) << xf as i32
            | (1 as i32) << yf as i32)) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as i32
        | f_szpxy[(*z).c2rust_unnamed.c2rust_unnamed.a as usize] as i32)
        as uint8_t;
}
#[inline]
unsafe fn displace(
    z: *mut Z80,
    mut base_addr: uint16_t,
    mut displacement: int8_t,
) -> uint16_t {
    let addr: uint16_t = (base_addr as i32 + displacement as i32)
        as uint16_t;
    (*z).mem_ptr = addr;
    return addr;
}
#[inline]
unsafe fn process_interrupts(z: *mut Z80) -> u32 {
    let mut cyc: u32 = 0;
    if (*z).iff_delay as i32 > 0 as i32 {
        (*z).iff_delay = ((*z).iff_delay as i32 - 1 as i32) as uint8_t;
        if (*z).iff_delay as i32 == 0 as i32 {
            (*z).iff1 = 1 as i32 != 0;
            (*z).iff2 = 1 as i32 != 0;
        }
        return cyc;
    }
    if (*z).nmi_pending != 0 {
        (*z)
            .nmi_pending = ((*z).nmi_pending as i32
            & !(PULSE as i32)) as uint8_t;
        (*z).halted = 0 as i32 != 0;
        (*z).iff1 = 0 as i32 != 0;
        inc_r(z);
        cyc = cyc.wrapping_add(11);
        call(z, 0x66 as i32 as uint16_t);
        return cyc;
    }
    if (*z).irq_pending as i32 != 0 && (*z).iff1 as i32 != 0 {
        (*z)
            .irq_pending = ((*z).irq_pending as i32
            & !(PULSE as i32)) as uint8_t;
        (*z).halted = 0 as i32 != 0;
        (*z).iff1 = 0 as i32 != 0;
        (*z).iff2 = 0 as i32 != 0;
        inc_r(z);
        match (*z).interrupt_mode as i32 {
            0 => {
                cyc = cyc.wrapping_add(11);
                cyc = cyc.wrapping_add(exec_opcode(z, (*z).irq_data));
            }
            1 => {
                cyc = cyc.wrapping_add(13);
                call(z, 0x38 as i32 as uint16_t);
            }
            2 => {
                cyc = cyc.wrapping_add(19);
                call(
                    z,
                    rw(
                        z,
                        (((*z).i as i32) << 8 as i32
                            | (*z).irq_data as i32) as uint16_t,
                    ),
                );
            }
            _ => {}
        }
        return cyc;
    }
    return cyc;
}
#[no_mangle]
pub unsafe fn reset(z: *mut Z80) {
    (*z).pc = 0 as i32 as uint16_t;
    (*z).mem_ptr = 0 as i32 as uint16_t;
    (*z).i = 0 as i32 as uint8_t;
    (*z).r = 0 as i32 as uint8_t;
    (*z).interrupt_mode = 0 as i32 as uint8_t;
    (*z).iff_delay = 0 as i32 as uint8_t;
    (*z).iff1 = 0 as i32 != 0;
    (*z).iff2 = 0 as i32 != 0;
    (*z).halted = 0 as i32 != 0;
    (*z).nmi_pending = 0 as i32 as uint8_t;
}
unsafe fn step_s(z: *mut Z80) -> u32 {
    let mut cyc: u32 = 0;
    if (*z).halted {
        cyc = cyc.wrapping_add(exec_opcode(z, 0 as i32 as uint8_t));
    } else {
        let opcode: uint8_t = nextb(z);
        cyc = cyc.wrapping_add(exec_opcode(z, opcode));
    }
    cyc = cyc.wrapping_add(process_interrupts(z));
    return cyc;
}
#[no_mangle]
pub unsafe fn set_pc(z: *mut Z80, mut pc: uint16_t) {
    (*z).pc = pc;
}
#[no_mangle]
pub unsafe fn set_sp(z: *mut Z80, mut sp: uint16_t) {
    (*z).sp = sp;
}
#[no_mangle]
pub unsafe fn step_n(
    z: *mut Z80,
    mut cycles: u32,
) -> u32 {
    let mut cyc: u32 = 0;
    while cyc < cycles {
        cyc = cyc.wrapping_add(step_s(z));
    }
    return cyc;
}
#[no_mangle]
pub unsafe fn assert_nmi(z: *mut Z80) {
    (*z)
        .nmi_pending = ((*z).nmi_pending as i32 | ASSERT as i32)
        as uint8_t;
}
#[no_mangle]
pub unsafe fn pulse_nmi(z: *mut Z80) {
    (*z)
        .nmi_pending = ((*z).nmi_pending as i32 | PULSE as i32)
        as uint8_t;
}
#[no_mangle]
pub unsafe fn clr_nmi(z: *mut Z80) {
    (*z).nmi_pending = 0 as i32 as uint8_t;
}
#[no_mangle]
pub unsafe fn assert_irq(z: *mut Z80, mut data: uint8_t) {
    (*z)
        .irq_pending = ((*z).irq_pending as i32 | ASSERT as i32)
        as uint8_t;
    (*z).irq_data = data;
}
#[no_mangle]
pub unsafe fn pulse_irq(z: *mut Z80, mut data: uint8_t) {
    (*z)
        .irq_pending = ((*z).irq_pending as i32 | PULSE as i32)
        as uint8_t;
    (*z).irq_data = data;
}
#[no_mangle]
pub unsafe fn clr_irq(z: *mut Z80) {
    (*z).irq_pending = 0 as i32 as uint8_t;
}
unsafe fn exec_opcode(z: *mut Z80, mut opcode: uint8_t) -> u32 {
    let mut cyc: u32 = 0;
    inc_r(z);
    match opcode as i32 {
        127 => {
            cyc = cyc.wrapping_add(4);
            (*z).c2rust_unnamed.c2rust_unnamed.a = (*z).c2rust_unnamed.c2rust_unnamed.a;
        }
        120 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (*z).c2rust_unnamed_0.c2rust_unnamed.b;
        }
        121 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (*z).c2rust_unnamed_0.c2rust_unnamed.c;
        }
        122 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (*z).c2rust_unnamed_1.c2rust_unnamed.d;
        }
        123 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (*z).c2rust_unnamed_1.c2rust_unnamed.e;
        }
        124 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (*z).c2rust_unnamed_2.c2rust_unnamed.h;
        }
        125 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (*z).c2rust_unnamed_2.c2rust_unnamed.l;
        }
        71 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = (*z).c2rust_unnamed.c2rust_unnamed.a;
        }
        64 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = (*z).c2rust_unnamed_0.c2rust_unnamed.b;
        }
        65 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = (*z).c2rust_unnamed_0.c2rust_unnamed.c;
        }
        66 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = (*z).c2rust_unnamed_1.c2rust_unnamed.d;
        }
        67 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = (*z).c2rust_unnamed_1.c2rust_unnamed.e;
        }
        68 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = (*z).c2rust_unnamed_2.c2rust_unnamed.h;
        }
        69 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = (*z).c2rust_unnamed_2.c2rust_unnamed.l;
        }
        79 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = (*z).c2rust_unnamed.c2rust_unnamed.a;
        }
        72 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = (*z).c2rust_unnamed_0.c2rust_unnamed.b;
        }
        73 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = (*z).c2rust_unnamed_0.c2rust_unnamed.c;
        }
        74 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = (*z).c2rust_unnamed_1.c2rust_unnamed.d;
        }
        75 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = (*z).c2rust_unnamed_1.c2rust_unnamed.e;
        }
        76 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = (*z).c2rust_unnamed_2.c2rust_unnamed.h;
        }
        77 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = (*z).c2rust_unnamed_2.c2rust_unnamed.l;
        }
        87 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = (*z).c2rust_unnamed.c2rust_unnamed.a;
        }
        80 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = (*z).c2rust_unnamed_0.c2rust_unnamed.b;
        }
        81 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = (*z).c2rust_unnamed_0.c2rust_unnamed.c;
        }
        82 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = (*z).c2rust_unnamed_1.c2rust_unnamed.d;
        }
        83 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = (*z).c2rust_unnamed_1.c2rust_unnamed.e;
        }
        84 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = (*z).c2rust_unnamed_2.c2rust_unnamed.h;
        }
        85 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = (*z).c2rust_unnamed_2.c2rust_unnamed.l;
        }
        95 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = (*z).c2rust_unnamed.c2rust_unnamed.a;
        }
        88 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = (*z).c2rust_unnamed_0.c2rust_unnamed.b;
        }
        89 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = (*z).c2rust_unnamed_0.c2rust_unnamed.c;
        }
        90 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = (*z).c2rust_unnamed_1.c2rust_unnamed.d;
        }
        91 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = (*z).c2rust_unnamed_1.c2rust_unnamed.e;
        }
        92 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = (*z).c2rust_unnamed_2.c2rust_unnamed.h;
        }
        93 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = (*z).c2rust_unnamed_2.c2rust_unnamed.l;
        }
        103 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .h = (*z).c2rust_unnamed.c2rust_unnamed.a;
        }
        96 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .h = (*z).c2rust_unnamed_0.c2rust_unnamed.b;
        }
        97 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .h = (*z).c2rust_unnamed_0.c2rust_unnamed.c;
        }
        98 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .h = (*z).c2rust_unnamed_1.c2rust_unnamed.d;
        }
        99 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .h = (*z).c2rust_unnamed_1.c2rust_unnamed.e;
        }
        100 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .h = (*z).c2rust_unnamed_2.c2rust_unnamed.h;
        }
        101 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .h = (*z).c2rust_unnamed_2.c2rust_unnamed.l;
        }
        111 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .l = (*z).c2rust_unnamed.c2rust_unnamed.a;
        }
        104 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .l = (*z).c2rust_unnamed_0.c2rust_unnamed.b;
        }
        105 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .l = (*z).c2rust_unnamed_0.c2rust_unnamed.c;
        }
        106 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .l = (*z).c2rust_unnamed_1.c2rust_unnamed.d;
        }
        107 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .l = (*z).c2rust_unnamed_1.c2rust_unnamed.e;
        }
        108 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .l = (*z).c2rust_unnamed_2.c2rust_unnamed.h;
        }
        109 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .l = (*z).c2rust_unnamed_2.c2rust_unnamed.l;
        }
        126 => {
            cyc = cyc.wrapping_add(7);
            (*z).c2rust_unnamed.c2rust_unnamed.a = rb(z, (*z).c2rust_unnamed_2.hl);
        }
        70 => {
            cyc = cyc.wrapping_add(7);
            (*z).c2rust_unnamed_0.c2rust_unnamed.b = rb(z, (*z).c2rust_unnamed_2.hl);
        }
        78 => {
            cyc = cyc.wrapping_add(7);
            (*z).c2rust_unnamed_0.c2rust_unnamed.c = rb(z, (*z).c2rust_unnamed_2.hl);
        }
        86 => {
            cyc = cyc.wrapping_add(7);
            (*z).c2rust_unnamed_1.c2rust_unnamed.d = rb(z, (*z).c2rust_unnamed_2.hl);
        }
        94 => {
            cyc = cyc.wrapping_add(7);
            (*z).c2rust_unnamed_1.c2rust_unnamed.e = rb(z, (*z).c2rust_unnamed_2.hl);
        }
        102 => {
            cyc = cyc.wrapping_add(7);
            (*z).c2rust_unnamed_2.c2rust_unnamed.h = rb(z, (*z).c2rust_unnamed_2.hl);
        }
        110 => {
            cyc = cyc.wrapping_add(7);
            (*z).c2rust_unnamed_2.c2rust_unnamed.l = rb(z, (*z).c2rust_unnamed_2.hl);
        }
        119 => {
            cyc = cyc.wrapping_add(7);
            wb(z, (*z).c2rust_unnamed_2.hl, (*z).c2rust_unnamed.c2rust_unnamed.a);
        }
        112 => {
            cyc = cyc.wrapping_add(7);
            wb(z, (*z).c2rust_unnamed_2.hl, (*z).c2rust_unnamed_0.c2rust_unnamed.b);
        }
        113 => {
            cyc = cyc.wrapping_add(7);
            wb(z, (*z).c2rust_unnamed_2.hl, (*z).c2rust_unnamed_0.c2rust_unnamed.c);
        }
        114 => {
            cyc = cyc.wrapping_add(7);
            wb(z, (*z).c2rust_unnamed_2.hl, (*z).c2rust_unnamed_1.c2rust_unnamed.d);
        }
        115 => {
            cyc = cyc.wrapping_add(7);
            wb(z, (*z).c2rust_unnamed_2.hl, (*z).c2rust_unnamed_1.c2rust_unnamed.e);
        }
        116 => {
            cyc = cyc.wrapping_add(7);
            wb(z, (*z).c2rust_unnamed_2.hl, (*z).c2rust_unnamed_2.c2rust_unnamed.h);
        }
        117 => {
            cyc = cyc.wrapping_add(7);
            wb(z, (*z).c2rust_unnamed_2.hl, (*z).c2rust_unnamed_2.c2rust_unnamed.l);
        }
        62 => {
            cyc = cyc.wrapping_add(7);
            (*z).c2rust_unnamed.c2rust_unnamed.a = nextb(z);
        }
        6 => {
            cyc = cyc.wrapping_add(7);
            (*z).c2rust_unnamed_0.c2rust_unnamed.b = nextb(z);
        }
        14 => {
            cyc = cyc.wrapping_add(7);
            (*z).c2rust_unnamed_0.c2rust_unnamed.c = nextb(z);
        }
        22 => {
            cyc = cyc.wrapping_add(7);
            (*z).c2rust_unnamed_1.c2rust_unnamed.d = nextb(z);
        }
        30 => {
            cyc = cyc.wrapping_add(7);
            (*z).c2rust_unnamed_1.c2rust_unnamed.e = nextb(z);
        }
        38 => {
            cyc = cyc.wrapping_add(7);
            (*z).c2rust_unnamed_2.c2rust_unnamed.h = nextb(z);
        }
        46 => {
            cyc = cyc.wrapping_add(7);
            (*z).c2rust_unnamed_2.c2rust_unnamed.l = nextb(z);
        }
        54 => {
            cyc = cyc.wrapping_add(10);
            wb(z, (*z).c2rust_unnamed_2.hl, nextb(z));
        }
        10 => {
            cyc = cyc.wrapping_add(7);
            (*z).c2rust_unnamed.c2rust_unnamed.a = rb(z, (*z).c2rust_unnamed_0.bc);
            (*z)
                .mem_ptr = ((*z).c2rust_unnamed_0.bc as i32 + 1 as i32)
                as uint16_t;
        }
        26 => {
            cyc = cyc.wrapping_add(7);
            (*z).c2rust_unnamed.c2rust_unnamed.a = rb(z, (*z).c2rust_unnamed_1.de);
            (*z)
                .mem_ptr = ((*z).c2rust_unnamed_1.de as i32 + 1 as i32)
                as uint16_t;
        }
        58 => {
            cyc = cyc.wrapping_add(13);
            let addr: uint16_t = nextw(z);
            (*z).c2rust_unnamed.c2rust_unnamed.a = rb(z, addr);
            (*z).mem_ptr = (addr as i32 + 1 as i32) as uint16_t;
        }
        2 => {
            cyc = cyc.wrapping_add(7);
            wb(z, (*z).c2rust_unnamed_0.bc, (*z).c2rust_unnamed.c2rust_unnamed.a);
            (*z)
                .mem_ptr = (((*z).c2rust_unnamed.c2rust_unnamed.a as i32)
                << 8 as i32
                | (*z).c2rust_unnamed_0.bc as i32 + 1 as i32
                    & 0xff as i32) as uint16_t;
        }
        18 => {
            cyc = cyc.wrapping_add(7);
            wb(z, (*z).c2rust_unnamed_1.de, (*z).c2rust_unnamed.c2rust_unnamed.a);
            (*z)
                .mem_ptr = (((*z).c2rust_unnamed.c2rust_unnamed.a as i32)
                << 8 as i32
                | (*z).c2rust_unnamed_1.de as i32 + 1 as i32
                    & 0xff as i32) as uint16_t;
        }
        50 => {
            cyc = cyc.wrapping_add(13);
            let addr_0: uint16_t = nextw(z);
            wb(z, addr_0, (*z).c2rust_unnamed.c2rust_unnamed.a);
            (*z)
                .mem_ptr = (((*z).c2rust_unnamed.c2rust_unnamed.a as i32)
                << 8 as i32
                | addr_0 as i32 + 1 as i32 & 0xff as i32)
                as uint16_t;
        }
        1 => {
            cyc = cyc.wrapping_add(10);
            (*z).c2rust_unnamed_0.bc = nextw(z);
        }
        17 => {
            cyc = cyc.wrapping_add(10);
            (*z).c2rust_unnamed_1.de = nextw(z);
        }
        33 => {
            cyc = cyc.wrapping_add(10);
            (*z).c2rust_unnamed_2.hl = nextw(z);
        }
        49 => {
            cyc = cyc.wrapping_add(10);
            (*z).sp = nextw(z);
        }
        42 => {
            cyc = cyc.wrapping_add(16);
            let addr_1: uint16_t = nextw(z);
            (*z).c2rust_unnamed_2.hl = rw(z, addr_1);
            (*z).mem_ptr = (addr_1 as i32 + 1 as i32) as uint16_t;
        }
        34 => {
            cyc = cyc.wrapping_add(16);
            let addr_2: uint16_t = nextw(z);
            ww(z, addr_2, (*z).c2rust_unnamed_2.hl);
            (*z).mem_ptr = (addr_2 as i32 + 1 as i32) as uint16_t;
        }
        249 => {
            cyc = cyc.wrapping_add(6);
            (*z).sp = (*z).c2rust_unnamed_2.hl;
        }
        235 => {
            cyc = cyc.wrapping_add(4);
            let de: uint16_t = (*z).c2rust_unnamed_1.de;
            (*z).c2rust_unnamed_1.de = (*z).c2rust_unnamed_2.hl;
            (*z).c2rust_unnamed_2.hl = de;
        }
        227 => {
            cyc = cyc.wrapping_add(19);
            let val: uint16_t = rw(z, (*z).sp);
            ww(z, (*z).sp, (*z).c2rust_unnamed_2.hl);
            (*z).c2rust_unnamed_2.hl = val;
            (*z).mem_ptr = val;
        }
        135 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                0 as i32 != 0,
            );
        }
        128 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_0.c2rust_unnamed.b as uint32_t,
                0 as i32 != 0,
            );
        }
        129 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_0.c2rust_unnamed.c as uint32_t,
                0 as i32 != 0,
            );
        }
        130 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_1.c2rust_unnamed.d as uint32_t,
                0 as i32 != 0,
            );
        }
        131 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_1.c2rust_unnamed.e as uint32_t,
                0 as i32 != 0,
            );
        }
        132 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_2.c2rust_unnamed.h as uint32_t,
                0 as i32 != 0,
            );
        }
        133 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_2.c2rust_unnamed.l as uint32_t,
                0 as i32 != 0,
            );
        }
        134 => {
            cyc = cyc.wrapping_add(7);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                rb(z, (*z).c2rust_unnamed_2.hl) as uint32_t,
                0 as i32 != 0,
            );
        }
        198 => {
            cyc = cyc.wrapping_add(7);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                nextb(z) as uint32_t,
                0 as i32 != 0,
            );
        }
        143 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                flag_get(z, cf),
            );
        }
        136 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_0.c2rust_unnamed.b as uint32_t,
                flag_get(z, cf),
            );
        }
        137 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_0.c2rust_unnamed.c as uint32_t,
                flag_get(z, cf),
            );
        }
        138 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_1.c2rust_unnamed.d as uint32_t,
                flag_get(z, cf),
            );
        }
        139 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_1.c2rust_unnamed.e as uint32_t,
                flag_get(z, cf),
            );
        }
        140 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_2.c2rust_unnamed.h as uint32_t,
                flag_get(z, cf),
            );
        }
        141 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_2.c2rust_unnamed.l as uint32_t,
                flag_get(z, cf),
            );
        }
        142 => {
            cyc = cyc.wrapping_add(7);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                rb(z, (*z).c2rust_unnamed_2.hl) as uint32_t,
                flag_get(z, cf),
            );
        }
        206 => {
            cyc = cyc.wrapping_add(7);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                nextb(z) as uint32_t,
                flag_get(z, cf),
            );
        }
        151 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                0 as i32 != 0,
            );
        }
        144 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_0.c2rust_unnamed.b as uint32_t,
                0 as i32 != 0,
            );
        }
        145 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_0.c2rust_unnamed.c as uint32_t,
                0 as i32 != 0,
            );
        }
        146 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_1.c2rust_unnamed.d as uint32_t,
                0 as i32 != 0,
            );
        }
        147 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_1.c2rust_unnamed.e as uint32_t,
                0 as i32 != 0,
            );
        }
        148 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_2.c2rust_unnamed.h as uint32_t,
                0 as i32 != 0,
            );
        }
        149 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_2.c2rust_unnamed.l as uint32_t,
                0 as i32 != 0,
            );
        }
        150 => {
            cyc = cyc.wrapping_add(7);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                rb(z, (*z).c2rust_unnamed_2.hl) as uint32_t,
                0 as i32 != 0,
            );
        }
        214 => {
            cyc = cyc.wrapping_add(7);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                nextb(z) as uint32_t,
                0 as i32 != 0,
            );
        }
        159 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                flag_get(z, cf),
            );
        }
        152 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_0.c2rust_unnamed.b as uint32_t,
                flag_get(z, cf),
            );
        }
        153 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_0.c2rust_unnamed.c as uint32_t,
                flag_get(z, cf),
            );
        }
        154 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_1.c2rust_unnamed.d as uint32_t,
                flag_get(z, cf),
            );
        }
        155 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_1.c2rust_unnamed.e as uint32_t,
                flag_get(z, cf),
            );
        }
        156 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_2.c2rust_unnamed.h as uint32_t,
                flag_get(z, cf),
            );
        }
        157 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_2.c2rust_unnamed.l as uint32_t,
                flag_get(z, cf),
            );
        }
        158 => {
            cyc = cyc.wrapping_add(7);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                rb(z, (*z).c2rust_unnamed_2.hl) as uint32_t,
                flag_get(z, cf),
            );
        }
        222 => {
            cyc = cyc.wrapping_add(7);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                nextb(z) as uint32_t,
                flag_get(z, cf),
            );
        }
        9 => {
            cyc = cyc.wrapping_add(11);
            addhl(z, (*z).c2rust_unnamed_0.bc);
        }
        25 => {
            cyc = cyc.wrapping_add(11);
            addhl(z, (*z).c2rust_unnamed_1.de);
        }
        41 => {
            cyc = cyc.wrapping_add(11);
            addhl(z, (*z).c2rust_unnamed_2.hl);
        }
        57 => {
            cyc = cyc.wrapping_add(11);
            addhl(z, (*z).sp);
        }
        243 => {
            cyc = cyc.wrapping_add(4);
            (*z).iff2 = 0 as i32 != 0;
            (*z).iff1 = (*z).iff2;
        }
        251 => {
            cyc = cyc.wrapping_add(4);
            (*z).iff_delay = 1 as i32 as uint8_t;
        }
        0 => {
            cyc = cyc.wrapping_add(4);
        }
        118 => {
            cyc = cyc.wrapping_add(4);
            (*z).halted = 1 as i32 != 0;
        }
        60 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = inc(z, (*z).c2rust_unnamed.c2rust_unnamed.a);
        }
        4 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = inc(z, (*z).c2rust_unnamed_0.c2rust_unnamed.b);
        }
        12 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = inc(z, (*z).c2rust_unnamed_0.c2rust_unnamed.c);
        }
        20 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = inc(z, (*z).c2rust_unnamed_1.c2rust_unnamed.d);
        }
        28 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = inc(z, (*z).c2rust_unnamed_1.c2rust_unnamed.e);
        }
        36 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .h = inc(z, (*z).c2rust_unnamed_2.c2rust_unnamed.h);
        }
        44 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .l = inc(z, (*z).c2rust_unnamed_2.c2rust_unnamed.l);
        }
        52 => {
            cyc = cyc.wrapping_add(11);
            let mut result: uint8_t = inc(z, rb(z, (*z).c2rust_unnamed_2.hl));
            wb(z, (*z).c2rust_unnamed_2.hl, result);
        }
        61 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = dec(z, (*z).c2rust_unnamed.c2rust_unnamed.a);
        }
        5 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = dec(z, (*z).c2rust_unnamed_0.c2rust_unnamed.b);
        }
        13 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = dec(z, (*z).c2rust_unnamed_0.c2rust_unnamed.c);
        }
        21 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = dec(z, (*z).c2rust_unnamed_1.c2rust_unnamed.d);
        }
        29 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = dec(z, (*z).c2rust_unnamed_1.c2rust_unnamed.e);
        }
        37 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .h = dec(z, (*z).c2rust_unnamed_2.c2rust_unnamed.h);
        }
        45 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .l = dec(z, (*z).c2rust_unnamed_2.c2rust_unnamed.l);
        }
        53 => {
            cyc = cyc.wrapping_add(11);
            let mut result_0: uint8_t = dec(z, rb(z, (*z).c2rust_unnamed_2.hl));
            wb(z, (*z).c2rust_unnamed_2.hl, result_0);
        }
        3 => {
            cyc = cyc.wrapping_add(6);
            (*z).c2rust_unnamed_0.bc = ((*z).c2rust_unnamed_0.bc).wrapping_add(1);
        }
        19 => {
            cyc = cyc.wrapping_add(6);
            (*z).c2rust_unnamed_1.de = ((*z).c2rust_unnamed_1.de).wrapping_add(1);
        }
        35 => {
            cyc = cyc.wrapping_add(6);
            (*z).c2rust_unnamed_2.hl = ((*z).c2rust_unnamed_2.hl).wrapping_add(1);
        }
        51 => {
            cyc = cyc.wrapping_add(6);
            (*z).sp = ((*z).sp).wrapping_add(1);
        }
        11 => {
            cyc = cyc.wrapping_add(6);
            (*z).c2rust_unnamed_0.bc = ((*z).c2rust_unnamed_0.bc).wrapping_sub(1);
        }
        27 => {
            cyc = cyc.wrapping_add(6);
            (*z).c2rust_unnamed_1.de = ((*z).c2rust_unnamed_1.de).wrapping_sub(1);
        }
        43 => {
            cyc = cyc.wrapping_add(6);
            (*z).c2rust_unnamed_2.hl = ((*z).c2rust_unnamed_2.hl).wrapping_sub(1);
        }
        59 => {
            cyc = cyc.wrapping_add(6);
            (*z).sp = ((*z).sp).wrapping_sub(1);
        }
        39 => {
            cyc = cyc.wrapping_add(4);
            daa(z);
        }
        47 => {
            cyc = cyc.wrapping_add(4);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = !((*z).c2rust_unnamed.c2rust_unnamed.a as i32) as uint8_t;
            flag_set(z, nf, 1 as i32 != 0);
            flag_set(z, hf, 1 as i32 != 0);
            flag_set(
                z,
                xf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 >> 3 as i32
                    & 1 as i32 != 0,
            );
            flag_set(
                z,
                yf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 >> 5 as i32
                    & 1 as i32 != 0,
            );
        }
        55 => {
            cyc = cyc.wrapping_add(4);
            flag_set(z, cf, 1 as i32 != 0);
            flag_set(z, nf, 0 as i32 != 0);
            flag_set(z, hf, 0 as i32 != 0);
            flag_set(
                z,
                xf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 >> 3 as i32
                    & 1 as i32 != 0,
            );
            flag_set(
                z,
                yf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 >> 5 as i32
                    & 1 as i32 != 0,
            );
        }
        63 => {
            cyc = cyc.wrapping_add(4);
            flag_set(z, hf, flag_get(z, cf));
            flag_set(z, cf, !flag_get(z, cf));
            flag_set(z, nf, 0 as i32 != 0);
            flag_set(
                z,
                xf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 >> 3 as i32
                    & 1 as i32 != 0,
            );
            flag_set(
                z,
                yf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 >> 5 as i32
                    & 1 as i32 != 0,
            );
        }
        7 => {
            cyc = cyc.wrapping_add(4);
            flag_set(
                z,
                cf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 >> 7 as i32
                    != 0,
            );
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (((*z).c2rust_unnamed.c2rust_unnamed.a as i32)
                << 1 as i32 | flag_get(z, cf) as i32) as uint8_t;
            flag_set(z, nf, 0 as i32 != 0);
            flag_set(z, hf, 0 as i32 != 0);
            flag_set(
                z,
                xf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 >> 3 as i32
                    & 1 as i32 != 0,
            );
            flag_set(
                z,
                yf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 >> 5 as i32
                    & 1 as i32 != 0,
            );
        }
        15 => {
            cyc = cyc.wrapping_add(4);
            flag_set(
                z,
                cf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 & 1 as i32
                    != 0,
            );
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = ((*z).c2rust_unnamed.c2rust_unnamed.a as i32
                >> 1 as i32
                | (flag_get(z, cf) as i32) << 7 as i32) as uint8_t;
            flag_set(z, nf, 0 as i32 != 0);
            flag_set(z, hf, 0 as i32 != 0);
            flag_set(
                z,
                xf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 >> 3 as i32
                    & 1 as i32 != 0,
            );
            flag_set(
                z,
                yf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 >> 5 as i32
                    & 1 as i32 != 0,
            );
        }
        23 => {
            cyc = cyc.wrapping_add(4);
            let cy: bool = flag_get(z, cf);
            flag_set(
                z,
                cf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 >> 7 as i32
                    != 0,
            );
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (((*z).c2rust_unnamed.c2rust_unnamed.a as i32)
                << 1 as i32 | cy as i32) as uint8_t;
            flag_set(z, nf, 0 as i32 != 0);
            flag_set(z, hf, 0 as i32 != 0);
            flag_set(
                z,
                xf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 >> 3 as i32
                    & 1 as i32 != 0,
            );
            flag_set(
                z,
                yf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 >> 5 as i32
                    & 1 as i32 != 0,
            );
        }
        31 => {
            cyc = cyc.wrapping_add(4);
            let cy_0: bool = flag_get(z, cf);
            flag_set(
                z,
                cf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 & 1 as i32
                    != 0,
            );
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = ((*z).c2rust_unnamed.c2rust_unnamed.a as i32
                >> 1 as i32 | (cy_0 as i32) << 7 as i32)
                as uint8_t;
            flag_set(z, nf, 0 as i32 != 0);
            flag_set(z, hf, 0 as i32 != 0);
            flag_set(
                z,
                xf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 >> 3 as i32
                    & 1 as i32 != 0,
            );
            flag_set(
                z,
                yf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 >> 5 as i32
                    & 1 as i32 != 0,
            );
        }
        167 => {
            cyc = cyc.wrapping_add(4);
            land(z, (*z).c2rust_unnamed.c2rust_unnamed.a);
        }
        160 => {
            cyc = cyc.wrapping_add(4);
            land(z, (*z).c2rust_unnamed_0.c2rust_unnamed.b);
        }
        161 => {
            cyc = cyc.wrapping_add(4);
            land(z, (*z).c2rust_unnamed_0.c2rust_unnamed.c);
        }
        162 => {
            cyc = cyc.wrapping_add(4);
            land(z, (*z).c2rust_unnamed_1.c2rust_unnamed.d);
        }
        163 => {
            cyc = cyc.wrapping_add(4);
            land(z, (*z).c2rust_unnamed_1.c2rust_unnamed.e);
        }
        164 => {
            cyc = cyc.wrapping_add(4);
            land(z, (*z).c2rust_unnamed_2.c2rust_unnamed.h);
        }
        165 => {
            cyc = cyc.wrapping_add(4);
            land(z, (*z).c2rust_unnamed_2.c2rust_unnamed.l);
        }
        166 => {
            cyc = cyc.wrapping_add(7);
            land(z, rb(z, (*z).c2rust_unnamed_2.hl));
        }
        230 => {
            cyc = cyc.wrapping_add(7);
            land(z, nextb(z));
        }
        175 => {
            cyc = cyc.wrapping_add(4);
            lxor(z, (*z).c2rust_unnamed.c2rust_unnamed.a);
        }
        168 => {
            cyc = cyc.wrapping_add(4);
            lxor(z, (*z).c2rust_unnamed_0.c2rust_unnamed.b);
        }
        169 => {
            cyc = cyc.wrapping_add(4);
            lxor(z, (*z).c2rust_unnamed_0.c2rust_unnamed.c);
        }
        170 => {
            cyc = cyc.wrapping_add(4);
            lxor(z, (*z).c2rust_unnamed_1.c2rust_unnamed.d);
        }
        171 => {
            cyc = cyc.wrapping_add(4);
            lxor(z, (*z).c2rust_unnamed_1.c2rust_unnamed.e);
        }
        172 => {
            cyc = cyc.wrapping_add(4);
            lxor(z, (*z).c2rust_unnamed_2.c2rust_unnamed.h);
        }
        173 => {
            cyc = cyc.wrapping_add(4);
            lxor(z, (*z).c2rust_unnamed_2.c2rust_unnamed.l);
        }
        174 => {
            cyc = cyc.wrapping_add(7);
            lxor(z, rb(z, (*z).c2rust_unnamed_2.hl));
        }
        238 => {
            cyc = cyc.wrapping_add(7);
            lxor(z, nextb(z));
        }
        183 => {
            cyc = cyc.wrapping_add(4);
            lor(z, (*z).c2rust_unnamed.c2rust_unnamed.a);
        }
        176 => {
            cyc = cyc.wrapping_add(4);
            lor(z, (*z).c2rust_unnamed_0.c2rust_unnamed.b);
        }
        177 => {
            cyc = cyc.wrapping_add(4);
            lor(z, (*z).c2rust_unnamed_0.c2rust_unnamed.c);
        }
        178 => {
            cyc = cyc.wrapping_add(4);
            lor(z, (*z).c2rust_unnamed_1.c2rust_unnamed.d);
        }
        179 => {
            cyc = cyc.wrapping_add(4);
            lor(z, (*z).c2rust_unnamed_1.c2rust_unnamed.e);
        }
        180 => {
            cyc = cyc.wrapping_add(4);
            lor(z, (*z).c2rust_unnamed_2.c2rust_unnamed.h);
        }
        181 => {
            cyc = cyc.wrapping_add(4);
            lor(z, (*z).c2rust_unnamed_2.c2rust_unnamed.l);
        }
        182 => {
            cyc = cyc.wrapping_add(7);
            lor(z, rb(z, (*z).c2rust_unnamed_2.hl));
        }
        246 => {
            cyc = cyc.wrapping_add(7);
            lor(z, nextb(z));
        }
        191 => {
            cyc = cyc.wrapping_add(4);
            cp(z, (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t);
        }
        184 => {
            cyc = cyc.wrapping_add(4);
            cp(z, (*z).c2rust_unnamed_0.c2rust_unnamed.b as uint32_t);
        }
        185 => {
            cyc = cyc.wrapping_add(4);
            cp(z, (*z).c2rust_unnamed_0.c2rust_unnamed.c as uint32_t);
        }
        186 => {
            cyc = cyc.wrapping_add(4);
            cp(z, (*z).c2rust_unnamed_1.c2rust_unnamed.d as uint32_t);
        }
        187 => {
            cyc = cyc.wrapping_add(4);
            cp(z, (*z).c2rust_unnamed_1.c2rust_unnamed.e as uint32_t);
        }
        188 => {
            cyc = cyc.wrapping_add(4);
            cp(z, (*z).c2rust_unnamed_2.c2rust_unnamed.h as uint32_t);
        }
        189 => {
            cyc = cyc.wrapping_add(4);
            cp(z, (*z).c2rust_unnamed_2.c2rust_unnamed.l as uint32_t);
        }
        190 => {
            cyc = cyc.wrapping_add(7);
            cp(z, rb(z, (*z).c2rust_unnamed_2.hl) as uint32_t);
        }
        254 => {
            cyc = cyc.wrapping_add(7);
            cp(z, nextb(z) as uint32_t);
        }
        195 => {
            cyc = cyc.wrapping_add(10);
            jump(z, nextw(z));
        }
        194 => {
            cyc = cyc.wrapping_add(10);
            cond_jump(z, flag_get(z, zf) as i32 == 0 as i32);
        }
        202 => {
            cyc = cyc.wrapping_add(10);
            cond_jump(z, flag_get(z, zf) as i32 == 1 as i32);
        }
        210 => {
            cyc = cyc.wrapping_add(10);
            cond_jump(z, flag_get(z, cf) as i32 == 0 as i32);
        }
        218 => {
            cyc = cyc.wrapping_add(10);
            cond_jump(z, flag_get(z, cf) as i32 == 1 as i32);
        }
        226 => {
            cyc = cyc.wrapping_add(10);
            cond_jump(z, flag_get(z, pf) as i32 == 0 as i32);
        }
        234 => {
            cyc = cyc.wrapping_add(10);
            cond_jump(z, flag_get(z, pf) as i32 == 1 as i32);
        }
        242 => {
            cyc = cyc.wrapping_add(10);
            cond_jump(z, flag_get(z, sf) as i32 == 0 as i32);
        }
        250 => {
            cyc = cyc.wrapping_add(10);
            cond_jump(z, flag_get(z, sf) as i32 == 1 as i32);
        }
        16 => {
            cyc = cyc.wrapping_add(8);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = ((*z).c2rust_unnamed_0.c2rust_unnamed.b).wrapping_sub(1);
            cyc = cyc
                .wrapping_add(
                    cond_jr(
                        z,
                        (*z).c2rust_unnamed_0.c2rust_unnamed.b as i32
                            != 0 as i32,
                    ),
                );
        }
        24 => {
            cyc = cyc.wrapping_add(12);
            jr(z, nextb(z) as int8_t);
        }
        32 => {
            cyc = cyc.wrapping_add(7);
            cyc = cyc
                .wrapping_add(
                    cond_jr(z, flag_get(z, zf) as i32 == 0 as i32),
                );
        }
        40 => {
            cyc = cyc.wrapping_add(7);
            cyc = cyc
                .wrapping_add(
                    cond_jr(z, flag_get(z, zf) as i32 == 1 as i32),
                );
        }
        48 => {
            cyc = cyc.wrapping_add(7);
            cyc = cyc
                .wrapping_add(
                    cond_jr(z, flag_get(z, cf) as i32 == 0 as i32),
                );
        }
        56 => {
            cyc = cyc.wrapping_add(7);
            cyc = cyc
                .wrapping_add(
                    cond_jr(z, flag_get(z, cf) as i32 == 1 as i32),
                );
        }
        233 => {
            cyc = cyc.wrapping_add(4);
            (*z).pc = (*z).c2rust_unnamed_2.hl;
        }
        205 => {
            cyc = cyc.wrapping_add(17);
            call(z, nextw(z));
        }
        196 => {
            cyc = cyc.wrapping_add(10);
            cyc = cyc
                .wrapping_add(
                    cond_call(z, flag_get(z, zf) as i32 == 0 as i32),
                );
        }
        204 => {
            cyc = cyc.wrapping_add(10);
            cyc = cyc
                .wrapping_add(
                    cond_call(z, flag_get(z, zf) as i32 == 1 as i32),
                );
        }
        212 => {
            cyc = cyc.wrapping_add(10);
            cyc = cyc
                .wrapping_add(
                    cond_call(z, flag_get(z, cf) as i32 == 0 as i32),
                );
        }
        220 => {
            cyc = cyc.wrapping_add(10);
            cyc = cyc
                .wrapping_add(
                    cond_call(z, flag_get(z, cf) as i32 == 1 as i32),
                );
        }
        228 => {
            cyc = cyc.wrapping_add(10);
            cyc = cyc
                .wrapping_add(
                    cond_call(z, flag_get(z, pf) as i32 == 0 as i32),
                );
        }
        236 => {
            cyc = cyc.wrapping_add(10);
            cyc = cyc
                .wrapping_add(
                    cond_call(z, flag_get(z, pf) as i32 == 1 as i32),
                );
        }
        244 => {
            cyc = cyc.wrapping_add(10);
            cyc = cyc
                .wrapping_add(
                    cond_call(z, flag_get(z, sf) as i32 == 0 as i32),
                );
        }
        252 => {
            cyc = cyc.wrapping_add(10);
            cyc = cyc
                .wrapping_add(
                    cond_call(z, flag_get(z, sf) as i32 == 1 as i32),
                );
        }
        201 => {
            cyc = cyc.wrapping_add(10);
            ret(z);
        }
        192 => {
            cyc = cyc.wrapping_add(5);
            cyc = cyc
                .wrapping_add(
                    cond_ret(z, flag_get(z, zf) as i32 == 0 as i32),
                );
        }
        200 => {
            cyc = cyc.wrapping_add(5);
            cyc = cyc
                .wrapping_add(
                    cond_ret(z, flag_get(z, zf) as i32 == 1 as i32),
                );
        }
        208 => {
            cyc = cyc.wrapping_add(5);
            cyc = cyc
                .wrapping_add(
                    cond_ret(z, flag_get(z, cf) as i32 == 0 as i32),
                );
        }
        216 => {
            cyc = cyc.wrapping_add(5);
            cyc = cyc
                .wrapping_add(
                    cond_ret(z, flag_get(z, cf) as i32 == 1 as i32),
                );
        }
        224 => {
            cyc = cyc.wrapping_add(5);
            cyc = cyc
                .wrapping_add(
                    cond_ret(z, flag_get(z, pf) as i32 == 0 as i32),
                );
        }
        232 => {
            cyc = cyc.wrapping_add(5);
            cyc = cyc
                .wrapping_add(
                    cond_ret(z, flag_get(z, pf) as i32 == 1 as i32),
                );
        }
        240 => {
            cyc = cyc.wrapping_add(5);
            cyc = cyc
                .wrapping_add(
                    cond_ret(z, flag_get(z, sf) as i32 == 0 as i32),
                );
        }
        248 => {
            cyc = cyc.wrapping_add(5);
            cyc = cyc
                .wrapping_add(
                    cond_ret(z, flag_get(z, sf) as i32 == 1 as i32),
                );
        }
        199 => {
            cyc = cyc.wrapping_add(11);
            call(z, 0 as i32 as uint16_t);
        }
        207 => {
            cyc = cyc.wrapping_add(11);
            call(z, 0x8 as i32 as uint16_t);
        }
        215 => {
            cyc = cyc.wrapping_add(11);
            call(z, 0x10 as i32 as uint16_t);
        }
        223 => {
            cyc = cyc.wrapping_add(11);
            call(z, 0x18 as i32 as uint16_t);
        }
        231 => {
            cyc = cyc.wrapping_add(11);
            call(z, 0x20 as i32 as uint16_t);
        }
        239 => {
            cyc = cyc.wrapping_add(11);
            call(z, 0x28 as i32 as uint16_t);
        }
        247 => {
            cyc = cyc.wrapping_add(11);
            call(z, 0x30 as i32 as uint16_t);
        }
        255 => {
            cyc = cyc.wrapping_add(11);
            call(z, 0x38 as i32 as uint16_t);
        }
        197 => {
            cyc = cyc.wrapping_add(11);
            pushw(z, (*z).c2rust_unnamed_0.bc);
        }
        213 => {
            cyc = cyc.wrapping_add(11);
            pushw(z, (*z).c2rust_unnamed_1.de);
        }
        229 => {
            cyc = cyc.wrapping_add(11);
            pushw(z, (*z).c2rust_unnamed_2.hl);
        }
        245 => {
            cyc = cyc.wrapping_add(11);
            pushw(z, (*z).c2rust_unnamed.af);
        }
        193 => {
            cyc = cyc.wrapping_add(10);
            (*z).c2rust_unnamed_0.bc = popw(z);
        }
        209 => {
            cyc = cyc.wrapping_add(10);
            (*z).c2rust_unnamed_1.de = popw(z);
        }
        225 => {
            cyc = cyc.wrapping_add(10);
            (*z).c2rust_unnamed_2.hl = popw(z);
        }
        241 => {
            cyc = cyc.wrapping_add(10);
            (*z).c2rust_unnamed.af = popw(z);
        }
        219 => {
            cyc = cyc.wrapping_add(11);
            let port: uint16_t = (nextb(z) as i32
                | ((*z).c2rust_unnamed.c2rust_unnamed.a as i32)
                    << 8 as i32) as uint16_t;
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (*z).internal_port_in(port);
            (*z).mem_ptr = (port as i32 + 1 as i32) as uint16_t;
        }
        211 => {
            cyc = cyc.wrapping_add(11);
            let port_0: uint16_t = (nextb(z) as i32
                | ((*z).c2rust_unnamed.c2rust_unnamed.a as i32)
                    << 8 as i32) as uint16_t;
            (*z).internal_port_out(port_0, (*z).c2rust_unnamed.c2rust_unnamed.a);
            (*z)
                .mem_ptr = (port_0 as i32 + 1 as i32
                & 0xff as i32
                | ((*z).c2rust_unnamed.c2rust_unnamed.a as i32)
                    << 8 as i32) as uint16_t;
        }
        8 => {
            cyc = cyc.wrapping_add(4);
            let mut af: uint16_t = (*z).c2rust_unnamed.af;
            (*z).c2rust_unnamed.af = (*z).c2rust_unnamed_3.a_f_;
            (*z).c2rust_unnamed_3.a_f_ = af;
        }
        217 => {
            cyc = cyc.wrapping_add(4);
            let mut bc: uint16_t = (*z).c2rust_unnamed_0.bc;
            let mut de_0: uint16_t = (*z).c2rust_unnamed_1.de;
            let mut hl: uint16_t = (*z).c2rust_unnamed_2.hl;
            (*z).c2rust_unnamed_0.bc = (*z).c2rust_unnamed_4.b_c_;
            (*z).c2rust_unnamed_1.de = (*z).c2rust_unnamed_5.d_e_;
            (*z).c2rust_unnamed_2.hl = (*z).c2rust_unnamed_6.h_l_;
            (*z).c2rust_unnamed_4.b_c_ = bc;
            (*z).c2rust_unnamed_5.d_e_ = de_0;
            (*z).c2rust_unnamed_6.h_l_ = hl;
        }
        203 => {
            cyc = cyc.wrapping_add(0);
            cyc = cyc.wrapping_add(exec_opcode_cb(z, nextb(z)));
        }
        237 => {
            cyc = cyc.wrapping_add(0);
            cyc = cyc.wrapping_add(exec_opcode_ed(z, nextb(z)));
        }
        221 => {
            cyc = cyc.wrapping_add(0);
            cyc = cyc.wrapping_add(exec_opcode_ddfd(z, nextb(z), &mut (*z).ix));
        }
        253 => {
            cyc = cyc.wrapping_add(0);
            cyc = cyc.wrapping_add(exec_opcode_ddfd(z, nextb(z), &mut (*z).iy));
        }
        _ => {}
    }
    return cyc;
}
unsafe fn exec_opcode_ddfd(
    z: *mut Z80,
    mut opcode: uint8_t,
    iz: *mut uint16_t,
) -> u32 {
    let mut cyc: u32 = 0;
    inc_r(z);
    match opcode as i32 {
        225 => {
            cyc = cyc.wrapping_add(14);
            *iz = popw(z);
        }
        229 => {
            cyc = cyc.wrapping_add(15);
            pushw(z, *iz);
        }
        233 => {
            cyc = cyc.wrapping_add(8);
            jump(z, *iz);
        }
        9 => {
            cyc = cyc.wrapping_add(15);
            addiz(z, iz, (*z).c2rust_unnamed_0.bc);
        }
        25 => {
            cyc = cyc.wrapping_add(15);
            addiz(z, iz, (*z).c2rust_unnamed_1.de);
        }
        41 => {
            cyc = cyc.wrapping_add(15);
            addiz(z, iz, *iz);
        }
        57 => {
            cyc = cyc.wrapping_add(15);
            addiz(z, iz, (*z).sp);
        }
        132 => {
            cyc = cyc.wrapping_add(8);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*iz as i32 >> 8 as i32) as uint32_t,
                0 as i32 != 0,
            );
        }
        133 => {
            cyc = cyc.wrapping_add(8);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*iz as i32 & 0xff as i32) as uint32_t,
                0 as i32 != 0,
            );
        }
        140 => {
            cyc = cyc.wrapping_add(8);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*iz as i32 >> 8 as i32) as uint32_t,
                flag_get(z, cf),
            );
        }
        141 => {
            cyc = cyc.wrapping_add(8);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*iz as i32 & 0xff as i32) as uint32_t,
                flag_get(z, cf),
            );
        }
        134 => {
            cyc = cyc.wrapping_add(19);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                rb(z, displace(z, *iz, nextb(z) as int8_t)) as uint32_t,
                0 as i32 != 0,
            );
        }
        142 => {
            cyc = cyc.wrapping_add(19);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                rb(z, displace(z, *iz, nextb(z) as int8_t)) as uint32_t,
                flag_get(z, cf),
            );
        }
        150 => {
            cyc = cyc.wrapping_add(19);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                rb(z, displace(z, *iz, nextb(z) as int8_t)) as uint32_t,
                0 as i32 != 0,
            );
        }
        158 => {
            cyc = cyc.wrapping_add(19);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                rb(z, displace(z, *iz, nextb(z) as int8_t)) as uint32_t,
                flag_get(z, cf),
            );
        }
        148 => {
            cyc = cyc.wrapping_add(8);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*iz as i32 >> 8 as i32) as uint32_t,
                0 as i32 != 0,
            );
        }
        149 => {
            cyc = cyc.wrapping_add(8);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*iz as i32 & 0xff as i32) as uint32_t,
                0 as i32 != 0,
            );
        }
        156 => {
            cyc = cyc.wrapping_add(8);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*iz as i32 >> 8 as i32) as uint32_t,
                flag_get(z, cf),
            );
        }
        157 => {
            cyc = cyc.wrapping_add(8);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*iz as i32 & 0xff as i32) as uint32_t,
                flag_get(z, cf),
            );
        }
        166 => {
            cyc = cyc.wrapping_add(19);
            land(z, rb(z, displace(z, *iz, nextb(z) as int8_t)));
        }
        164 => {
            cyc = cyc.wrapping_add(8);
            land(z, (*iz as i32 >> 8 as i32) as uint8_t);
        }
        165 => {
            cyc = cyc.wrapping_add(8);
            land(z, (*iz as i32 & 0xff as i32) as uint8_t);
        }
        174 => {
            cyc = cyc.wrapping_add(19);
            lxor(z, rb(z, displace(z, *iz, nextb(z) as int8_t)));
        }
        172 => {
            cyc = cyc.wrapping_add(8);
            lxor(z, (*iz as i32 >> 8 as i32) as uint8_t);
        }
        173 => {
            cyc = cyc.wrapping_add(8);
            lxor(z, (*iz as i32 & 0xff as i32) as uint8_t);
        }
        182 => {
            cyc = cyc.wrapping_add(19);
            lor(z, rb(z, displace(z, *iz, nextb(z) as int8_t)));
        }
        180 => {
            cyc = cyc.wrapping_add(8);
            lor(z, (*iz as i32 >> 8 as i32) as uint8_t);
        }
        181 => {
            cyc = cyc.wrapping_add(8);
            lor(z, (*iz as i32 & 0xff as i32) as uint8_t);
        }
        190 => {
            cyc = cyc.wrapping_add(19);
            cp(z, rb(z, displace(z, *iz, nextb(z) as int8_t)) as uint32_t);
        }
        188 => {
            cyc = cyc.wrapping_add(8);
            cp(z, (*iz as i32 >> 8 as i32) as uint32_t);
        }
        189 => {
            cyc = cyc.wrapping_add(8);
            cp(z, (*iz as i32 & 0xff as i32) as uint32_t);
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
            let mut addr: uint16_t = displace(z, *iz, nextb(z) as int8_t);
            wb(z, addr, inc(z, rb(z, addr)));
        }
        53 => {
            cyc = cyc.wrapping_add(23);
            let mut addr_0: uint16_t = displace(z, *iz, nextb(z) as int8_t);
            wb(z, addr_0, dec(z, rb(z, addr_0)));
        }
        36 => {
            cyc = cyc.wrapping_add(8);
            *iz = (*iz as i32 & 0xff as i32
                | (inc(z, (*iz as i32 >> 8 as i32) as uint8_t)
                    as i32) << 8 as i32) as uint16_t;
        }
        37 => {
            cyc = cyc.wrapping_add(8);
            *iz = (*iz as i32 & 0xff as i32
                | (dec(z, (*iz as i32 >> 8 as i32) as uint8_t)
                    as i32) << 8 as i32) as uint16_t;
        }
        44 => {
            cyc = cyc.wrapping_add(8);
            *iz = ((*iz as i32 >> 8 as i32) << 8 as i32
                | inc(z, (*iz as i32 & 0xff as i32) as uint8_t)
                    as i32) as uint16_t;
        }
        45 => {
            cyc = cyc.wrapping_add(8);
            *iz = ((*iz as i32 >> 8 as i32) << 8 as i32
                | dec(z, (*iz as i32 & 0xff as i32) as uint8_t)
                    as i32) as uint16_t;
        }
        42 => {
            cyc = cyc.wrapping_add(20);
            *iz = rw(z, nextw(z));
        }
        34 => {
            cyc = cyc.wrapping_add(20);
            ww(z, nextw(z), *iz);
        }
        33 => {
            cyc = cyc.wrapping_add(14);
            *iz = nextw(z);
        }
        54 => {
            cyc = cyc.wrapping_add(19);
            let mut addr_1: uint16_t = displace(z, *iz, nextb(z) as int8_t);
            wb(z, addr_1, nextb(z));
        }
        112 => {
            cyc = cyc.wrapping_add(19);
            wb(
                z,
                displace(z, *iz, nextb(z) as int8_t),
                (*z).c2rust_unnamed_0.c2rust_unnamed.b,
            );
        }
        113 => {
            cyc = cyc.wrapping_add(19);
            wb(
                z,
                displace(z, *iz, nextb(z) as int8_t),
                (*z).c2rust_unnamed_0.c2rust_unnamed.c,
            );
        }
        114 => {
            cyc = cyc.wrapping_add(19);
            wb(
                z,
                displace(z, *iz, nextb(z) as int8_t),
                (*z).c2rust_unnamed_1.c2rust_unnamed.d,
            );
        }
        115 => {
            cyc = cyc.wrapping_add(19);
            wb(
                z,
                displace(z, *iz, nextb(z) as int8_t),
                (*z).c2rust_unnamed_1.c2rust_unnamed.e,
            );
        }
        116 => {
            cyc = cyc.wrapping_add(19);
            wb(
                z,
                displace(z, *iz, nextb(z) as int8_t),
                (*z).c2rust_unnamed_2.c2rust_unnamed.h,
            );
        }
        117 => {
            cyc = cyc.wrapping_add(19);
            wb(
                z,
                displace(z, *iz, nextb(z) as int8_t),
                (*z).c2rust_unnamed_2.c2rust_unnamed.l,
            );
        }
        119 => {
            cyc = cyc.wrapping_add(19);
            wb(
                z,
                displace(z, *iz, nextb(z) as int8_t),
                (*z).c2rust_unnamed.c2rust_unnamed.a,
            );
        }
        70 => {
            cyc = cyc.wrapping_add(19);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = rb(z, displace(z, *iz, nextb(z) as int8_t));
        }
        78 => {
            cyc = cyc.wrapping_add(19);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = rb(z, displace(z, *iz, nextb(z) as int8_t));
        }
        86 => {
            cyc = cyc.wrapping_add(19);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = rb(z, displace(z, *iz, nextb(z) as int8_t));
        }
        94 => {
            cyc = cyc.wrapping_add(19);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = rb(z, displace(z, *iz, nextb(z) as int8_t));
        }
        102 => {
            cyc = cyc.wrapping_add(19);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .h = rb(z, displace(z, *iz, nextb(z) as int8_t));
        }
        110 => {
            cyc = cyc.wrapping_add(19);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .l = rb(z, displace(z, *iz, nextb(z) as int8_t));
        }
        126 => {
            cyc = cyc.wrapping_add(19);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = rb(z, displace(z, *iz, nextb(z) as int8_t));
        }
        68 => {
            cyc = cyc.wrapping_add(8);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = (*iz as i32 >> 8 as i32) as uint8_t;
        }
        76 => {
            cyc = cyc.wrapping_add(8);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = (*iz as i32 >> 8 as i32) as uint8_t;
        }
        84 => {
            cyc = cyc.wrapping_add(8);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = (*iz as i32 >> 8 as i32) as uint8_t;
        }
        92 => {
            cyc = cyc.wrapping_add(8);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = (*iz as i32 >> 8 as i32) as uint8_t;
        }
        124 => {
            cyc = cyc.wrapping_add(8);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (*iz as i32 >> 8 as i32) as uint8_t;
        }
        69 => {
            cyc = cyc.wrapping_add(8);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = (*iz as i32 & 0xff as i32) as uint8_t;
        }
        77 => {
            cyc = cyc.wrapping_add(8);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = (*iz as i32 & 0xff as i32) as uint8_t;
        }
        85 => {
            cyc = cyc.wrapping_add(8);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = (*iz as i32 & 0xff as i32) as uint8_t;
        }
        93 => {
            cyc = cyc.wrapping_add(8);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = (*iz as i32 & 0xff as i32) as uint8_t;
        }
        125 => {
            cyc = cyc.wrapping_add(8);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (*iz as i32 & 0xff as i32) as uint8_t;
        }
        96 => {
            cyc = cyc.wrapping_add(8);
            *iz = (*iz as i32 & 0xff as i32
                | ((*z).c2rust_unnamed_0.c2rust_unnamed.b as i32)
                    << 8 as i32) as uint16_t;
        }
        97 => {
            cyc = cyc.wrapping_add(8);
            *iz = (*iz as i32 & 0xff as i32
                | ((*z).c2rust_unnamed_0.c2rust_unnamed.c as i32)
                    << 8 as i32) as uint16_t;
        }
        98 => {
            cyc = cyc.wrapping_add(8);
            *iz = (*iz as i32 & 0xff as i32
                | ((*z).c2rust_unnamed_1.c2rust_unnamed.d as i32)
                    << 8 as i32) as uint16_t;
        }
        99 => {
            cyc = cyc.wrapping_add(8);
            *iz = (*iz as i32 & 0xff as i32
                | ((*z).c2rust_unnamed_1.c2rust_unnamed.e as i32)
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
                | ((*z).c2rust_unnamed.c2rust_unnamed.a as i32)
                    << 8 as i32) as uint16_t;
        }
        38 => {
            cyc = cyc.wrapping_add(11);
            *iz = (*iz as i32 & 0xff as i32
                | (nextb(z) as i32) << 8 as i32) as uint16_t;
        }
        104 => {
            cyc = cyc.wrapping_add(8);
            *iz = ((*iz as i32 >> 8 as i32) << 8 as i32
                | (*z).c2rust_unnamed_0.c2rust_unnamed.b as i32) as uint16_t;
        }
        105 => {
            cyc = cyc.wrapping_add(8);
            *iz = ((*iz as i32 >> 8 as i32) << 8 as i32
                | (*z).c2rust_unnamed_0.c2rust_unnamed.c as i32) as uint16_t;
        }
        106 => {
            cyc = cyc.wrapping_add(8);
            *iz = ((*iz as i32 >> 8 as i32) << 8 as i32
                | (*z).c2rust_unnamed_1.c2rust_unnamed.d as i32) as uint16_t;
        }
        107 => {
            cyc = cyc.wrapping_add(8);
            *iz = ((*iz as i32 >> 8 as i32) << 8 as i32
                | (*z).c2rust_unnamed_1.c2rust_unnamed.e as i32) as uint16_t;
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
                | (*z).c2rust_unnamed.c2rust_unnamed.a as i32) as uint16_t;
        }
        46 => {
            cyc = cyc.wrapping_add(11);
            *iz = ((*iz as i32 >> 8 as i32) << 8 as i32
                | nextb(z) as i32) as uint16_t;
        }
        249 => {
            cyc = cyc.wrapping_add(10);
            (*z).sp = *iz;
        }
        227 => {
            cyc = cyc.wrapping_add(23);
            let val: uint16_t = rw(z, (*z).sp);
            ww(z, (*z).sp, *iz);
            *iz = val;
            (*z).mem_ptr = val;
        }
        203 => {
            let mut addr_2: uint16_t = displace(z, *iz, nextb(z) as int8_t);
            let mut op: uint8_t = nextb(z);
            cyc = cyc.wrapping_add(exec_opcode_dcb(z, op, addr_2));
        }
        _ => {
            cyc = cyc
                .wrapping_add(
                    (4_u32)
                        .wrapping_add(exec_opcode(z, opcode)),
                );
            (*z)
                .r = ((*z).r as i32 & 0x80 as i32
                | (*z).r as i32 - 1 as i32 & 0x7f as i32)
                as uint8_t;
        }
    }
    return cyc;
}
unsafe fn exec_opcode_cb(z: *mut Z80, mut opcode: uint8_t) -> u32 {
    let mut cyc: u32 = 8;
    inc_r(z);
    let mut x_: uint8_t = (opcode as i32 >> 6 as i32 & 3 as i32)
        as uint8_t;
    let mut y_: uint8_t = (opcode as i32 >> 3 as i32 & 7 as i32)
        as uint8_t;
    let mut z_: uint8_t = (opcode as i32 & 7 as i32) as uint8_t;
    let mut hl: uint8_t = 0 as i32 as uint8_t;
    let mut reg: *mut uint8_t = 0 as *mut uint8_t;
    match z_ as i32 {
        0 => {
            reg = &mut (*z).c2rust_unnamed_0.c2rust_unnamed.b;
        }
        1 => {
            reg = &mut (*z).c2rust_unnamed_0.c2rust_unnamed.c;
        }
        2 => {
            reg = &mut (*z).c2rust_unnamed_1.c2rust_unnamed.d;
        }
        3 => {
            reg = &mut (*z).c2rust_unnamed_1.c2rust_unnamed.e;
        }
        4 => {
            reg = &mut (*z).c2rust_unnamed_2.c2rust_unnamed.h;
        }
        5 => {
            reg = &mut (*z).c2rust_unnamed_2.c2rust_unnamed.l;
        }
        6 => {
            hl = rb(z, (*z).c2rust_unnamed_2.hl);
            reg = &mut hl;
        }
        7 => {
            reg = &mut (*z).c2rust_unnamed.c2rust_unnamed.a;
        }
        _ => {}
    }
    match x_ as i32 {
        0 => {
            match y_ as i32 {
                0 => {
                    *reg = cb_rlc(z, *reg);
                }
                1 => {
                    *reg = cb_rrc(z, *reg);
                }
                2 => {
                    *reg = cb_rl(z, *reg);
                }
                3 => {
                    *reg = cb_rr(z, *reg);
                }
                4 => {
                    *reg = cb_sla(z, *reg);
                }
                5 => {
                    *reg = cb_sra(z, *reg);
                }
                6 => {
                    *reg = cb_sll(z, *reg);
                }
                7 => {
                    *reg = cb_srl(z, *reg);
                }
                _ => {}
            }
        }
        1 => {
            cb_bit(z, *reg, y_);
            if z_ as i32 == 6 as i32 {
                flag_set(
                    z,
                    yf,
                    (*z).mem_ptr as i32 >> 8 as i32 >> 5 as i32
                        & 1 as i32 != 0,
                );
                flag_set(
                    z,
                    xf,
                    (*z).mem_ptr as i32 >> 8 as i32 >> 3 as i32
                        & 1 as i32 != 0,
                );
                cyc = cyc.wrapping_add(4);
            } else {
                flag_set(
                    z,
                    yf,
                    *reg as i32 >> 5 as i32 & 1 as i32 != 0,
                );
                flag_set(
                    z,
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
        wb(z, (*z).c2rust_unnamed_2.hl, hl);
        cyc = cyc.wrapping_add(7);
    }
    return cyc;
}
unsafe fn exec_opcode_dcb(
    z: *mut Z80,
    mut opcode: uint8_t,
    mut addr: uint16_t,
) -> u32 {
    let mut cyc: u32 = 0;
    let mut val: uint8_t = rb(z, addr);
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
                    result = cb_rlc(z, val);
                }
                1 => {
                    result = cb_rrc(z, val);
                }
                2 => {
                    result = cb_rl(z, val);
                }
                3 => {
                    result = cb_rr(z, val);
                }
                4 => {
                    result = cb_sla(z, val);
                }
                5 => {
                    result = cb_sra(z, val);
                }
                6 => {
                    result = cb_sll(z, val);
                }
                7 => {
                    result = cb_srl(z, val);
                }
                _ => {}
            }
        }
        1 => {
            result = cb_bit(z, val, y_);
            flag_set(
                z,
                yf,
                addr as i32 >> 8 as i32 >> 5 as i32
                    & 1 as i32 != 0,
            );
            flag_set(
                z,
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
                (*z).c2rust_unnamed_0.c2rust_unnamed.b = result;
            }
            1 => {
                (*z).c2rust_unnamed_0.c2rust_unnamed.c = result;
            }
            2 => {
                (*z).c2rust_unnamed_1.c2rust_unnamed.d = result;
            }
            3 => {
                (*z).c2rust_unnamed_1.c2rust_unnamed.e = result;
            }
            4 => {
                (*z).c2rust_unnamed_2.c2rust_unnamed.h = result;
            }
            5 => {
                (*z).c2rust_unnamed_2.c2rust_unnamed.l = result;
            }
            6 => {
                wb(z, (*z).c2rust_unnamed_2.hl, result);
            }
            7 => {
                (*z).c2rust_unnamed.c2rust_unnamed.a = result;
            }
            _ => {}
        }
    }
    if x_ as i32 == 1 as i32 {
        cyc = cyc.wrapping_add(20);
    } else {
        wb(z, addr, result);
        cyc = cyc.wrapping_add(23);
    }
    return cyc;
}
unsafe fn exec_opcode_ed(z: *mut Z80, mut opcode: uint8_t) -> u32 {
    let mut cyc: u32 = 0;
    inc_r(z);
    match opcode as i32 {
        71 => {
            cyc = cyc.wrapping_add(9);
            (*z).i = (*z).c2rust_unnamed.c2rust_unnamed.a;
        }
        79 => {
            cyc = cyc.wrapping_add(9);
            (*z).r = (*z).c2rust_unnamed.c2rust_unnamed.a;
        }
        87 => {
            cyc = cyc.wrapping_add(9);
            (*z).c2rust_unnamed.c2rust_unnamed.a = (*z).i;
            flag_set(
                z,
                sf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 >> 7 as i32
                    != 0,
            );
            flag_set(
                z,
                zf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 == 0 as i32,
            );
            flag_set(z, hf, 0 as i32 != 0);
            flag_set(z, nf, 0 as i32 != 0);
            flag_set(z, pf, (*z).iff2);
        }
        95 => {
            cyc = cyc.wrapping_add(9);
            (*z).c2rust_unnamed.c2rust_unnamed.a = (*z).r;
            flag_set(
                z,
                sf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 >> 7 as i32
                    != 0,
            );
            flag_set(
                z,
                zf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as i32 == 0 as i32,
            );
            flag_set(z, hf, 0 as i32 != 0);
            flag_set(z, nf, 0 as i32 != 0);
            flag_set(z, pf, (*z).iff2);
        }
        69 | 85 | 93 | 101 | 109 | 117 | 125 => {
            cyc = cyc.wrapping_add(14);
            (*z).iff1 = (*z).iff2;
            ret(z);
        }
        77 => {
            cyc = cyc.wrapping_add(14);
            ret(z);
        }
        160 => {
            cyc = cyc.wrapping_add(16);
            ldi(z);
        }
        176 => {
            cyc = cyc.wrapping_add(16);
            ldi(z);
            if (*z).c2rust_unnamed_0.bc as i32 != 0 as i32 {
                (*z).pc = ((*z).pc as i32 - 2 as i32) as uint16_t;
                cyc = cyc.wrapping_add(5);
                (*z).mem_ptr = ((*z).pc as i32 + 1 as i32) as uint16_t;
            }
        }
        168 => {
            cyc = cyc.wrapping_add(16);
            ldd(z);
        }
        184 => {
            cyc = cyc.wrapping_add(16);
            ldd(z);
            if (*z).c2rust_unnamed_0.bc as i32 != 0 as i32 {
                (*z).pc = ((*z).pc as i32 - 2 as i32) as uint16_t;
                cyc = cyc.wrapping_add(5);
                (*z).mem_ptr = ((*z).pc as i32 + 1 as i32) as uint16_t;
            }
        }
        161 => {
            cyc = cyc.wrapping_add(16);
            cpi(z);
        }
        169 => {
            cyc = cyc.wrapping_add(16);
            cpd(z);
        }
        177 => {
            cyc = cyc.wrapping_add(16);
            cpi(z);
            if (*z).c2rust_unnamed_0.bc as i32 != 0 as i32
                && !flag_get(z, zf)
            {
                (*z).pc = ((*z).pc as i32 - 2 as i32) as uint16_t;
                cyc = cyc.wrapping_add(5);
                (*z).mem_ptr = ((*z).pc as i32 + 1 as i32) as uint16_t;
            } else {
                (*z)
                    .mem_ptr = ((*z).mem_ptr as i32 + 1 as i32)
                    as uint16_t;
            }
        }
        185 => {
            cyc = cyc.wrapping_add(16);
            cpd(z);
            if (*z).c2rust_unnamed_0.bc as i32 != 0 as i32
                && !flag_get(z, zf)
            {
                (*z).pc = ((*z).pc as i32 - 2 as i32) as uint16_t;
                cyc = cyc.wrapping_add(5);
            } else {
                (*z)
                    .mem_ptr = ((*z).mem_ptr as i32 + 1 as i32)
                    as uint16_t;
            }
        }
        64 => {
            cyc = cyc.wrapping_add(12);
            in_r_c(z, &mut (*z).c2rust_unnamed_0.c2rust_unnamed.b);
        }
        72 => {
            cyc = cyc.wrapping_add(12);
            in_r_c(z, &mut (*z).c2rust_unnamed_0.c2rust_unnamed.c);
        }
        80 => {
            cyc = cyc.wrapping_add(12);
            in_r_c(z, &mut (*z).c2rust_unnamed_1.c2rust_unnamed.d);
        }
        88 => {
            cyc = cyc.wrapping_add(12);
            in_r_c(z, &mut (*z).c2rust_unnamed_1.c2rust_unnamed.e);
        }
        96 => {
            cyc = cyc.wrapping_add(12);
            in_r_c(z, &mut (*z).c2rust_unnamed_2.c2rust_unnamed.h);
        }
        104 => {
            cyc = cyc.wrapping_add(12);
            in_r_c(z, &mut (*z).c2rust_unnamed_2.c2rust_unnamed.l);
        }
        112 => {
            cyc = cyc.wrapping_add(12);
            let mut val: uint8_t = 0;
            in_r_c(z, &mut val);
        }
        120 => {
            cyc = cyc.wrapping_add(12);
            in_r_c(z, &mut (*z).c2rust_unnamed.c2rust_unnamed.a);
            (*z)
                .mem_ptr = ((*z).c2rust_unnamed_0.bc as i32 + 1 as i32)
                as uint16_t;
        }
        162 => {
            cyc = cyc.wrapping_add(16);
            ini(z);
        }
        178 => {
            cyc = cyc.wrapping_add(16);
            ini(z);
            if (*z).c2rust_unnamed_0.c2rust_unnamed.b as i32 > 0 as i32 {
                (*z).pc = ((*z).pc as i32 - 2 as i32) as uint16_t;
                cyc = cyc.wrapping_add(5);
                (*z).mem_ptr = ((*z).pc as i32 + 1 as i32) as uint16_t;
            }
        }
        170 => {
            cyc = cyc.wrapping_add(16);
            ind(z);
        }
        186 => {
            cyc = cyc.wrapping_add(16);
            ind(z);
            if (*z).c2rust_unnamed_0.c2rust_unnamed.b as i32 > 0 as i32 {
                (*z).pc = ((*z).pc as i32 - 2 as i32) as uint16_t;
                cyc = cyc.wrapping_add(5);
                (*z).mem_ptr = ((*z).pc as i32 + 1 as i32) as uint16_t;
            }
        }
        121 => {
            cyc = cyc.wrapping_add(12);
            outc(z, (*z).c2rust_unnamed.c2rust_unnamed.a);
        }
        65 => {
            cyc = cyc.wrapping_add(12);
            outc(z, (*z).c2rust_unnamed_0.c2rust_unnamed.b);
        }
        73 => {
            cyc = cyc.wrapping_add(12);
            outc(z, (*z).c2rust_unnamed_0.c2rust_unnamed.c);
        }
        81 => {
            cyc = cyc.wrapping_add(12);
            outc(z, (*z).c2rust_unnamed_1.c2rust_unnamed.d);
        }
        89 => {
            cyc = cyc.wrapping_add(12);
            outc(z, (*z).c2rust_unnamed_1.c2rust_unnamed.e);
        }
        97 => {
            cyc = cyc.wrapping_add(12);
            outc(z, (*z).c2rust_unnamed_2.c2rust_unnamed.h);
        }
        105 => {
            cyc = cyc.wrapping_add(12);
            outc(z, (*z).c2rust_unnamed_2.c2rust_unnamed.l);
        }
        113 => {
            cyc = cyc.wrapping_add(12);
            outc(z, 0 as i32 as uint8_t);
        }
        163 => {
            cyc = cyc.wrapping_add(16);
            outi(z);
        }
        179 => {
            cyc = cyc.wrapping_add(16);
            outi(z);
            if (*z).c2rust_unnamed_0.c2rust_unnamed.b as i32 > 0 as i32 {
                (*z).pc = ((*z).pc as i32 - 2 as i32) as uint16_t;
                cyc = cyc.wrapping_add(5);
                (*z).mem_ptr = ((*z).pc as i32 + 1 as i32) as uint16_t;
            }
        }
        171 => {
            cyc = cyc.wrapping_add(16);
            outd(z);
        }
        187 => {
            cyc = cyc.wrapping_add(16);
            outd(z);
            if (*z).c2rust_unnamed_0.c2rust_unnamed.b as i32 > 0 as i32 {
                (*z).pc = ((*z).pc as i32 - 2 as i32) as uint16_t;
                cyc = cyc.wrapping_add(5);
                (*z).mem_ptr = ((*z).pc as i32 + 1 as i32) as uint16_t;
            }
        }
        66 => {
            cyc = cyc.wrapping_add(15);
            sbchl(z, (*z).c2rust_unnamed_0.bc);
        }
        82 => {
            cyc = cyc.wrapping_add(15);
            sbchl(z, (*z).c2rust_unnamed_1.de);
        }
        98 => {
            cyc = cyc.wrapping_add(15);
            sbchl(z, (*z).c2rust_unnamed_2.hl);
        }
        114 => {
            cyc = cyc.wrapping_add(15);
            sbchl(z, (*z).sp);
        }
        74 => {
            cyc = cyc.wrapping_add(15);
            adchl(z, (*z).c2rust_unnamed_0.bc);
        }
        90 => {
            cyc = cyc.wrapping_add(15);
            adchl(z, (*z).c2rust_unnamed_1.de);
        }
        106 => {
            cyc = cyc.wrapping_add(15);
            adchl(z, (*z).c2rust_unnamed_2.hl);
        }
        122 => {
            cyc = cyc.wrapping_add(15);
            adchl(z, (*z).sp);
        }
        67 => {
            cyc = cyc.wrapping_add(20);
            let addr: uint16_t = nextw(z);
            ww(z, addr, (*z).c2rust_unnamed_0.bc);
            (*z).mem_ptr = (addr as i32 + 1 as i32) as uint16_t;
        }
        83 => {
            cyc = cyc.wrapping_add(20);
            let addr_0: uint16_t = nextw(z);
            ww(z, addr_0, (*z).c2rust_unnamed_1.de);
            (*z).mem_ptr = (addr_0 as i32 + 1 as i32) as uint16_t;
        }
        99 => {
            cyc = cyc.wrapping_add(20);
            let addr_1: uint16_t = nextw(z);
            ww(z, addr_1, (*z).c2rust_unnamed_2.hl);
            (*z).mem_ptr = (addr_1 as i32 + 1 as i32) as uint16_t;
        }
        115 => {
            cyc = cyc.wrapping_add(20);
            let addr_2: uint16_t = nextw(z);
            ww(z, addr_2, (*z).sp);
            (*z).mem_ptr = (addr_2 as i32 + 1 as i32) as uint16_t;
        }
        75 => {
            cyc = cyc.wrapping_add(20);
            let addr_3: uint16_t = nextw(z);
            (*z).c2rust_unnamed_0.bc = rw(z, addr_3);
            (*z).mem_ptr = (addr_3 as i32 + 1 as i32) as uint16_t;
        }
        91 => {
            cyc = cyc.wrapping_add(20);
            let addr_4: uint16_t = nextw(z);
            (*z).c2rust_unnamed_1.de = rw(z, addr_4);
            (*z).mem_ptr = (addr_4 as i32 + 1 as i32) as uint16_t;
        }
        107 => {
            cyc = cyc.wrapping_add(20);
            let addr_5: uint16_t = nextw(z);
            (*z).c2rust_unnamed_2.hl = rw(z, addr_5);
            (*z).mem_ptr = (addr_5 as i32 + 1 as i32) as uint16_t;
        }
        123 => {
            cyc = cyc.wrapping_add(20);
            let addr_6: uint16_t = nextw(z);
            (*z).sp = rw(z, addr_6);
            (*z).mem_ptr = (addr_6 as i32 + 1 as i32) as uint16_t;
        }
        68 | 84 | 100 | 116 | 76 | 92 | 108 | 124 => {
            cyc = cyc.wrapping_add(8);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                0 as i32 as uint32_t,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                0 as i32 != 0,
            );
        }
        70 | 102 => {
            cyc = cyc.wrapping_add(8);
            (*z).interrupt_mode = 0 as i32 as uint8_t;
        }
        86 | 118 => {
            cyc = cyc.wrapping_add(8);
            (*z).interrupt_mode = 1 as i32 as uint8_t;
        }
        94 | 126 => {
            cyc = cyc.wrapping_add(8);
            (*z).interrupt_mode = 2 as i32 as uint8_t;
        }
        103 => {
            cyc = cyc.wrapping_add(18);
            let mut a: uint8_t = (*z).c2rust_unnamed.c2rust_unnamed.a;
            let mut val_0: uint8_t = rb(z, (*z).c2rust_unnamed_2.hl);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (a as i32 & 0xf0 as i32
                | val_0 as i32 & 0xf as i32) as uint8_t;
            wb(
                z,
                (*z).c2rust_unnamed_2.hl,
                (val_0 as i32 >> 4 as i32
                    | (a as i32) << 4 as i32) as uint8_t,
            );
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .f = (f_szpxy[(*z).c2rust_unnamed.c2rust_unnamed.a as usize]
                as i32 | flag_val(cf, flag_get(z, cf)) as i32
                | flag_val(nf, 0 as i32 != 0) as i32
                | flag_val(hf, 0 as i32 != 0) as i32) as uint8_t;
            (*z)
                .mem_ptr = ((*z).c2rust_unnamed_2.hl as i32 + 1 as i32)
                as uint16_t;
        }
        111 => {
            cyc = cyc.wrapping_add(18);
            let mut a_0: uint8_t = (*z).c2rust_unnamed.c2rust_unnamed.a;
            let mut val_1: uint8_t = rb(z, (*z).c2rust_unnamed_2.hl);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (a_0 as i32 & 0xf0 as i32
                | val_1 as i32 >> 4 as i32) as uint8_t;
            wb(
                z,
                (*z).c2rust_unnamed_2.hl,
                ((val_1 as i32) << 4 as i32
                    | a_0 as i32 & 0xf as i32) as uint8_t,
            );
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .f = (f_szpxy[(*z).c2rust_unnamed.c2rust_unnamed.a as usize]
                as i32 | flag_val(cf, flag_get(z, cf)) as i32
                | flag_val(nf, 0 as i32 != 0) as i32
                | flag_val(hf, 0 as i32 != 0) as i32) as uint8_t;
            (*z)
                .mem_ptr = ((*z).c2rust_unnamed_2.hl as i32 + 1 as i32)
                as uint16_t;
        }
        _ => {}
    }
    return cyc;
}
