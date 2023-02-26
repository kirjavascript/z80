use ::libc;
use ::c2rust_bitfields;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const Z80_ASSERT: C2RustUnnamed = 2;
pub const Z80_PULSE: C2RustUnnamed = 1;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct z80 {
    pub read_byte: Option::<
        unsafe extern "C" fn(*mut libc::c_void, uint16_t) -> uint8_t,
    >,
    pub write_byte: Option::<
        unsafe extern "C" fn(*mut libc::c_void, uint16_t, uint8_t) -> (),
    >,
    pub port_in: Option::<unsafe extern "C" fn(*mut z80, uint16_t) -> uint8_t>,
    pub port_out: Option::<unsafe extern "C" fn(*mut z80, uint16_t, uint8_t) -> ()>,
    pub userdata: *mut libc::c_void,
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
    #[bitfield(name = "iff1", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "iff2", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "halted", ty = "bool", bits = "2..=2")]
    pub iff1_iff2_halted: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub h_l_: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub l_: uint8_t,
    pub h_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub d_e_: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub e_: uint8_t,
    pub d_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub b_c_: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub c_: uint8_t,
    pub b_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub c2rust_unnamed: C2RustUnnamed_7,
    pub a_f_: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub f_: uint8_t,
    pub a_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub c2rust_unnamed: C2RustUnnamed_9,
    pub hl: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub l: uint8_t,
    pub h: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub c2rust_unnamed: C2RustUnnamed_11,
    pub de: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub e: uint8_t,
    pub d: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub c2rust_unnamed: C2RustUnnamed_13,
    pub bc: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub c: uint8_t,
    pub b: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub c2rust_unnamed: C2RustUnnamed_15,
    pub af: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub f: uint8_t,
    pub a: uint8_t,
}
pub type z80_flagbit = libc::c_uint;
pub const sf: z80_flagbit = 7;
pub const zf: z80_flagbit = 6;
pub const yf: z80_flagbit = 5;
pub const hf: z80_flagbit = 4;
pub const xf: z80_flagbit = 3;
pub const pf: z80_flagbit = 2;
pub const nf: z80_flagbit = 1;
pub const cf: z80_flagbit = 0;
static mut f_szpxy: [uint8_t; 256] = [
    0x44 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
];
#[inline]
unsafe extern "C" fn flag_val(mut bit: z80_flagbit, mut cond: bool) -> uint8_t {
    return ((cond as libc::c_int) << bit as libc::c_uint) as uint8_t;
}
#[inline]
unsafe extern "C" fn flag_get(z: *mut z80, mut bit: z80_flagbit) -> bool {
    return (*z).c2rust_unnamed.c2rust_unnamed.f as libc::c_int
        & (1 as libc::c_int) << bit as libc::c_uint != 0;
}
#[inline]
unsafe extern "C" fn flag_set(z: *mut z80, mut bit: z80_flagbit, mut val: bool) {
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as libc::c_int
        & !((1 as libc::c_int) << bit as libc::c_uint)) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as libc::c_int
        | (val as libc::c_int) << bit as libc::c_uint) as uint8_t;
}
#[inline]
unsafe extern "C" fn rb(z: *mut z80, mut addr: uint16_t) -> uint8_t {
    return ((*z).read_byte).expect("non-null function pointer")((*z).userdata, addr);
}
#[inline]
unsafe extern "C" fn wb(z: *mut z80, mut addr: uint16_t, mut val: uint8_t) {
    ((*z).write_byte).expect("non-null function pointer")((*z).userdata, addr, val);
}
#[inline]
unsafe extern "C" fn rw(z: *mut z80, mut addr: uint16_t) -> uint16_t {
    return ((((*z).read_byte)
        .expect(
            "non-null function pointer",
        )((*z).userdata, (addr as libc::c_int + 1 as libc::c_int) as uint16_t)
        as libc::c_int) << 8 as libc::c_int
        | ((*z).read_byte).expect("non-null function pointer")((*z).userdata, addr)
            as libc::c_int) as uint16_t;
}
#[inline]
unsafe extern "C" fn ww(z: *mut z80, mut addr: uint16_t, mut val: uint16_t) {
    ((*z).write_byte)
        .expect(
            "non-null function pointer",
        )((*z).userdata, addr, (val as libc::c_int & 0xff as libc::c_int) as uint8_t);
    ((*z).write_byte)
        .expect(
            "non-null function pointer",
        )(
        (*z).userdata,
        (addr as libc::c_int + 1 as libc::c_int) as uint16_t,
        (val as libc::c_int >> 8 as libc::c_int) as uint8_t,
    );
}
#[inline]
unsafe extern "C" fn pushw(z: *mut z80, mut val: uint16_t) {
    (*z).sp = ((*z).sp as libc::c_int - 2 as libc::c_int) as uint16_t;
    ww(z, (*z).sp, val);
}
#[inline]
unsafe extern "C" fn popw(z: *mut z80) -> uint16_t {
    (*z).sp = ((*z).sp as libc::c_int + 2 as libc::c_int) as uint16_t;
    return rw(z, ((*z).sp as libc::c_int - 2 as libc::c_int) as uint16_t);
}
#[inline]
unsafe extern "C" fn nextb(z: *mut z80) -> uint8_t {
    let fresh0 = (*z).pc;
    (*z).pc = ((*z).pc).wrapping_add(1);
    return rb(z, fresh0);
}
#[inline]
unsafe extern "C" fn nextw(z: *mut z80) -> uint16_t {
    (*z).pc = ((*z).pc as libc::c_int + 2 as libc::c_int) as uint16_t;
    return rw(z, ((*z).pc as libc::c_int - 2 as libc::c_int) as uint16_t);
}
#[inline]
unsafe extern "C" fn inc_r(z: *mut z80) {
    (*z)
        .r = ((*z).r as libc::c_int & 0x80 as libc::c_int
        | (*z).r as libc::c_int + 1 as libc::c_int & 0x7f as libc::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn parity(mut v: uint8_t) -> bool {
    v = (v as libc::c_int ^ v as libc::c_int >> 4 as libc::c_int) as uint8_t;
    v = (v as libc::c_int & 0xf as libc::c_int) as uint8_t;
    return 0x6996 as libc::c_int >> v as libc::c_int & 1 as libc::c_int == 0;
}
#[inline]
unsafe extern "C" fn jump(z: *mut z80, mut addr: uint16_t) {
    (*z).pc = addr;
    (*z).mem_ptr = addr;
}
#[inline]
unsafe extern "C" fn cond_jump(z: *mut z80, mut condition: bool) {
    let addr: uint16_t = nextw(z);
    if condition {
        jump(z, addr);
    }
    (*z).mem_ptr = addr;
}
#[inline]
unsafe extern "C" fn call(z: *mut z80, mut addr: uint16_t) {
    pushw(z, (*z).pc);
    (*z).pc = addr;
    (*z).mem_ptr = addr;
}
#[inline]
unsafe extern "C" fn cond_call(z: *mut z80, mut condition: bool) -> libc::c_uint {
    let addr: uint16_t = nextw(z);
    let mut cyc: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if condition {
        call(z, addr);
        cyc = 7 as libc::c_int as libc::c_uint;
    }
    (*z).mem_ptr = addr;
    return cyc;
}
#[inline]
unsafe extern "C" fn ret(z: *mut z80) {
    (*z).pc = popw(z);
    (*z).mem_ptr = (*z).pc;
}
#[inline]
unsafe extern "C" fn cond_ret(z: *mut z80, mut condition: bool) -> libc::c_uint {
    if condition {
        ret(z);
        return 6 as libc::c_int as libc::c_uint;
    }
    return 0 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn jr(z: *mut z80, mut displacement: int8_t) {
    (*z).pc = ((*z).pc as libc::c_int + displacement as libc::c_int) as uint16_t;
    (*z).mem_ptr = (*z).pc;
}
#[inline]
unsafe extern "C" fn cond_jr(z: *mut z80, mut condition: bool) -> libc::c_uint {
    let b: int8_t = nextb(z) as int8_t;
    if condition {
        jr(z, b);
        return 5 as libc::c_int as libc::c_uint;
    }
    return 0 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn addb(
    z: *mut z80,
    mut a: uint32_t,
    mut b: uint32_t,
    mut cy: bool,
) -> uint8_t {
    let mut result: int32_t = a.wrapping_add(b).wrapping_add(cy as libc::c_uint)
        as int32_t;
    let mut carry: int32_t = (result as libc::c_uint ^ a ^ b) as int32_t;
    result &= 0xff as libc::c_int;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[result as usize] as libc::c_int
        & !((1 as libc::c_int) << pf as libc::c_int)) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as libc::c_int
        | carry & (1 as libc::c_int) << hf as libc::c_int) as uint8_t;
    carry >>= 6 as libc::c_int;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as libc::c_int
        | carry + 2 as libc::c_int & 4 as libc::c_int) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as libc::c_int
        | carry >> 2 as libc::c_int) as uint8_t;
    return result as uint8_t;
}
#[inline]
unsafe extern "C" fn subb(
    z: *mut z80,
    mut a: uint32_t,
    mut b: uint32_t,
    mut cy: bool,
) -> uint8_t {
    let mut result: int32_t = a.wrapping_sub(b).wrapping_sub(cy as libc::c_uint)
        as int32_t;
    let mut carry: int32_t = (result as libc::c_uint ^ a ^ b) as int32_t;
    result &= 0xff as libc::c_int;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((1 as libc::c_int) << nf as libc::c_int
        | f_szpxy[result as usize] as libc::c_int
            & !((1 as libc::c_int) << pf as libc::c_int)) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as libc::c_int
        | carry & (1 as libc::c_int) << hf as libc::c_int) as uint8_t;
    carry >>= 6 as libc::c_int;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as libc::c_int
        | carry + 2 as libc::c_int & 4 as libc::c_int) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as libc::c_int
        | carry >> 2 as libc::c_int & 1 as libc::c_int) as uint8_t;
    return result as uint8_t;
}
#[inline]
unsafe extern "C" fn addw(
    z: *mut z80,
    mut a: uint16_t,
    mut b: uint16_t,
    mut cy: bool,
) -> uint16_t {
    let mut lsb: uint8_t = addb(z, a as uint32_t, b as uint32_t, cy);
    let mut msb: uint8_t = addb(
        z,
        (a as libc::c_int >> 8 as libc::c_int) as uint32_t,
        (b as libc::c_int >> 8 as libc::c_int) as uint32_t,
        flag_get(z, cf),
    );
    let mut result: uint16_t = ((msb as libc::c_int) << 8 as libc::c_int
        | lsb as libc::c_int) as uint16_t;
    flag_set(z, zf, result as libc::c_int == 0 as libc::c_int);
    (*z).mem_ptr = (a as libc::c_int + 1 as libc::c_int) as uint16_t;
    return result;
}
#[inline]
unsafe extern "C" fn subw(
    z: *mut z80,
    mut a: uint16_t,
    mut b: uint16_t,
    mut cy: bool,
) -> uint16_t {
    let mut lsb: uint8_t = subb(z, a as uint32_t, b as uint32_t, cy);
    let mut msb: uint8_t = subb(
        z,
        (a as libc::c_int >> 8 as libc::c_int) as uint32_t,
        (b as libc::c_int >> 8 as libc::c_int) as uint32_t,
        flag_get(z, cf),
    );
    let mut result: uint16_t = ((msb as libc::c_int) << 8 as libc::c_int
        | lsb as libc::c_int) as uint16_t;
    flag_set(z, zf, result as libc::c_int == 0 as libc::c_int);
    (*z).mem_ptr = (a as libc::c_int + 1 as libc::c_int) as uint16_t;
    return result;
}
#[inline]
unsafe extern "C" fn addhl(z: *mut z80, mut val: uint16_t) {
    let mut sfc: bool = flag_get(z, sf);
    let mut zfc: bool = flag_get(z, zf);
    let mut pfc: bool = flag_get(z, pf);
    let mut result: uint16_t = addw(
        z,
        (*z).c2rust_unnamed_2.hl,
        val,
        0 as libc::c_int != 0,
    );
    (*z).c2rust_unnamed_2.hl = result;
    flag_set(z, sf, sfc);
    flag_set(z, zf, zfc);
    flag_set(z, pf, pfc);
}
#[inline]
unsafe extern "C" fn addiz(z: *mut z80, mut reg: *mut uint16_t, mut val: uint16_t) {
    let mut sfc: bool = flag_get(z, sf);
    let mut zfc: bool = flag_get(z, zf);
    let mut pfc: bool = flag_get(z, pf);
    let mut result: uint16_t = addw(z, *reg, val, 0 as libc::c_int != 0);
    *reg = result;
    flag_set(z, sf, sfc);
    flag_set(z, zf, zfc);
    flag_set(z, pf, pfc);
}
#[inline]
unsafe extern "C" fn adchl(z: *mut z80, mut val: uint16_t) {
    let mut result: uint16_t = addw(z, (*z).c2rust_unnamed_2.hl, val, flag_get(z, cf));
    flag_set(z, sf, result as libc::c_int >> 15 as libc::c_int != 0);
    flag_set(z, zf, result as libc::c_int == 0 as libc::c_int);
    (*z).c2rust_unnamed_2.hl = result;
}
#[inline]
unsafe extern "C" fn sbchl(z: *mut z80, mut val: uint16_t) {
    let result: uint16_t = subw(z, (*z).c2rust_unnamed_2.hl, val, flag_get(z, cf));
    flag_set(z, sf, result as libc::c_int >> 15 as libc::c_int != 0);
    flag_set(z, zf, result as libc::c_int == 0 as libc::c_int);
    (*z).c2rust_unnamed_2.hl = result;
}
#[inline]
unsafe extern "C" fn inc(z: *mut z80, mut a: uint8_t) -> uint8_t {
    let mut cfc: bool = flag_get(z, cf);
    let mut result: uint8_t = addb(
        z,
        a as uint32_t,
        1 as libc::c_int as uint32_t,
        0 as libc::c_int != 0,
    );
    flag_set(z, cf, cfc);
    return result;
}
#[inline]
unsafe extern "C" fn dec(z: *mut z80, mut a: uint8_t) -> uint8_t {
    let mut cfc: bool = flag_get(z, cf);
    let mut result: uint8_t = subb(
        z,
        a as uint32_t,
        1 as libc::c_int as uint32_t,
        0 as libc::c_int != 0,
    );
    flag_set(z, cf, cfc);
    return result;
}
#[inline]
unsafe extern "C" fn land(z: *mut z80, mut val: uint8_t) {
    let result: uint8_t = ((*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int
        & val as libc::c_int) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[result as usize] as libc::c_int
        | flag_val(hf, 1 as libc::c_int != 0) as libc::c_int
        | flag_val(nf, 0 as libc::c_int != 0) as libc::c_int
        | flag_val(cf, 0 as libc::c_int != 0) as libc::c_int) as uint8_t;
    (*z).c2rust_unnamed.c2rust_unnamed.a = result;
}
#[inline]
unsafe extern "C" fn lxor(z: *mut z80, val: uint8_t) {
    let result: uint8_t = ((*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int
        ^ val as libc::c_int) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[result as usize] as libc::c_int
        | flag_val(hf, 0 as libc::c_int != 0) as libc::c_int
        | flag_val(nf, 0 as libc::c_int != 0) as libc::c_int
        | flag_val(cf, 0 as libc::c_int != 0) as libc::c_int) as uint8_t;
    (*z).c2rust_unnamed.c2rust_unnamed.a = result;
}
#[inline]
unsafe extern "C" fn lor(z: *mut z80, val: uint8_t) {
    let result: uint8_t = ((*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int
        | val as libc::c_int) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[result as usize] as libc::c_int
        | flag_val(hf, 0 as libc::c_int != 0) as libc::c_int
        | flag_val(nf, 0 as libc::c_int != 0) as libc::c_int
        | flag_val(cf, 0 as libc::c_int != 0) as libc::c_int) as uint8_t;
    (*z).c2rust_unnamed.c2rust_unnamed.a = result;
}
#[inline]
unsafe extern "C" fn cp(z: *mut z80, val: uint32_t) {
    let mut result: int32_t = ((*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_uint)
        .wrapping_sub(val) as int32_t;
    let mut carry: int32_t = ((result
        ^ (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int) as libc::c_uint ^ val)
        as int32_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (((1 as libc::c_int) << nf as libc::c_int) as libc::c_uint
        | val
            & ((1 as libc::c_int) << xf as libc::c_int
                | (1 as libc::c_int) << yf as libc::c_int) as libc::c_uint
        | (result & (1 as libc::c_int) << sf as libc::c_int) as libc::c_uint
        | (((result & 0xff as libc::c_int == 0) as libc::c_int) << zf as libc::c_int)
            as libc::c_uint) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as libc::c_int
        | carry & (1 as libc::c_int) << hf as libc::c_int) as uint8_t;
    carry >>= 6 as libc::c_int;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as libc::c_int
        | carry + 2 as libc::c_int & 4 as libc::c_int) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as libc::c_int
        | carry >> 2 as libc::c_int & 1 as libc::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn cb_rlc(z: *mut z80, mut val: uint8_t) -> uint8_t {
    let old: bool = val as libc::c_int >> 7 as libc::c_int != 0;
    val = ((val as libc::c_int) << 1 as libc::c_int | old as libc::c_int) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[val as usize] as libc::c_int
        | flag_val(nf, 0 as libc::c_int != 0) as libc::c_int
        | flag_val(hf, 0 as libc::c_int != 0) as libc::c_int
        | flag_val(cf, old) as libc::c_int) as uint8_t;
    return val;
}
#[inline]
unsafe extern "C" fn cb_rrc(z: *mut z80, mut val: uint8_t) -> uint8_t {
    let old: bool = val as libc::c_int & 1 as libc::c_int != 0;
    val = (val as libc::c_int >> 1 as libc::c_int
        | (old as libc::c_int) << 7 as libc::c_int) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[val as usize] as libc::c_int
        | flag_val(nf, 0 as libc::c_int != 0) as libc::c_int
        | flag_val(hf, 0 as libc::c_int != 0) as libc::c_int
        | flag_val(cf, old) as libc::c_int) as uint8_t;
    return val;
}
#[inline]
unsafe extern "C" fn cb_rl(z: *mut z80, mut val: uint8_t) -> uint8_t {
    let cfc: bool = flag_get(z, cf);
    let cfn: bool = val as libc::c_int >> 7 as libc::c_int != 0;
    val = ((val as libc::c_int) << 1 as libc::c_int | cfc as libc::c_int) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[val as usize] as libc::c_int | flag_val(cf, cfn) as libc::c_int
        | flag_val(nf, 0 as libc::c_int != 0) as libc::c_int
        | flag_val(hf, 0 as libc::c_int != 0) as libc::c_int) as uint8_t;
    return val;
}
#[inline]
unsafe extern "C" fn cb_rr(z: *mut z80, mut val: uint8_t) -> uint8_t {
    let c: bool = flag_get(z, cf);
    let cfn: bool = val as libc::c_int & 1 as libc::c_int != 0;
    val = (val as libc::c_int >> 1 as libc::c_int
        | (c as libc::c_int) << 7 as libc::c_int) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[val as usize] as libc::c_int | flag_val(cf, cfn) as libc::c_int
        | flag_val(nf, 0 as libc::c_int != 0) as libc::c_int
        | flag_val(hf, 0 as libc::c_int != 0) as libc::c_int) as uint8_t;
    return val;
}
#[inline]
unsafe extern "C" fn cb_sla(z: *mut z80, mut val: uint8_t) -> uint8_t {
    let cfn: bool = val as libc::c_int >> 7 as libc::c_int != 0;
    val = ((val as libc::c_int) << 1 as libc::c_int) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[val as usize] as libc::c_int | flag_val(cf, cfn) as libc::c_int
        | flag_val(nf, 0 as libc::c_int != 0) as libc::c_int
        | flag_val(hf, 0 as libc::c_int != 0) as libc::c_int) as uint8_t;
    return val;
}
#[inline]
unsafe extern "C" fn cb_sll(z: *mut z80, mut val: uint8_t) -> uint8_t {
    let cfn: bool = val as libc::c_int >> 7 as libc::c_int != 0;
    val = ((val as libc::c_int) << 1 as libc::c_int) as uint8_t;
    val = (val as libc::c_int | 1 as libc::c_int) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[val as usize] as libc::c_int | flag_val(cf, cfn) as libc::c_int
        | flag_val(nf, 0 as libc::c_int != 0) as libc::c_int
        | flag_val(hf, 0 as libc::c_int != 0) as libc::c_int) as uint8_t;
    return val;
}
#[inline]
unsafe extern "C" fn cb_sra(z: *mut z80, mut val: uint8_t) -> uint8_t {
    let cfn: bool = val as libc::c_int & 1 as libc::c_int != 0;
    val = (val as libc::c_int >> 1 as libc::c_int
        | val as libc::c_int & 0x80 as libc::c_int) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[val as usize] as libc::c_int | flag_val(cf, cfn) as libc::c_int
        | flag_val(nf, 0 as libc::c_int != 0) as libc::c_int
        | flag_val(hf, 0 as libc::c_int != 0) as libc::c_int) as uint8_t;
    return val;
}
#[inline]
unsafe extern "C" fn cb_srl(z: *mut z80, mut val: uint8_t) -> uint8_t {
    let cfn: bool = val as libc::c_int & 1 as libc::c_int != 0;
    val = (val as libc::c_int >> 1 as libc::c_int) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[val as usize] as libc::c_int | flag_val(cf, cfn) as libc::c_int
        | flag_val(nf, 0 as libc::c_int != 0) as libc::c_int
        | flag_val(hf, 0 as libc::c_int != 0) as libc::c_int) as uint8_t;
    return val;
}
#[inline]
unsafe extern "C" fn cb_bit(z: *mut z80, mut val: uint8_t, mut n: uint8_t) -> uint8_t {
    let result: uint8_t = (val as libc::c_int & (1 as libc::c_int) << n as libc::c_int)
        as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = (f_szpxy[result as usize] as libc::c_int
        | flag_val(cf, flag_get(z, cf)) as libc::c_int
        | flag_val(hf, 1 as libc::c_int != 0) as libc::c_int
        | flag_val(nf, 0 as libc::c_int != 0) as libc::c_int) as uint8_t;
    return result;
}
#[inline]
unsafe extern "C" fn ldi(z: *mut z80) {
    let de: uint16_t = (*z).c2rust_unnamed_1.de;
    let hl: uint16_t = (*z).c2rust_unnamed_2.hl;
    let val: uint8_t = rb(z, hl);
    wb(z, de, val);
    (*z).c2rust_unnamed_2.hl = ((*z).c2rust_unnamed_2.hl).wrapping_add(1);
    (*z).c2rust_unnamed_1.de = ((*z).c2rust_unnamed_1.de).wrapping_add(1);
    (*z).c2rust_unnamed_0.bc = ((*z).c2rust_unnamed_0.bc).wrapping_sub(1);
    let result: uint8_t = (val as libc::c_int
        + (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int) as uint8_t;
    flag_set(z, xf, result as libc::c_int >> 3 as libc::c_int & 1 as libc::c_int != 0);
    flag_set(z, yf, result as libc::c_int >> 1 as libc::c_int & 1 as libc::c_int != 0);
    flag_set(z, nf, 0 as libc::c_int != 0);
    flag_set(z, hf, 0 as libc::c_int != 0);
    flag_set(z, pf, (*z).c2rust_unnamed_0.bc as libc::c_int > 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn ldd(z: *mut z80) {
    ldi(z);
    (*z)
        .c2rust_unnamed_2
        .hl = ((*z).c2rust_unnamed_2.hl as libc::c_int - 2 as libc::c_int) as uint16_t;
    (*z)
        .c2rust_unnamed_1
        .de = ((*z).c2rust_unnamed_1.de as libc::c_int - 2 as libc::c_int) as uint16_t;
}
#[inline]
unsafe extern "C" fn cpi(z: *mut z80) {
    let mut cfc: bool = flag_get(z, cf);
    let result: uint8_t = subb(
        z,
        (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
        rb(z, (*z).c2rust_unnamed_2.hl) as uint32_t,
        0 as libc::c_int != 0,
    );
    (*z).c2rust_unnamed_2.hl = ((*z).c2rust_unnamed_2.hl).wrapping_add(1);
    (*z).c2rust_unnamed_0.bc = ((*z).c2rust_unnamed_0.bc).wrapping_sub(1);
    let mut hfc: bool = flag_get(z, hf);
    flag_set(
        z,
        xf,
        result as libc::c_int - hfc as libc::c_int >> 3 as libc::c_int & 1 as libc::c_int
            != 0,
    );
    flag_set(
        z,
        yf,
        result as libc::c_int - hfc as libc::c_int >> 1 as libc::c_int & 1 as libc::c_int
            != 0,
    );
    flag_set(z, pf, (*z).c2rust_unnamed_0.bc as libc::c_int != 0 as libc::c_int);
    flag_set(z, cf, cfc);
    (*z).mem_ptr = ((*z).mem_ptr as libc::c_int + 1 as libc::c_int) as uint16_t;
}
#[inline]
unsafe extern "C" fn cpd(z: *mut z80) {
    cpi(z);
    (*z)
        .c2rust_unnamed_2
        .hl = ((*z).c2rust_unnamed_2.hl as libc::c_int - 2 as libc::c_int) as uint16_t;
    (*z).mem_ptr = ((*z).mem_ptr as libc::c_int - 2 as libc::c_int) as uint16_t;
}
unsafe extern "C" fn in_r_c(z: *mut z80, mut r: *mut uint8_t) {
    *r = ((*z).port_in).expect("non-null function pointer")(z, (*z).c2rust_unnamed_0.bc);
    flag_set(z, zf, *r as libc::c_int == 0 as libc::c_int);
    flag_set(z, sf, *r as libc::c_int >> 7 as libc::c_int != 0);
    flag_set(z, pf, parity(*r));
    flag_set(z, nf, 0 as libc::c_int != 0);
    flag_set(z, hf, 0 as libc::c_int != 0);
}
unsafe extern "C" fn ini(z: *mut z80) {
    let mut tmp: libc::c_uint = ((*z).port_in)
        .expect("non-null function pointer")(z, (*z).c2rust_unnamed_0.bc)
        as libc::c_uint;
    let mut tmp2: libc::c_uint = tmp
        .wrapping_add(
            ((*z).c2rust_unnamed_0.c2rust_unnamed.c as libc::c_int + 1 as libc::c_int
                & 0xff as libc::c_int) as libc::c_uint,
        );
    (*z)
        .mem_ptr = ((*z).c2rust_unnamed_0.bc as libc::c_int + 1 as libc::c_int)
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
        .f = (f_szpxy[(*z).c2rust_unnamed_0.c2rust_unnamed.b as usize] as libc::c_int
        & !((1 as libc::c_int) << pf as libc::c_int)
        | flag_val(nf, tmp >> 7 as libc::c_int & 1 as libc::c_int as libc::c_uint != 0)
            as libc::c_int
        | flag_val(
            pf,
            parity(
                (tmp2 & 7 as libc::c_int as libc::c_uint
                    ^ (*z).c2rust_unnamed_0.c2rust_unnamed.b as libc::c_uint) as uint8_t,
            ),
        ) as libc::c_int
        | flag_val(hf, tmp2 > 255 as libc::c_int as libc::c_uint) as libc::c_int
        | flag_val(cf, tmp2 > 255 as libc::c_int as libc::c_uint) as libc::c_int)
        as uint8_t;
}
unsafe extern "C" fn ind(z: *mut z80) {
    let mut tmp: libc::c_uint = ((*z).port_in)
        .expect("non-null function pointer")(z, (*z).c2rust_unnamed_0.bc)
        as libc::c_uint;
    let mut tmp2: libc::c_uint = tmp
        .wrapping_add(
            ((*z).c2rust_unnamed_0.c2rust_unnamed.c as libc::c_int - 1 as libc::c_int
                & 0xff as libc::c_int) as libc::c_uint,
        );
    (*z)
        .mem_ptr = ((*z).c2rust_unnamed_0.bc as libc::c_int - 1 as libc::c_int)
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
        .f = (f_szpxy[(*z).c2rust_unnamed_0.c2rust_unnamed.b as usize] as libc::c_int
        & !((1 as libc::c_int) << pf as libc::c_int)
        | flag_val(nf, tmp >> 7 as libc::c_int & 1 as libc::c_int as libc::c_uint != 0)
            as libc::c_int
        | flag_val(
            pf,
            parity(
                (tmp2 & 7 as libc::c_int as libc::c_uint
                    ^ (*z).c2rust_unnamed_0.c2rust_unnamed.b as libc::c_uint) as uint8_t,
            ),
        ) as libc::c_int
        | flag_val(hf, tmp2 > 255 as libc::c_int as libc::c_uint) as libc::c_int
        | flag_val(cf, tmp2 > 255 as libc::c_int as libc::c_uint) as libc::c_int)
        as uint8_t;
}
unsafe extern "C" fn outi(z: *mut z80) {
    let mut tmp: libc::c_uint = rb(z, (*z).c2rust_unnamed_2.hl) as libc::c_uint;
    let mut tmp2: libc::c_uint = 0;
    ((*z).port_out)
        .expect(
            "non-null function pointer",
        )(z, (*z).c2rust_unnamed_0.bc, tmp as uint8_t);
    (*z).c2rust_unnamed_2.hl = ((*z).c2rust_unnamed_2.hl).wrapping_add(1);
    (*z)
        .c2rust_unnamed_0
        .c2rust_unnamed
        .b = ((*z).c2rust_unnamed_0.c2rust_unnamed.b as libc::c_int - 1 as libc::c_int)
        as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = f_szpxy[(*z).c2rust_unnamed_0.c2rust_unnamed.b as usize];
    flag_set(z, nf, tmp >> 7 as libc::c_int & 1 as libc::c_int as libc::c_uint != 0);
    tmp2 = tmp.wrapping_add((*z).c2rust_unnamed_2.c2rust_unnamed.l as libc::c_uint);
    flag_set(
        z,
        pf,
        parity(
            (tmp2 & 7 as libc::c_int as libc::c_uint
                ^ (*z).c2rust_unnamed_0.c2rust_unnamed.b as libc::c_uint) as uint8_t,
        ),
    );
    flag_set(z, hf, tmp2 > 255 as libc::c_int as libc::c_uint);
    flag_set(z, cf, tmp2 > 255 as libc::c_int as libc::c_uint);
    (*z)
        .mem_ptr = ((*z).c2rust_unnamed_0.bc as libc::c_int + 1 as libc::c_int)
        as uint16_t;
}
unsafe extern "C" fn outd(z: *mut z80) {
    outi(z);
    (*z)
        .c2rust_unnamed_2
        .hl = ((*z).c2rust_unnamed_2.hl as libc::c_int - 2 as libc::c_int) as uint16_t;
    (*z)
        .mem_ptr = ((*z).c2rust_unnamed_0.bc as libc::c_int - 2 as libc::c_int)
        as uint16_t;
}
unsafe extern "C" fn outc(z: *mut z80, mut data: uint8_t) {
    ((*z).port_out)
        .expect("non-null function pointer")(z, (*z).c2rust_unnamed_0.bc, data);
    (*z)
        .mem_ptr = ((*z).c2rust_unnamed_0.bc as libc::c_int + 1 as libc::c_int)
        as uint16_t;
}
unsafe extern "C" fn daa(z: *mut z80) {
    let mut correction: uint8_t = 0 as libc::c_int as uint8_t;
    if (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int & 0xf as libc::c_int
        > 0x9 as libc::c_int || flag_get(z, hf) as libc::c_int != 0
    {
        correction = (correction as libc::c_int + 0x6 as libc::c_int) as uint8_t;
    }
    if (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int > 0x99 as libc::c_int
        || flag_get(z, cf) as libc::c_int != 0
    {
        correction = (correction as libc::c_int + 0x60 as libc::c_int) as uint8_t;
        flag_set(z, cf, 1 as libc::c_int != 0);
    }
    let substraction: bool = flag_get(z, nf);
    if substraction {
        flag_set(
            z,
            hf,
            flag_get(z, hf) as libc::c_int != 0
                && ((*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int
                    & 0xf as libc::c_int) < 0x6 as libc::c_int,
        );
        (*z)
            .c2rust_unnamed
            .c2rust_unnamed
            .a = ((*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int
            - correction as libc::c_int) as uint8_t;
    } else {
        flag_set(
            z,
            hf,
            (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int & 0xf as libc::c_int
                > 0x9 as libc::c_int,
        );
        (*z)
            .c2rust_unnamed
            .c2rust_unnamed
            .a = ((*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int
            + correction as libc::c_int) as uint8_t;
    }
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as libc::c_int
        & !((1 as libc::c_int) << sf as libc::c_int
            | (1 as libc::c_int) << zf as libc::c_int
            | (1 as libc::c_int) << pf as libc::c_int
            | (1 as libc::c_int) << xf as libc::c_int
            | (1 as libc::c_int) << yf as libc::c_int)) as uint8_t;
    (*z)
        .c2rust_unnamed
        .c2rust_unnamed
        .f = ((*z).c2rust_unnamed.c2rust_unnamed.f as libc::c_int
        | f_szpxy[(*z).c2rust_unnamed.c2rust_unnamed.a as usize] as libc::c_int)
        as uint8_t;
}
#[inline]
unsafe extern "C" fn displace(
    z: *mut z80,
    mut base_addr: uint16_t,
    mut displacement: int8_t,
) -> uint16_t {
    let addr: uint16_t = (base_addr as libc::c_int + displacement as libc::c_int)
        as uint16_t;
    (*z).mem_ptr = addr;
    return addr;
}
#[inline]
unsafe extern "C" fn process_interrupts(z: *mut z80) -> libc::c_uint {
    let mut cyc: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if (*z).iff_delay as libc::c_int > 0 as libc::c_int {
        (*z).iff_delay = ((*z).iff_delay as libc::c_int - 1 as libc::c_int) as uint8_t;
        if (*z).iff_delay as libc::c_int == 0 as libc::c_int {
            (*z).set_iff1(1 as libc::c_int != 0);
            (*z).set_iff2(1 as libc::c_int != 0);
        }
        return cyc;
    }
    if (*z).nmi_pending != 0 {
        (*z)
            .nmi_pending = ((*z).nmi_pending as libc::c_int
            & !(Z80_PULSE as libc::c_int)) as uint8_t;
        (*z).set_halted(0 as libc::c_int != 0);
        (*z).set_iff1(0 as libc::c_int != 0);
        inc_r(z);
        cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
        call(z, 0x66 as libc::c_int as uint16_t);
        return cyc;
    }
    if (*z).irq_pending as libc::c_int != 0 && (*z).iff1() as libc::c_int != 0 {
        (*z)
            .irq_pending = ((*z).irq_pending as libc::c_int
            & !(Z80_PULSE as libc::c_int)) as uint8_t;
        (*z).set_halted(0 as libc::c_int != 0);
        (*z).set_iff1(0 as libc::c_int != 0);
        (*z).set_iff2(0 as libc::c_int != 0);
        inc_r(z);
        match (*z).interrupt_mode as libc::c_int {
            0 => {
                cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
                cyc = cyc.wrapping_add(exec_opcode(z, (*z).irq_data));
            }
            1 => {
                cyc = cyc.wrapping_add(13 as libc::c_int as libc::c_uint);
                call(z, 0x38 as libc::c_int as uint16_t);
            }
            2 => {
                cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
                call(
                    z,
                    rw(
                        z,
                        (((*z).i as libc::c_int) << 8 as libc::c_int
                            | (*z).irq_data as libc::c_int) as uint16_t,
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
pub unsafe extern "C" fn z80_init(z: *mut z80) {
    (*z).read_byte = None;
    (*z).write_byte = None;
    (*z).port_in = None;
    (*z).port_out = None;
    (*z).userdata = 0 as *mut libc::c_void;
    (*z).pc = 0 as libc::c_int as uint16_t;
    (*z).sp = 0xffff as libc::c_int as uint16_t;
    (*z).ix = 0 as libc::c_int as uint16_t;
    (*z).iy = 0 as libc::c_int as uint16_t;
    (*z).mem_ptr = 0 as libc::c_int as uint16_t;
    (*z).c2rust_unnamed.af = 0xffff as libc::c_int as uint16_t;
    (*z).c2rust_unnamed_0.bc = 0 as libc::c_int as uint16_t;
    (*z).c2rust_unnamed_1.de = 0 as libc::c_int as uint16_t;
    (*z).c2rust_unnamed_2.hl = 0 as libc::c_int as uint16_t;
    (*z).c2rust_unnamed_3.a_f_ = 0 as libc::c_int as uint16_t;
    (*z).c2rust_unnamed_4.b_c_ = 0 as libc::c_int as uint16_t;
    (*z).c2rust_unnamed_5.d_e_ = 0 as libc::c_int as uint16_t;
    (*z).c2rust_unnamed_6.h_l_ = 0 as libc::c_int as uint16_t;
    (*z).i = 0 as libc::c_int as uint8_t;
    (*z).r = 0 as libc::c_int as uint8_t;
    (*z).iff_delay = 0 as libc::c_int as uint8_t;
    (*z).interrupt_mode = 0 as libc::c_int as uint8_t;
    (*z).set_iff1(0 as libc::c_int != 0);
    (*z).set_iff2(0 as libc::c_int != 0);
    (*z).set_halted(0 as libc::c_int != 0);
    (*z).irq_pending = 0 as libc::c_int as uint8_t;
    (*z).nmi_pending = 0 as libc::c_int as uint8_t;
    (*z).irq_data = 0 as libc::c_int as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn z80_reset(z: *mut z80) {
    (*z).pc = 0 as libc::c_int as uint16_t;
    (*z).mem_ptr = 0 as libc::c_int as uint16_t;
    (*z).i = 0 as libc::c_int as uint8_t;
    (*z).r = 0 as libc::c_int as uint8_t;
    (*z).interrupt_mode = 0 as libc::c_int as uint8_t;
    (*z).iff_delay = 0 as libc::c_int as uint8_t;
    (*z).set_iff1(0 as libc::c_int != 0);
    (*z).set_iff2(0 as libc::c_int != 0);
    (*z).set_halted(0 as libc::c_int != 0);
    (*z).nmi_pending = 0 as libc::c_int as uint8_t;
}
unsafe extern "C" fn z80_step_s(z: *mut z80) -> libc::c_uint {
    let mut cyc: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if (*z).halted() {
        cyc = cyc.wrapping_add(exec_opcode(z, 0 as libc::c_int as uint8_t));
    } else {
        let opcode: uint8_t = nextb(z);
        cyc = cyc.wrapping_add(exec_opcode(z, opcode));
    }
    cyc = cyc.wrapping_add(process_interrupts(z));
    return cyc;
}
#[no_mangle]
pub unsafe extern "C" fn z80_set_pc(z: *mut z80, mut pc: uint16_t) {
    (*z).pc = pc;
}
#[no_mangle]
pub unsafe extern "C" fn z80_set_sp(z: *mut z80, mut sp: uint16_t) {
    (*z).sp = sp;
}
#[no_mangle]
pub unsafe extern "C" fn z80_step(z: *mut z80) -> libc::c_uint {
    return z80_step_s(z);
}
#[no_mangle]
pub unsafe extern "C" fn z80_step_n(
    z: *mut z80,
    mut cycles: libc::c_uint,
) -> libc::c_uint {
    let mut cyc: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while cyc < cycles {
        cyc = cyc.wrapping_add(z80_step_s(z));
    }
    return cyc;
}
#[no_mangle]
pub unsafe extern "C" fn z80_debug_output(z: *mut z80) {
    !z.is_null();
}
#[no_mangle]
pub unsafe extern "C" fn z80_assert_nmi(z: *mut z80) {
    (*z)
        .nmi_pending = ((*z).nmi_pending as libc::c_int | Z80_ASSERT as libc::c_int)
        as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn z80_pulse_nmi(z: *mut z80) {
    (*z)
        .nmi_pending = ((*z).nmi_pending as libc::c_int | Z80_PULSE as libc::c_int)
        as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn z80_clr_nmi(z: *mut z80) {
    (*z).nmi_pending = 0 as libc::c_int as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn z80_assert_irq(z: *mut z80, mut data: uint8_t) {
    (*z)
        .irq_pending = ((*z).irq_pending as libc::c_int | Z80_ASSERT as libc::c_int)
        as uint8_t;
    (*z).irq_data = data;
}
#[no_mangle]
pub unsafe extern "C" fn z80_pulse_irq(z: *mut z80, mut data: uint8_t) {
    (*z)
        .irq_pending = ((*z).irq_pending as libc::c_int | Z80_PULSE as libc::c_int)
        as uint8_t;
    (*z).irq_data = data;
}
#[no_mangle]
pub unsafe extern "C" fn z80_clr_irq(z: *mut z80) {
    (*z).irq_pending = 0 as libc::c_int as uint8_t;
}
unsafe extern "C" fn exec_opcode(z: *mut z80, mut opcode: uint8_t) -> libc::c_uint {
    let mut cyc: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    inc_r(z);
    match opcode as libc::c_int {
        127 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed.c2rust_unnamed.a = (*z).c2rust_unnamed.c2rust_unnamed.a;
        }
        120 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (*z).c2rust_unnamed_0.c2rust_unnamed.b;
        }
        121 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (*z).c2rust_unnamed_0.c2rust_unnamed.c;
        }
        122 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (*z).c2rust_unnamed_1.c2rust_unnamed.d;
        }
        123 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (*z).c2rust_unnamed_1.c2rust_unnamed.e;
        }
        124 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (*z).c2rust_unnamed_2.c2rust_unnamed.h;
        }
        125 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (*z).c2rust_unnamed_2.c2rust_unnamed.l;
        }
        71 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = (*z).c2rust_unnamed.c2rust_unnamed.a;
        }
        64 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = (*z).c2rust_unnamed_0.c2rust_unnamed.b;
        }
        65 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = (*z).c2rust_unnamed_0.c2rust_unnamed.c;
        }
        66 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = (*z).c2rust_unnamed_1.c2rust_unnamed.d;
        }
        67 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = (*z).c2rust_unnamed_1.c2rust_unnamed.e;
        }
        68 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = (*z).c2rust_unnamed_2.c2rust_unnamed.h;
        }
        69 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = (*z).c2rust_unnamed_2.c2rust_unnamed.l;
        }
        79 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = (*z).c2rust_unnamed.c2rust_unnamed.a;
        }
        72 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = (*z).c2rust_unnamed_0.c2rust_unnamed.b;
        }
        73 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = (*z).c2rust_unnamed_0.c2rust_unnamed.c;
        }
        74 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = (*z).c2rust_unnamed_1.c2rust_unnamed.d;
        }
        75 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = (*z).c2rust_unnamed_1.c2rust_unnamed.e;
        }
        76 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = (*z).c2rust_unnamed_2.c2rust_unnamed.h;
        }
        77 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = (*z).c2rust_unnamed_2.c2rust_unnamed.l;
        }
        87 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = (*z).c2rust_unnamed.c2rust_unnamed.a;
        }
        80 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = (*z).c2rust_unnamed_0.c2rust_unnamed.b;
        }
        81 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = (*z).c2rust_unnamed_0.c2rust_unnamed.c;
        }
        82 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = (*z).c2rust_unnamed_1.c2rust_unnamed.d;
        }
        83 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = (*z).c2rust_unnamed_1.c2rust_unnamed.e;
        }
        84 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = (*z).c2rust_unnamed_2.c2rust_unnamed.h;
        }
        85 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = (*z).c2rust_unnamed_2.c2rust_unnamed.l;
        }
        95 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = (*z).c2rust_unnamed.c2rust_unnamed.a;
        }
        88 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = (*z).c2rust_unnamed_0.c2rust_unnamed.b;
        }
        89 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = (*z).c2rust_unnamed_0.c2rust_unnamed.c;
        }
        90 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = (*z).c2rust_unnamed_1.c2rust_unnamed.d;
        }
        91 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = (*z).c2rust_unnamed_1.c2rust_unnamed.e;
        }
        92 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = (*z).c2rust_unnamed_2.c2rust_unnamed.h;
        }
        93 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = (*z).c2rust_unnamed_2.c2rust_unnamed.l;
        }
        103 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .h = (*z).c2rust_unnamed.c2rust_unnamed.a;
        }
        96 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .h = (*z).c2rust_unnamed_0.c2rust_unnamed.b;
        }
        97 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .h = (*z).c2rust_unnamed_0.c2rust_unnamed.c;
        }
        98 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .h = (*z).c2rust_unnamed_1.c2rust_unnamed.d;
        }
        99 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .h = (*z).c2rust_unnamed_1.c2rust_unnamed.e;
        }
        100 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .h = (*z).c2rust_unnamed_2.c2rust_unnamed.h;
        }
        101 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .h = (*z).c2rust_unnamed_2.c2rust_unnamed.l;
        }
        111 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .l = (*z).c2rust_unnamed.c2rust_unnamed.a;
        }
        104 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .l = (*z).c2rust_unnamed_0.c2rust_unnamed.b;
        }
        105 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .l = (*z).c2rust_unnamed_0.c2rust_unnamed.c;
        }
        106 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .l = (*z).c2rust_unnamed_1.c2rust_unnamed.d;
        }
        107 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .l = (*z).c2rust_unnamed_1.c2rust_unnamed.e;
        }
        108 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .l = (*z).c2rust_unnamed_2.c2rust_unnamed.h;
        }
        109 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .l = (*z).c2rust_unnamed_2.c2rust_unnamed.l;
        }
        126 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed.c2rust_unnamed.a = rb(z, (*z).c2rust_unnamed_2.hl);
        }
        70 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_0.c2rust_unnamed.b = rb(z, (*z).c2rust_unnamed_2.hl);
        }
        78 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_0.c2rust_unnamed.c = rb(z, (*z).c2rust_unnamed_2.hl);
        }
        86 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_1.c2rust_unnamed.d = rb(z, (*z).c2rust_unnamed_2.hl);
        }
        94 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_1.c2rust_unnamed.e = rb(z, (*z).c2rust_unnamed_2.hl);
        }
        102 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_2.c2rust_unnamed.h = rb(z, (*z).c2rust_unnamed_2.hl);
        }
        110 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_2.c2rust_unnamed.l = rb(z, (*z).c2rust_unnamed_2.hl);
        }
        119 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            wb(z, (*z).c2rust_unnamed_2.hl, (*z).c2rust_unnamed.c2rust_unnamed.a);
        }
        112 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            wb(z, (*z).c2rust_unnamed_2.hl, (*z).c2rust_unnamed_0.c2rust_unnamed.b);
        }
        113 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            wb(z, (*z).c2rust_unnamed_2.hl, (*z).c2rust_unnamed_0.c2rust_unnamed.c);
        }
        114 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            wb(z, (*z).c2rust_unnamed_2.hl, (*z).c2rust_unnamed_1.c2rust_unnamed.d);
        }
        115 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            wb(z, (*z).c2rust_unnamed_2.hl, (*z).c2rust_unnamed_1.c2rust_unnamed.e);
        }
        116 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            wb(z, (*z).c2rust_unnamed_2.hl, (*z).c2rust_unnamed_2.c2rust_unnamed.h);
        }
        117 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            wb(z, (*z).c2rust_unnamed_2.hl, (*z).c2rust_unnamed_2.c2rust_unnamed.l);
        }
        62 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed.c2rust_unnamed.a = nextb(z);
        }
        6 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_0.c2rust_unnamed.b = nextb(z);
        }
        14 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_0.c2rust_unnamed.c = nextb(z);
        }
        22 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_1.c2rust_unnamed.d = nextb(z);
        }
        30 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_1.c2rust_unnamed.e = nextb(z);
        }
        38 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_2.c2rust_unnamed.h = nextb(z);
        }
        46 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_2.c2rust_unnamed.l = nextb(z);
        }
        54 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            wb(z, (*z).c2rust_unnamed_2.hl, nextb(z));
        }
        10 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed.c2rust_unnamed.a = rb(z, (*z).c2rust_unnamed_0.bc);
            (*z)
                .mem_ptr = ((*z).c2rust_unnamed_0.bc as libc::c_int + 1 as libc::c_int)
                as uint16_t;
        }
        26 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed.c2rust_unnamed.a = rb(z, (*z).c2rust_unnamed_1.de);
            (*z)
                .mem_ptr = ((*z).c2rust_unnamed_1.de as libc::c_int + 1 as libc::c_int)
                as uint16_t;
        }
        58 => {
            cyc = cyc.wrapping_add(13 as libc::c_int as libc::c_uint);
            let addr: uint16_t = nextw(z);
            (*z).c2rust_unnamed.c2rust_unnamed.a = rb(z, addr);
            (*z).mem_ptr = (addr as libc::c_int + 1 as libc::c_int) as uint16_t;
        }
        2 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            wb(z, (*z).c2rust_unnamed_0.bc, (*z).c2rust_unnamed.c2rust_unnamed.a);
            (*z)
                .mem_ptr = (((*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int)
                << 8 as libc::c_int
                | (*z).c2rust_unnamed_0.bc as libc::c_int + 1 as libc::c_int
                    & 0xff as libc::c_int) as uint16_t;
        }
        18 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            wb(z, (*z).c2rust_unnamed_1.de, (*z).c2rust_unnamed.c2rust_unnamed.a);
            (*z)
                .mem_ptr = (((*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int)
                << 8 as libc::c_int
                | (*z).c2rust_unnamed_1.de as libc::c_int + 1 as libc::c_int
                    & 0xff as libc::c_int) as uint16_t;
        }
        50 => {
            cyc = cyc.wrapping_add(13 as libc::c_int as libc::c_uint);
            let addr_0: uint16_t = nextw(z);
            wb(z, addr_0, (*z).c2rust_unnamed.c2rust_unnamed.a);
            (*z)
                .mem_ptr = (((*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int)
                << 8 as libc::c_int
                | addr_0 as libc::c_int + 1 as libc::c_int & 0xff as libc::c_int)
                as uint16_t;
        }
        1 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_0.bc = nextw(z);
        }
        17 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_1.de = nextw(z);
        }
        33 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_2.hl = nextw(z);
        }
        49 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            (*z).sp = nextw(z);
        }
        42 => {
            cyc = cyc.wrapping_add(16 as libc::c_int as libc::c_uint);
            let addr_1: uint16_t = nextw(z);
            (*z).c2rust_unnamed_2.hl = rw(z, addr_1);
            (*z).mem_ptr = (addr_1 as libc::c_int + 1 as libc::c_int) as uint16_t;
        }
        34 => {
            cyc = cyc.wrapping_add(16 as libc::c_int as libc::c_uint);
            let addr_2: uint16_t = nextw(z);
            ww(z, addr_2, (*z).c2rust_unnamed_2.hl);
            (*z).mem_ptr = (addr_2 as libc::c_int + 1 as libc::c_int) as uint16_t;
        }
        249 => {
            cyc = cyc.wrapping_add(6 as libc::c_int as libc::c_uint);
            (*z).sp = (*z).c2rust_unnamed_2.hl;
        }
        235 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            let de: uint16_t = (*z).c2rust_unnamed_1.de;
            (*z).c2rust_unnamed_1.de = (*z).c2rust_unnamed_2.hl;
            (*z).c2rust_unnamed_2.hl = de;
        }
        227 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            let val: uint16_t = rw(z, (*z).sp);
            ww(z, (*z).sp, (*z).c2rust_unnamed_2.hl);
            (*z).c2rust_unnamed_2.hl = val;
            (*z).mem_ptr = val;
        }
        135 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        128 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_0.c2rust_unnamed.b as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        129 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_0.c2rust_unnamed.c as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        130 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_1.c2rust_unnamed.d as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        131 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_1.c2rust_unnamed.e as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        132 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_2.c2rust_unnamed.h as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        133 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_2.c2rust_unnamed.l as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        134 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                rb(z, (*z).c2rust_unnamed_2.hl) as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        198 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                nextb(z) as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        143 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        144 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_0.c2rust_unnamed.b as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        145 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_0.c2rust_unnamed.c as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        146 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_1.c2rust_unnamed.d as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        147 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_1.c2rust_unnamed.e as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        148 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_2.c2rust_unnamed.h as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        149 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*z).c2rust_unnamed_2.c2rust_unnamed.l as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        150 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                rb(z, (*z).c2rust_unnamed_2.hl) as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        214 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                nextb(z) as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        159 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            addhl(z, (*z).c2rust_unnamed_0.bc);
        }
        25 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            addhl(z, (*z).c2rust_unnamed_1.de);
        }
        41 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            addhl(z, (*z).c2rust_unnamed_2.hl);
        }
        57 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            addhl(z, (*z).sp);
        }
        243 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z).set_iff2(0 as libc::c_int != 0);
            (*z).set_iff1((*z).iff2());
        }
        251 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z).iff_delay = 1 as libc::c_int as uint8_t;
        }
        0 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
        }
        118 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z).set_halted(1 as libc::c_int != 0);
        }
        60 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = inc(z, (*z).c2rust_unnamed.c2rust_unnamed.a);
        }
        4 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = inc(z, (*z).c2rust_unnamed_0.c2rust_unnamed.b);
        }
        12 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = inc(z, (*z).c2rust_unnamed_0.c2rust_unnamed.c);
        }
        20 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = inc(z, (*z).c2rust_unnamed_1.c2rust_unnamed.d);
        }
        28 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = inc(z, (*z).c2rust_unnamed_1.c2rust_unnamed.e);
        }
        36 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .h = inc(z, (*z).c2rust_unnamed_2.c2rust_unnamed.h);
        }
        44 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .l = inc(z, (*z).c2rust_unnamed_2.c2rust_unnamed.l);
        }
        52 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            let mut result: uint8_t = inc(z, rb(z, (*z).c2rust_unnamed_2.hl));
            wb(z, (*z).c2rust_unnamed_2.hl, result);
        }
        61 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = dec(z, (*z).c2rust_unnamed.c2rust_unnamed.a);
        }
        5 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = dec(z, (*z).c2rust_unnamed_0.c2rust_unnamed.b);
        }
        13 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = dec(z, (*z).c2rust_unnamed_0.c2rust_unnamed.c);
        }
        21 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = dec(z, (*z).c2rust_unnamed_1.c2rust_unnamed.d);
        }
        29 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = dec(z, (*z).c2rust_unnamed_1.c2rust_unnamed.e);
        }
        37 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .h = dec(z, (*z).c2rust_unnamed_2.c2rust_unnamed.h);
        }
        45 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .l = dec(z, (*z).c2rust_unnamed_2.c2rust_unnamed.l);
        }
        53 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            let mut result_0: uint8_t = dec(z, rb(z, (*z).c2rust_unnamed_2.hl));
            wb(z, (*z).c2rust_unnamed_2.hl, result_0);
        }
        3 => {
            cyc = cyc.wrapping_add(6 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_0.bc = ((*z).c2rust_unnamed_0.bc).wrapping_add(1);
        }
        19 => {
            cyc = cyc.wrapping_add(6 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_1.de = ((*z).c2rust_unnamed_1.de).wrapping_add(1);
        }
        35 => {
            cyc = cyc.wrapping_add(6 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_2.hl = ((*z).c2rust_unnamed_2.hl).wrapping_add(1);
        }
        51 => {
            cyc = cyc.wrapping_add(6 as libc::c_int as libc::c_uint);
            (*z).sp = ((*z).sp).wrapping_add(1);
        }
        11 => {
            cyc = cyc.wrapping_add(6 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_0.bc = ((*z).c2rust_unnamed_0.bc).wrapping_sub(1);
        }
        27 => {
            cyc = cyc.wrapping_add(6 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_1.de = ((*z).c2rust_unnamed_1.de).wrapping_sub(1);
        }
        43 => {
            cyc = cyc.wrapping_add(6 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_2.hl = ((*z).c2rust_unnamed_2.hl).wrapping_sub(1);
        }
        59 => {
            cyc = cyc.wrapping_add(6 as libc::c_int as libc::c_uint);
            (*z).sp = ((*z).sp).wrapping_sub(1);
        }
        39 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            daa(z);
        }
        47 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = !((*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int) as uint8_t;
            flag_set(z, nf, 1 as libc::c_int != 0);
            flag_set(z, hf, 1 as libc::c_int != 0);
            flag_set(
                z,
                xf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int >> 3 as libc::c_int
                    & 1 as libc::c_int != 0,
            );
            flag_set(
                z,
                yf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int >> 5 as libc::c_int
                    & 1 as libc::c_int != 0,
            );
        }
        55 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            flag_set(z, cf, 1 as libc::c_int != 0);
            flag_set(z, nf, 0 as libc::c_int != 0);
            flag_set(z, hf, 0 as libc::c_int != 0);
            flag_set(
                z,
                xf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int >> 3 as libc::c_int
                    & 1 as libc::c_int != 0,
            );
            flag_set(
                z,
                yf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int >> 5 as libc::c_int
                    & 1 as libc::c_int != 0,
            );
        }
        63 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            flag_set(z, hf, flag_get(z, cf));
            flag_set(z, cf, !flag_get(z, cf));
            flag_set(z, nf, 0 as libc::c_int != 0);
            flag_set(
                z,
                xf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int >> 3 as libc::c_int
                    & 1 as libc::c_int != 0,
            );
            flag_set(
                z,
                yf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int >> 5 as libc::c_int
                    & 1 as libc::c_int != 0,
            );
        }
        7 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            flag_set(
                z,
                cf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int >> 7 as libc::c_int
                    != 0,
            );
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (((*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int)
                << 1 as libc::c_int | flag_get(z, cf) as libc::c_int) as uint8_t;
            flag_set(z, nf, 0 as libc::c_int != 0);
            flag_set(z, hf, 0 as libc::c_int != 0);
            flag_set(
                z,
                xf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int >> 3 as libc::c_int
                    & 1 as libc::c_int != 0,
            );
            flag_set(
                z,
                yf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int >> 5 as libc::c_int
                    & 1 as libc::c_int != 0,
            );
        }
        15 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            flag_set(
                z,
                cf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int & 1 as libc::c_int
                    != 0,
            );
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = ((*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int
                >> 1 as libc::c_int
                | (flag_get(z, cf) as libc::c_int) << 7 as libc::c_int) as uint8_t;
            flag_set(z, nf, 0 as libc::c_int != 0);
            flag_set(z, hf, 0 as libc::c_int != 0);
            flag_set(
                z,
                xf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int >> 3 as libc::c_int
                    & 1 as libc::c_int != 0,
            );
            flag_set(
                z,
                yf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int >> 5 as libc::c_int
                    & 1 as libc::c_int != 0,
            );
        }
        23 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            let cy: bool = flag_get(z, cf);
            flag_set(
                z,
                cf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int >> 7 as libc::c_int
                    != 0,
            );
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (((*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int)
                << 1 as libc::c_int | cy as libc::c_int) as uint8_t;
            flag_set(z, nf, 0 as libc::c_int != 0);
            flag_set(z, hf, 0 as libc::c_int != 0);
            flag_set(
                z,
                xf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int >> 3 as libc::c_int
                    & 1 as libc::c_int != 0,
            );
            flag_set(
                z,
                yf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int >> 5 as libc::c_int
                    & 1 as libc::c_int != 0,
            );
        }
        31 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            let cy_0: bool = flag_get(z, cf);
            flag_set(
                z,
                cf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int & 1 as libc::c_int
                    != 0,
            );
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = ((*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int
                >> 1 as libc::c_int | (cy_0 as libc::c_int) << 7 as libc::c_int)
                as uint8_t;
            flag_set(z, nf, 0 as libc::c_int != 0);
            flag_set(z, hf, 0 as libc::c_int != 0);
            flag_set(
                z,
                xf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int >> 3 as libc::c_int
                    & 1 as libc::c_int != 0,
            );
            flag_set(
                z,
                yf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int >> 5 as libc::c_int
                    & 1 as libc::c_int != 0,
            );
        }
        167 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            land(z, (*z).c2rust_unnamed.c2rust_unnamed.a);
        }
        160 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            land(z, (*z).c2rust_unnamed_0.c2rust_unnamed.b);
        }
        161 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            land(z, (*z).c2rust_unnamed_0.c2rust_unnamed.c);
        }
        162 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            land(z, (*z).c2rust_unnamed_1.c2rust_unnamed.d);
        }
        163 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            land(z, (*z).c2rust_unnamed_1.c2rust_unnamed.e);
        }
        164 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            land(z, (*z).c2rust_unnamed_2.c2rust_unnamed.h);
        }
        165 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            land(z, (*z).c2rust_unnamed_2.c2rust_unnamed.l);
        }
        166 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            land(z, rb(z, (*z).c2rust_unnamed_2.hl));
        }
        230 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            land(z, nextb(z));
        }
        175 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            lxor(z, (*z).c2rust_unnamed.c2rust_unnamed.a);
        }
        168 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            lxor(z, (*z).c2rust_unnamed_0.c2rust_unnamed.b);
        }
        169 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            lxor(z, (*z).c2rust_unnamed_0.c2rust_unnamed.c);
        }
        170 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            lxor(z, (*z).c2rust_unnamed_1.c2rust_unnamed.d);
        }
        171 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            lxor(z, (*z).c2rust_unnamed_1.c2rust_unnamed.e);
        }
        172 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            lxor(z, (*z).c2rust_unnamed_2.c2rust_unnamed.h);
        }
        173 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            lxor(z, (*z).c2rust_unnamed_2.c2rust_unnamed.l);
        }
        174 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            lxor(z, rb(z, (*z).c2rust_unnamed_2.hl));
        }
        238 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            lxor(z, nextb(z));
        }
        183 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            lor(z, (*z).c2rust_unnamed.c2rust_unnamed.a);
        }
        176 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            lor(z, (*z).c2rust_unnamed_0.c2rust_unnamed.b);
        }
        177 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            lor(z, (*z).c2rust_unnamed_0.c2rust_unnamed.c);
        }
        178 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            lor(z, (*z).c2rust_unnamed_1.c2rust_unnamed.d);
        }
        179 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            lor(z, (*z).c2rust_unnamed_1.c2rust_unnamed.e);
        }
        180 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            lor(z, (*z).c2rust_unnamed_2.c2rust_unnamed.h);
        }
        181 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            lor(z, (*z).c2rust_unnamed_2.c2rust_unnamed.l);
        }
        182 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            lor(z, rb(z, (*z).c2rust_unnamed_2.hl));
        }
        246 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            lor(z, nextb(z));
        }
        191 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            cp(z, (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t);
        }
        184 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            cp(z, (*z).c2rust_unnamed_0.c2rust_unnamed.b as uint32_t);
        }
        185 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            cp(z, (*z).c2rust_unnamed_0.c2rust_unnamed.c as uint32_t);
        }
        186 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            cp(z, (*z).c2rust_unnamed_1.c2rust_unnamed.d as uint32_t);
        }
        187 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            cp(z, (*z).c2rust_unnamed_1.c2rust_unnamed.e as uint32_t);
        }
        188 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            cp(z, (*z).c2rust_unnamed_2.c2rust_unnamed.h as uint32_t);
        }
        189 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            cp(z, (*z).c2rust_unnamed_2.c2rust_unnamed.l as uint32_t);
        }
        190 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            cp(z, rb(z, (*z).c2rust_unnamed_2.hl) as uint32_t);
        }
        254 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            cp(z, nextb(z) as uint32_t);
        }
        195 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            jump(z, nextw(z));
        }
        194 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            cond_jump(z, flag_get(z, zf) as libc::c_int == 0 as libc::c_int);
        }
        202 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            cond_jump(z, flag_get(z, zf) as libc::c_int == 1 as libc::c_int);
        }
        210 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            cond_jump(z, flag_get(z, cf) as libc::c_int == 0 as libc::c_int);
        }
        218 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            cond_jump(z, flag_get(z, cf) as libc::c_int == 1 as libc::c_int);
        }
        226 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            cond_jump(z, flag_get(z, pf) as libc::c_int == 0 as libc::c_int);
        }
        234 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            cond_jump(z, flag_get(z, pf) as libc::c_int == 1 as libc::c_int);
        }
        242 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            cond_jump(z, flag_get(z, sf) as libc::c_int == 0 as libc::c_int);
        }
        250 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            cond_jump(z, flag_get(z, sf) as libc::c_int == 1 as libc::c_int);
        }
        16 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = ((*z).c2rust_unnamed_0.c2rust_unnamed.b).wrapping_sub(1);
            cyc = cyc
                .wrapping_add(
                    cond_jr(
                        z,
                        (*z).c2rust_unnamed_0.c2rust_unnamed.b as libc::c_int
                            != 0 as libc::c_int,
                    ),
                );
        }
        24 => {
            cyc = cyc.wrapping_add(12 as libc::c_int as libc::c_uint);
            jr(z, nextb(z) as int8_t);
        }
        32 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            cyc = cyc
                .wrapping_add(
                    cond_jr(z, flag_get(z, zf) as libc::c_int == 0 as libc::c_int),
                );
        }
        40 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            cyc = cyc
                .wrapping_add(
                    cond_jr(z, flag_get(z, zf) as libc::c_int == 1 as libc::c_int),
                );
        }
        48 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            cyc = cyc
                .wrapping_add(
                    cond_jr(z, flag_get(z, cf) as libc::c_int == 0 as libc::c_int),
                );
        }
        56 => {
            cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
            cyc = cyc
                .wrapping_add(
                    cond_jr(z, flag_get(z, cf) as libc::c_int == 1 as libc::c_int),
                );
        }
        233 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            (*z).pc = (*z).c2rust_unnamed_2.hl;
        }
        205 => {
            cyc = cyc.wrapping_add(17 as libc::c_int as libc::c_uint);
            call(z, nextw(z));
        }
        196 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            cyc = cyc
                .wrapping_add(
                    cond_call(z, flag_get(z, zf) as libc::c_int == 0 as libc::c_int),
                );
        }
        204 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            cyc = cyc
                .wrapping_add(
                    cond_call(z, flag_get(z, zf) as libc::c_int == 1 as libc::c_int),
                );
        }
        212 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            cyc = cyc
                .wrapping_add(
                    cond_call(z, flag_get(z, cf) as libc::c_int == 0 as libc::c_int),
                );
        }
        220 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            cyc = cyc
                .wrapping_add(
                    cond_call(z, flag_get(z, cf) as libc::c_int == 1 as libc::c_int),
                );
        }
        228 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            cyc = cyc
                .wrapping_add(
                    cond_call(z, flag_get(z, pf) as libc::c_int == 0 as libc::c_int),
                );
        }
        236 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            cyc = cyc
                .wrapping_add(
                    cond_call(z, flag_get(z, pf) as libc::c_int == 1 as libc::c_int),
                );
        }
        244 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            cyc = cyc
                .wrapping_add(
                    cond_call(z, flag_get(z, sf) as libc::c_int == 0 as libc::c_int),
                );
        }
        252 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            cyc = cyc
                .wrapping_add(
                    cond_call(z, flag_get(z, sf) as libc::c_int == 1 as libc::c_int),
                );
        }
        201 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            ret(z);
        }
        192 => {
            cyc = cyc.wrapping_add(5 as libc::c_int as libc::c_uint);
            cyc = cyc
                .wrapping_add(
                    cond_ret(z, flag_get(z, zf) as libc::c_int == 0 as libc::c_int),
                );
        }
        200 => {
            cyc = cyc.wrapping_add(5 as libc::c_int as libc::c_uint);
            cyc = cyc
                .wrapping_add(
                    cond_ret(z, flag_get(z, zf) as libc::c_int == 1 as libc::c_int),
                );
        }
        208 => {
            cyc = cyc.wrapping_add(5 as libc::c_int as libc::c_uint);
            cyc = cyc
                .wrapping_add(
                    cond_ret(z, flag_get(z, cf) as libc::c_int == 0 as libc::c_int),
                );
        }
        216 => {
            cyc = cyc.wrapping_add(5 as libc::c_int as libc::c_uint);
            cyc = cyc
                .wrapping_add(
                    cond_ret(z, flag_get(z, cf) as libc::c_int == 1 as libc::c_int),
                );
        }
        224 => {
            cyc = cyc.wrapping_add(5 as libc::c_int as libc::c_uint);
            cyc = cyc
                .wrapping_add(
                    cond_ret(z, flag_get(z, pf) as libc::c_int == 0 as libc::c_int),
                );
        }
        232 => {
            cyc = cyc.wrapping_add(5 as libc::c_int as libc::c_uint);
            cyc = cyc
                .wrapping_add(
                    cond_ret(z, flag_get(z, pf) as libc::c_int == 1 as libc::c_int),
                );
        }
        240 => {
            cyc = cyc.wrapping_add(5 as libc::c_int as libc::c_uint);
            cyc = cyc
                .wrapping_add(
                    cond_ret(z, flag_get(z, sf) as libc::c_int == 0 as libc::c_int),
                );
        }
        248 => {
            cyc = cyc.wrapping_add(5 as libc::c_int as libc::c_uint);
            cyc = cyc
                .wrapping_add(
                    cond_ret(z, flag_get(z, sf) as libc::c_int == 1 as libc::c_int),
                );
        }
        199 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            call(z, 0 as libc::c_int as uint16_t);
        }
        207 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            call(z, 0x8 as libc::c_int as uint16_t);
        }
        215 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            call(z, 0x10 as libc::c_int as uint16_t);
        }
        223 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            call(z, 0x18 as libc::c_int as uint16_t);
        }
        231 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            call(z, 0x20 as libc::c_int as uint16_t);
        }
        239 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            call(z, 0x28 as libc::c_int as uint16_t);
        }
        247 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            call(z, 0x30 as libc::c_int as uint16_t);
        }
        255 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            call(z, 0x38 as libc::c_int as uint16_t);
        }
        197 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            pushw(z, (*z).c2rust_unnamed_0.bc);
        }
        213 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            pushw(z, (*z).c2rust_unnamed_1.de);
        }
        229 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            pushw(z, (*z).c2rust_unnamed_2.hl);
        }
        245 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            pushw(z, (*z).c2rust_unnamed.af);
        }
        193 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_0.bc = popw(z);
        }
        209 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_1.de = popw(z);
        }
        225 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed_2.hl = popw(z);
        }
        241 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed.af = popw(z);
        }
        219 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            let port: uint16_t = (nextb(z) as libc::c_int
                | ((*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int)
                    << 8 as libc::c_int) as uint16_t;
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = ((*z).port_in).expect("non-null function pointer")(z, port);
            (*z).mem_ptr = (port as libc::c_int + 1 as libc::c_int) as uint16_t;
        }
        211 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            let port_0: uint16_t = (nextb(z) as libc::c_int
                | ((*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int)
                    << 8 as libc::c_int) as uint16_t;
            ((*z).port_out)
                .expect(
                    "non-null function pointer",
                )(z, port_0, (*z).c2rust_unnamed.c2rust_unnamed.a);
            (*z)
                .mem_ptr = (port_0 as libc::c_int + 1 as libc::c_int
                & 0xff as libc::c_int
                | ((*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int)
                    << 8 as libc::c_int) as uint16_t;
        }
        8 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            let mut af: uint16_t = (*z).c2rust_unnamed.af;
            (*z).c2rust_unnamed.af = (*z).c2rust_unnamed_3.a_f_;
            (*z).c2rust_unnamed_3.a_f_ = af;
        }
        217 => {
            cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(0 as libc::c_int as libc::c_uint);
            cyc = cyc.wrapping_add(exec_opcode_cb(z, nextb(z)));
        }
        237 => {
            cyc = cyc.wrapping_add(0 as libc::c_int as libc::c_uint);
            cyc = cyc.wrapping_add(exec_opcode_ed(z, nextb(z)));
        }
        221 => {
            cyc = cyc.wrapping_add(0 as libc::c_int as libc::c_uint);
            cyc = cyc.wrapping_add(exec_opcode_ddfd(z, nextb(z), &mut (*z).ix));
        }
        253 => {
            cyc = cyc.wrapping_add(0 as libc::c_int as libc::c_uint);
            cyc = cyc.wrapping_add(exec_opcode_ddfd(z, nextb(z), &mut (*z).iy));
        }
        _ => {}
    }
    return cyc;
}
unsafe extern "C" fn exec_opcode_ddfd(
    z: *mut z80,
    mut opcode: uint8_t,
    iz: *mut uint16_t,
) -> libc::c_uint {
    let mut cyc: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    inc_r(z);
    match opcode as libc::c_int {
        225 => {
            cyc = cyc.wrapping_add(14 as libc::c_int as libc::c_uint);
            *iz = popw(z);
        }
        229 => {
            cyc = cyc.wrapping_add(15 as libc::c_int as libc::c_uint);
            pushw(z, *iz);
        }
        233 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            jump(z, *iz);
        }
        9 => {
            cyc = cyc.wrapping_add(15 as libc::c_int as libc::c_uint);
            addiz(z, iz, (*z).c2rust_unnamed_0.bc);
        }
        25 => {
            cyc = cyc.wrapping_add(15 as libc::c_int as libc::c_uint);
            addiz(z, iz, (*z).c2rust_unnamed_1.de);
        }
        41 => {
            cyc = cyc.wrapping_add(15 as libc::c_int as libc::c_uint);
            addiz(z, iz, *iz);
        }
        57 => {
            cyc = cyc.wrapping_add(15 as libc::c_int as libc::c_uint);
            addiz(z, iz, (*z).sp);
        }
        132 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*iz as libc::c_int >> 8 as libc::c_int) as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        133 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*iz as libc::c_int & 0xff as libc::c_int) as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        140 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*iz as libc::c_int >> 8 as libc::c_int) as uint32_t,
                flag_get(z, cf),
            );
        }
        141 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*iz as libc::c_int & 0xff as libc::c_int) as uint32_t,
                flag_get(z, cf),
            );
        }
        134 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = addb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                rb(z, displace(z, *iz, nextb(z) as int8_t)) as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        142 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                rb(z, displace(z, *iz, nextb(z) as int8_t)) as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        158 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
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
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*iz as libc::c_int >> 8 as libc::c_int) as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        149 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*iz as libc::c_int & 0xff as libc::c_int) as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        156 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*iz as libc::c_int >> 8 as libc::c_int) as uint32_t,
                flag_get(z, cf),
            );
        }
        157 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                (*iz as libc::c_int & 0xff as libc::c_int) as uint32_t,
                flag_get(z, cf),
            );
        }
        166 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            land(z, rb(z, displace(z, *iz, nextb(z) as int8_t)));
        }
        164 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            land(z, (*iz as libc::c_int >> 8 as libc::c_int) as uint8_t);
        }
        165 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            land(z, (*iz as libc::c_int & 0xff as libc::c_int) as uint8_t);
        }
        174 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            lxor(z, rb(z, displace(z, *iz, nextb(z) as int8_t)));
        }
        172 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            lxor(z, (*iz as libc::c_int >> 8 as libc::c_int) as uint8_t);
        }
        173 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            lxor(z, (*iz as libc::c_int & 0xff as libc::c_int) as uint8_t);
        }
        182 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            lor(z, rb(z, displace(z, *iz, nextb(z) as int8_t)));
        }
        180 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            lor(z, (*iz as libc::c_int >> 8 as libc::c_int) as uint8_t);
        }
        181 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            lor(z, (*iz as libc::c_int & 0xff as libc::c_int) as uint8_t);
        }
        190 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            cp(z, rb(z, displace(z, *iz, nextb(z) as int8_t)) as uint32_t);
        }
        188 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            cp(z, (*iz as libc::c_int >> 8 as libc::c_int) as uint32_t);
        }
        189 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            cp(z, (*iz as libc::c_int & 0xff as libc::c_int) as uint32_t);
        }
        35 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            *iz = (*iz as libc::c_int + 1 as libc::c_int) as uint16_t;
        }
        43 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            *iz = (*iz as libc::c_int - 1 as libc::c_int) as uint16_t;
        }
        52 => {
            cyc = cyc.wrapping_add(23 as libc::c_int as libc::c_uint);
            let mut addr: uint16_t = displace(z, *iz, nextb(z) as int8_t);
            wb(z, addr, inc(z, rb(z, addr)));
        }
        53 => {
            cyc = cyc.wrapping_add(23 as libc::c_int as libc::c_uint);
            let mut addr_0: uint16_t = displace(z, *iz, nextb(z) as int8_t);
            wb(z, addr_0, dec(z, rb(z, addr_0)));
        }
        36 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            *iz = (*iz as libc::c_int & 0xff as libc::c_int
                | (inc(z, (*iz as libc::c_int >> 8 as libc::c_int) as uint8_t)
                    as libc::c_int) << 8 as libc::c_int) as uint16_t;
        }
        37 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            *iz = (*iz as libc::c_int & 0xff as libc::c_int
                | (dec(z, (*iz as libc::c_int >> 8 as libc::c_int) as uint8_t)
                    as libc::c_int) << 8 as libc::c_int) as uint16_t;
        }
        44 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            *iz = ((*iz as libc::c_int >> 8 as libc::c_int) << 8 as libc::c_int
                | inc(z, (*iz as libc::c_int & 0xff as libc::c_int) as uint8_t)
                    as libc::c_int) as uint16_t;
        }
        45 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            *iz = ((*iz as libc::c_int >> 8 as libc::c_int) << 8 as libc::c_int
                | dec(z, (*iz as libc::c_int & 0xff as libc::c_int) as uint8_t)
                    as libc::c_int) as uint16_t;
        }
        42 => {
            cyc = cyc.wrapping_add(20 as libc::c_int as libc::c_uint);
            *iz = rw(z, nextw(z));
        }
        34 => {
            cyc = cyc.wrapping_add(20 as libc::c_int as libc::c_uint);
            ww(z, nextw(z), *iz);
        }
        33 => {
            cyc = cyc.wrapping_add(14 as libc::c_int as libc::c_uint);
            *iz = nextw(z);
        }
        54 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            let mut addr_1: uint16_t = displace(z, *iz, nextb(z) as int8_t);
            wb(z, addr_1, nextb(z));
        }
        112 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            wb(
                z,
                displace(z, *iz, nextb(z) as int8_t),
                (*z).c2rust_unnamed_0.c2rust_unnamed.b,
            );
        }
        113 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            wb(
                z,
                displace(z, *iz, nextb(z) as int8_t),
                (*z).c2rust_unnamed_0.c2rust_unnamed.c,
            );
        }
        114 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            wb(
                z,
                displace(z, *iz, nextb(z) as int8_t),
                (*z).c2rust_unnamed_1.c2rust_unnamed.d,
            );
        }
        115 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            wb(
                z,
                displace(z, *iz, nextb(z) as int8_t),
                (*z).c2rust_unnamed_1.c2rust_unnamed.e,
            );
        }
        116 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            wb(
                z,
                displace(z, *iz, nextb(z) as int8_t),
                (*z).c2rust_unnamed_2.c2rust_unnamed.h,
            );
        }
        117 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            wb(
                z,
                displace(z, *iz, nextb(z) as int8_t),
                (*z).c2rust_unnamed_2.c2rust_unnamed.l,
            );
        }
        119 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            wb(
                z,
                displace(z, *iz, nextb(z) as int8_t),
                (*z).c2rust_unnamed.c2rust_unnamed.a,
            );
        }
        70 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = rb(z, displace(z, *iz, nextb(z) as int8_t));
        }
        78 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = rb(z, displace(z, *iz, nextb(z) as int8_t));
        }
        86 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = rb(z, displace(z, *iz, nextb(z) as int8_t));
        }
        94 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = rb(z, displace(z, *iz, nextb(z) as int8_t));
        }
        102 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .h = rb(z, displace(z, *iz, nextb(z) as int8_t));
        }
        110 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_2
                .c2rust_unnamed
                .l = rb(z, displace(z, *iz, nextb(z) as int8_t));
        }
        126 => {
            cyc = cyc.wrapping_add(19 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = rb(z, displace(z, *iz, nextb(z) as int8_t));
        }
        68 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = (*iz as libc::c_int >> 8 as libc::c_int) as uint8_t;
        }
        76 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = (*iz as libc::c_int >> 8 as libc::c_int) as uint8_t;
        }
        84 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = (*iz as libc::c_int >> 8 as libc::c_int) as uint8_t;
        }
        92 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = (*iz as libc::c_int >> 8 as libc::c_int) as uint8_t;
        }
        124 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (*iz as libc::c_int >> 8 as libc::c_int) as uint8_t;
        }
        69 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .b = (*iz as libc::c_int & 0xff as libc::c_int) as uint8_t;
        }
        77 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c = (*iz as libc::c_int & 0xff as libc::c_int) as uint8_t;
        }
        85 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .d = (*iz as libc::c_int & 0xff as libc::c_int) as uint8_t;
        }
        93 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed_1
                .c2rust_unnamed
                .e = (*iz as libc::c_int & 0xff as libc::c_int) as uint8_t;
        }
        125 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (*iz as libc::c_int & 0xff as libc::c_int) as uint8_t;
        }
        96 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            *iz = (*iz as libc::c_int & 0xff as libc::c_int
                | ((*z).c2rust_unnamed_0.c2rust_unnamed.b as libc::c_int)
                    << 8 as libc::c_int) as uint16_t;
        }
        97 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            *iz = (*iz as libc::c_int & 0xff as libc::c_int
                | ((*z).c2rust_unnamed_0.c2rust_unnamed.c as libc::c_int)
                    << 8 as libc::c_int) as uint16_t;
        }
        98 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            *iz = (*iz as libc::c_int & 0xff as libc::c_int
                | ((*z).c2rust_unnamed_1.c2rust_unnamed.d as libc::c_int)
                    << 8 as libc::c_int) as uint16_t;
        }
        99 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            *iz = (*iz as libc::c_int & 0xff as libc::c_int
                | ((*z).c2rust_unnamed_1.c2rust_unnamed.e as libc::c_int)
                    << 8 as libc::c_int) as uint16_t;
        }
        100 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
        }
        101 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            *iz = ((*iz as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int
                | *iz as libc::c_int & 0xff as libc::c_int) as uint16_t;
        }
        103 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            *iz = (*iz as libc::c_int & 0xff as libc::c_int
                | ((*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int)
                    << 8 as libc::c_int) as uint16_t;
        }
        38 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            *iz = (*iz as libc::c_int & 0xff as libc::c_int
                | (nextb(z) as libc::c_int) << 8 as libc::c_int) as uint16_t;
        }
        104 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            *iz = ((*iz as libc::c_int >> 8 as libc::c_int) << 8 as libc::c_int
                | (*z).c2rust_unnamed_0.c2rust_unnamed.b as libc::c_int) as uint16_t;
        }
        105 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            *iz = ((*iz as libc::c_int >> 8 as libc::c_int) << 8 as libc::c_int
                | (*z).c2rust_unnamed_0.c2rust_unnamed.c as libc::c_int) as uint16_t;
        }
        106 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            *iz = ((*iz as libc::c_int >> 8 as libc::c_int) << 8 as libc::c_int
                | (*z).c2rust_unnamed_1.c2rust_unnamed.d as libc::c_int) as uint16_t;
        }
        107 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            *iz = ((*iz as libc::c_int >> 8 as libc::c_int) << 8 as libc::c_int
                | (*z).c2rust_unnamed_1.c2rust_unnamed.e as libc::c_int) as uint16_t;
        }
        108 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            *iz = ((*iz as libc::c_int >> 8 as libc::c_int) << 8 as libc::c_int
                | *iz as libc::c_int >> 8 as libc::c_int) as uint16_t;
        }
        109 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
        }
        111 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            *iz = ((*iz as libc::c_int >> 8 as libc::c_int) << 8 as libc::c_int
                | (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int) as uint16_t;
        }
        46 => {
            cyc = cyc.wrapping_add(11 as libc::c_int as libc::c_uint);
            *iz = ((*iz as libc::c_int >> 8 as libc::c_int) << 8 as libc::c_int
                | nextb(z) as libc::c_int) as uint16_t;
        }
        249 => {
            cyc = cyc.wrapping_add(10 as libc::c_int as libc::c_uint);
            (*z).sp = *iz;
        }
        227 => {
            cyc = cyc.wrapping_add(23 as libc::c_int as libc::c_uint);
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
                    (4 as libc::c_int as libc::c_uint)
                        .wrapping_add(exec_opcode(z, opcode)),
                );
            (*z)
                .r = ((*z).r as libc::c_int & 0x80 as libc::c_int
                | (*z).r as libc::c_int - 1 as libc::c_int & 0x7f as libc::c_int)
                as uint8_t;
        }
    }
    return cyc;
}
unsafe extern "C" fn exec_opcode_cb(z: *mut z80, mut opcode: uint8_t) -> libc::c_uint {
    let mut cyc: libc::c_uint = 8 as libc::c_int as libc::c_uint;
    inc_r(z);
    let mut x_: uint8_t = (opcode as libc::c_int >> 6 as libc::c_int & 3 as libc::c_int)
        as uint8_t;
    let mut y_: uint8_t = (opcode as libc::c_int >> 3 as libc::c_int & 7 as libc::c_int)
        as uint8_t;
    let mut z_: uint8_t = (opcode as libc::c_int & 7 as libc::c_int) as uint8_t;
    let mut hl: uint8_t = 0 as libc::c_int as uint8_t;
    let mut reg: *mut uint8_t = 0 as *mut uint8_t;
    match z_ as libc::c_int {
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
    match x_ as libc::c_int {
        0 => {
            match y_ as libc::c_int {
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
            if z_ as libc::c_int == 6 as libc::c_int {
                flag_set(
                    z,
                    yf,
                    (*z).mem_ptr as libc::c_int >> 8 as libc::c_int >> 5 as libc::c_int
                        & 1 as libc::c_int != 0,
                );
                flag_set(
                    z,
                    xf,
                    (*z).mem_ptr as libc::c_int >> 8 as libc::c_int >> 3 as libc::c_int
                        & 1 as libc::c_int != 0,
                );
                cyc = cyc.wrapping_add(4 as libc::c_int as libc::c_uint);
            } else {
                flag_set(
                    z,
                    yf,
                    *reg as libc::c_int >> 5 as libc::c_int & 1 as libc::c_int != 0,
                );
                flag_set(
                    z,
                    xf,
                    *reg as libc::c_int >> 3 as libc::c_int & 1 as libc::c_int != 0,
                );
            }
        }
        2 => {
            *reg = (*reg as libc::c_int & !((1 as libc::c_int) << y_ as libc::c_int))
                as uint8_t;
        }
        3 => {
            *reg = (*reg as libc::c_int | (1 as libc::c_int) << y_ as libc::c_int)
                as uint8_t;
        }
        _ => {}
    }
    if x_ as libc::c_int != 1 as libc::c_int && z_ as libc::c_int == 6 as libc::c_int {
        wb(z, (*z).c2rust_unnamed_2.hl, hl);
        cyc = cyc.wrapping_add(7 as libc::c_int as libc::c_uint);
    }
    return cyc;
}
unsafe extern "C" fn exec_opcode_dcb(
    z: *mut z80,
    mut opcode: uint8_t,
    mut addr: uint16_t,
) -> libc::c_uint {
    let mut cyc: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut val: uint8_t = rb(z, addr);
    let mut result: uint8_t = 0 as libc::c_int as uint8_t;
    let mut x_: uint8_t = (opcode as libc::c_int >> 6 as libc::c_int & 3 as libc::c_int)
        as uint8_t;
    let mut y_: uint8_t = (opcode as libc::c_int >> 3 as libc::c_int & 7 as libc::c_int)
        as uint8_t;
    let mut z_: uint8_t = (opcode as libc::c_int & 7 as libc::c_int) as uint8_t;
    match x_ as libc::c_int {
        0 => {
            match y_ as libc::c_int {
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
                addr as libc::c_int >> 8 as libc::c_int >> 5 as libc::c_int
                    & 1 as libc::c_int != 0,
            );
            flag_set(
                z,
                xf,
                addr as libc::c_int >> 8 as libc::c_int >> 3 as libc::c_int
                    & 1 as libc::c_int != 0,
            );
        }
        2 => {
            result = (val as libc::c_int & !((1 as libc::c_int) << y_ as libc::c_int))
                as uint8_t;
        }
        3 => {
            result = (val as libc::c_int | (1 as libc::c_int) << y_ as libc::c_int)
                as uint8_t;
        }
        _ => {}
    }
    if x_ as libc::c_int != 1 as libc::c_int && z_ as libc::c_int != 6 as libc::c_int {
        match z_ as libc::c_int {
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
    if x_ as libc::c_int == 1 as libc::c_int {
        cyc = cyc.wrapping_add(20 as libc::c_int as libc::c_uint);
    } else {
        wb(z, addr, result);
        cyc = cyc.wrapping_add(23 as libc::c_int as libc::c_uint);
    }
    return cyc;
}
unsafe extern "C" fn exec_opcode_ed(z: *mut z80, mut opcode: uint8_t) -> libc::c_uint {
    let mut cyc: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    inc_r(z);
    match opcode as libc::c_int {
        71 => {
            cyc = cyc.wrapping_add(9 as libc::c_int as libc::c_uint);
            (*z).i = (*z).c2rust_unnamed.c2rust_unnamed.a;
        }
        79 => {
            cyc = cyc.wrapping_add(9 as libc::c_int as libc::c_uint);
            (*z).r = (*z).c2rust_unnamed.c2rust_unnamed.a;
        }
        87 => {
            cyc = cyc.wrapping_add(9 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed.c2rust_unnamed.a = (*z).i;
            flag_set(
                z,
                sf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int >> 7 as libc::c_int
                    != 0,
            );
            flag_set(
                z,
                zf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int == 0 as libc::c_int,
            );
            flag_set(z, hf, 0 as libc::c_int != 0);
            flag_set(z, nf, 0 as libc::c_int != 0);
            flag_set(z, pf, (*z).iff2());
        }
        95 => {
            cyc = cyc.wrapping_add(9 as libc::c_int as libc::c_uint);
            (*z).c2rust_unnamed.c2rust_unnamed.a = (*z).r;
            flag_set(
                z,
                sf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int >> 7 as libc::c_int
                    != 0,
            );
            flag_set(
                z,
                zf,
                (*z).c2rust_unnamed.c2rust_unnamed.a as libc::c_int == 0 as libc::c_int,
            );
            flag_set(z, hf, 0 as libc::c_int != 0);
            flag_set(z, nf, 0 as libc::c_int != 0);
            flag_set(z, pf, (*z).iff2());
        }
        69 | 85 | 93 | 101 | 109 | 117 | 125 => {
            cyc = cyc.wrapping_add(14 as libc::c_int as libc::c_uint);
            (*z).set_iff1((*z).iff2());
            ret(z);
        }
        77 => {
            cyc = cyc.wrapping_add(14 as libc::c_int as libc::c_uint);
            ret(z);
        }
        160 => {
            cyc = cyc.wrapping_add(16 as libc::c_int as libc::c_uint);
            ldi(z);
        }
        176 => {
            cyc = cyc.wrapping_add(16 as libc::c_int as libc::c_uint);
            ldi(z);
            if (*z).c2rust_unnamed_0.bc as libc::c_int != 0 as libc::c_int {
                (*z).pc = ((*z).pc as libc::c_int - 2 as libc::c_int) as uint16_t;
                cyc = cyc.wrapping_add(5 as libc::c_int as libc::c_uint);
                (*z).mem_ptr = ((*z).pc as libc::c_int + 1 as libc::c_int) as uint16_t;
            }
        }
        168 => {
            cyc = cyc.wrapping_add(16 as libc::c_int as libc::c_uint);
            ldd(z);
        }
        184 => {
            cyc = cyc.wrapping_add(16 as libc::c_int as libc::c_uint);
            ldd(z);
            if (*z).c2rust_unnamed_0.bc as libc::c_int != 0 as libc::c_int {
                (*z).pc = ((*z).pc as libc::c_int - 2 as libc::c_int) as uint16_t;
                cyc = cyc.wrapping_add(5 as libc::c_int as libc::c_uint);
                (*z).mem_ptr = ((*z).pc as libc::c_int + 1 as libc::c_int) as uint16_t;
            }
        }
        161 => {
            cyc = cyc.wrapping_add(16 as libc::c_int as libc::c_uint);
            cpi(z);
        }
        169 => {
            cyc = cyc.wrapping_add(16 as libc::c_int as libc::c_uint);
            cpd(z);
        }
        177 => {
            cyc = cyc.wrapping_add(16 as libc::c_int as libc::c_uint);
            cpi(z);
            if (*z).c2rust_unnamed_0.bc as libc::c_int != 0 as libc::c_int
                && !flag_get(z, zf)
            {
                (*z).pc = ((*z).pc as libc::c_int - 2 as libc::c_int) as uint16_t;
                cyc = cyc.wrapping_add(5 as libc::c_int as libc::c_uint);
                (*z).mem_ptr = ((*z).pc as libc::c_int + 1 as libc::c_int) as uint16_t;
            } else {
                (*z)
                    .mem_ptr = ((*z).mem_ptr as libc::c_int + 1 as libc::c_int)
                    as uint16_t;
            }
        }
        185 => {
            cyc = cyc.wrapping_add(16 as libc::c_int as libc::c_uint);
            cpd(z);
            if (*z).c2rust_unnamed_0.bc as libc::c_int != 0 as libc::c_int
                && !flag_get(z, zf)
            {
                (*z).pc = ((*z).pc as libc::c_int - 2 as libc::c_int) as uint16_t;
                cyc = cyc.wrapping_add(5 as libc::c_int as libc::c_uint);
            } else {
                (*z)
                    .mem_ptr = ((*z).mem_ptr as libc::c_int + 1 as libc::c_int)
                    as uint16_t;
            }
        }
        64 => {
            cyc = cyc.wrapping_add(12 as libc::c_int as libc::c_uint);
            in_r_c(z, &mut (*z).c2rust_unnamed_0.c2rust_unnamed.b);
        }
        72 => {
            cyc = cyc.wrapping_add(12 as libc::c_int as libc::c_uint);
            in_r_c(z, &mut (*z).c2rust_unnamed_0.c2rust_unnamed.c);
        }
        80 => {
            cyc = cyc.wrapping_add(12 as libc::c_int as libc::c_uint);
            in_r_c(z, &mut (*z).c2rust_unnamed_1.c2rust_unnamed.d);
        }
        88 => {
            cyc = cyc.wrapping_add(12 as libc::c_int as libc::c_uint);
            in_r_c(z, &mut (*z).c2rust_unnamed_1.c2rust_unnamed.e);
        }
        96 => {
            cyc = cyc.wrapping_add(12 as libc::c_int as libc::c_uint);
            in_r_c(z, &mut (*z).c2rust_unnamed_2.c2rust_unnamed.h);
        }
        104 => {
            cyc = cyc.wrapping_add(12 as libc::c_int as libc::c_uint);
            in_r_c(z, &mut (*z).c2rust_unnamed_2.c2rust_unnamed.l);
        }
        112 => {
            cyc = cyc.wrapping_add(12 as libc::c_int as libc::c_uint);
            let mut val: uint8_t = 0;
            in_r_c(z, &mut val);
        }
        120 => {
            cyc = cyc.wrapping_add(12 as libc::c_int as libc::c_uint);
            in_r_c(z, &mut (*z).c2rust_unnamed.c2rust_unnamed.a);
            (*z)
                .mem_ptr = ((*z).c2rust_unnamed_0.bc as libc::c_int + 1 as libc::c_int)
                as uint16_t;
        }
        162 => {
            cyc = cyc.wrapping_add(16 as libc::c_int as libc::c_uint);
            ini(z);
        }
        178 => {
            cyc = cyc.wrapping_add(16 as libc::c_int as libc::c_uint);
            ini(z);
            if (*z).c2rust_unnamed_0.c2rust_unnamed.b as libc::c_int > 0 as libc::c_int {
                (*z).pc = ((*z).pc as libc::c_int - 2 as libc::c_int) as uint16_t;
                cyc = cyc.wrapping_add(5 as libc::c_int as libc::c_uint);
                (*z).mem_ptr = ((*z).pc as libc::c_int + 1 as libc::c_int) as uint16_t;
            }
        }
        170 => {
            cyc = cyc.wrapping_add(16 as libc::c_int as libc::c_uint);
            ind(z);
        }
        186 => {
            cyc = cyc.wrapping_add(16 as libc::c_int as libc::c_uint);
            ind(z);
            if (*z).c2rust_unnamed_0.c2rust_unnamed.b as libc::c_int > 0 as libc::c_int {
                (*z).pc = ((*z).pc as libc::c_int - 2 as libc::c_int) as uint16_t;
                cyc = cyc.wrapping_add(5 as libc::c_int as libc::c_uint);
                (*z).mem_ptr = ((*z).pc as libc::c_int + 1 as libc::c_int) as uint16_t;
            }
        }
        121 => {
            cyc = cyc.wrapping_add(12 as libc::c_int as libc::c_uint);
            outc(z, (*z).c2rust_unnamed.c2rust_unnamed.a);
        }
        65 => {
            cyc = cyc.wrapping_add(12 as libc::c_int as libc::c_uint);
            outc(z, (*z).c2rust_unnamed_0.c2rust_unnamed.b);
        }
        73 => {
            cyc = cyc.wrapping_add(12 as libc::c_int as libc::c_uint);
            outc(z, (*z).c2rust_unnamed_0.c2rust_unnamed.c);
        }
        81 => {
            cyc = cyc.wrapping_add(12 as libc::c_int as libc::c_uint);
            outc(z, (*z).c2rust_unnamed_1.c2rust_unnamed.d);
        }
        89 => {
            cyc = cyc.wrapping_add(12 as libc::c_int as libc::c_uint);
            outc(z, (*z).c2rust_unnamed_1.c2rust_unnamed.e);
        }
        97 => {
            cyc = cyc.wrapping_add(12 as libc::c_int as libc::c_uint);
            outc(z, (*z).c2rust_unnamed_2.c2rust_unnamed.h);
        }
        105 => {
            cyc = cyc.wrapping_add(12 as libc::c_int as libc::c_uint);
            outc(z, (*z).c2rust_unnamed_2.c2rust_unnamed.l);
        }
        113 => {
            cyc = cyc.wrapping_add(12 as libc::c_int as libc::c_uint);
            outc(z, 0 as libc::c_int as uint8_t);
        }
        163 => {
            cyc = cyc.wrapping_add(16 as libc::c_int as libc::c_uint);
            outi(z);
        }
        179 => {
            cyc = cyc.wrapping_add(16 as libc::c_int as libc::c_uint);
            outi(z);
            if (*z).c2rust_unnamed_0.c2rust_unnamed.b as libc::c_int > 0 as libc::c_int {
                (*z).pc = ((*z).pc as libc::c_int - 2 as libc::c_int) as uint16_t;
                cyc = cyc.wrapping_add(5 as libc::c_int as libc::c_uint);
                (*z).mem_ptr = ((*z).pc as libc::c_int + 1 as libc::c_int) as uint16_t;
            }
        }
        171 => {
            cyc = cyc.wrapping_add(16 as libc::c_int as libc::c_uint);
            outd(z);
        }
        187 => {
            cyc = cyc.wrapping_add(16 as libc::c_int as libc::c_uint);
            outd(z);
            if (*z).c2rust_unnamed_0.c2rust_unnamed.b as libc::c_int > 0 as libc::c_int {
                (*z).pc = ((*z).pc as libc::c_int - 2 as libc::c_int) as uint16_t;
                cyc = cyc.wrapping_add(5 as libc::c_int as libc::c_uint);
                (*z).mem_ptr = ((*z).pc as libc::c_int + 1 as libc::c_int) as uint16_t;
            }
        }
        66 => {
            cyc = cyc.wrapping_add(15 as libc::c_int as libc::c_uint);
            sbchl(z, (*z).c2rust_unnamed_0.bc);
        }
        82 => {
            cyc = cyc.wrapping_add(15 as libc::c_int as libc::c_uint);
            sbchl(z, (*z).c2rust_unnamed_1.de);
        }
        98 => {
            cyc = cyc.wrapping_add(15 as libc::c_int as libc::c_uint);
            sbchl(z, (*z).c2rust_unnamed_2.hl);
        }
        114 => {
            cyc = cyc.wrapping_add(15 as libc::c_int as libc::c_uint);
            sbchl(z, (*z).sp);
        }
        74 => {
            cyc = cyc.wrapping_add(15 as libc::c_int as libc::c_uint);
            adchl(z, (*z).c2rust_unnamed_0.bc);
        }
        90 => {
            cyc = cyc.wrapping_add(15 as libc::c_int as libc::c_uint);
            adchl(z, (*z).c2rust_unnamed_1.de);
        }
        106 => {
            cyc = cyc.wrapping_add(15 as libc::c_int as libc::c_uint);
            adchl(z, (*z).c2rust_unnamed_2.hl);
        }
        122 => {
            cyc = cyc.wrapping_add(15 as libc::c_int as libc::c_uint);
            adchl(z, (*z).sp);
        }
        67 => {
            cyc = cyc.wrapping_add(20 as libc::c_int as libc::c_uint);
            let addr: uint16_t = nextw(z);
            ww(z, addr, (*z).c2rust_unnamed_0.bc);
            (*z).mem_ptr = (addr as libc::c_int + 1 as libc::c_int) as uint16_t;
        }
        83 => {
            cyc = cyc.wrapping_add(20 as libc::c_int as libc::c_uint);
            let addr_0: uint16_t = nextw(z);
            ww(z, addr_0, (*z).c2rust_unnamed_1.de);
            (*z).mem_ptr = (addr_0 as libc::c_int + 1 as libc::c_int) as uint16_t;
        }
        99 => {
            cyc = cyc.wrapping_add(20 as libc::c_int as libc::c_uint);
            let addr_1: uint16_t = nextw(z);
            ww(z, addr_1, (*z).c2rust_unnamed_2.hl);
            (*z).mem_ptr = (addr_1 as libc::c_int + 1 as libc::c_int) as uint16_t;
        }
        115 => {
            cyc = cyc.wrapping_add(20 as libc::c_int as libc::c_uint);
            let addr_2: uint16_t = nextw(z);
            ww(z, addr_2, (*z).sp);
            (*z).mem_ptr = (addr_2 as libc::c_int + 1 as libc::c_int) as uint16_t;
        }
        75 => {
            cyc = cyc.wrapping_add(20 as libc::c_int as libc::c_uint);
            let addr_3: uint16_t = nextw(z);
            (*z).c2rust_unnamed_0.bc = rw(z, addr_3);
            (*z).mem_ptr = (addr_3 as libc::c_int + 1 as libc::c_int) as uint16_t;
        }
        91 => {
            cyc = cyc.wrapping_add(20 as libc::c_int as libc::c_uint);
            let addr_4: uint16_t = nextw(z);
            (*z).c2rust_unnamed_1.de = rw(z, addr_4);
            (*z).mem_ptr = (addr_4 as libc::c_int + 1 as libc::c_int) as uint16_t;
        }
        107 => {
            cyc = cyc.wrapping_add(20 as libc::c_int as libc::c_uint);
            let addr_5: uint16_t = nextw(z);
            (*z).c2rust_unnamed_2.hl = rw(z, addr_5);
            (*z).mem_ptr = (addr_5 as libc::c_int + 1 as libc::c_int) as uint16_t;
        }
        123 => {
            cyc = cyc.wrapping_add(20 as libc::c_int as libc::c_uint);
            let addr_6: uint16_t = nextw(z);
            (*z).sp = rw(z, addr_6);
            (*z).mem_ptr = (addr_6 as libc::c_int + 1 as libc::c_int) as uint16_t;
        }
        68 | 84 | 100 | 116 | 76 | 92 | 108 | 124 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = subb(
                z,
                0 as libc::c_int as uint32_t,
                (*z).c2rust_unnamed.c2rust_unnamed.a as uint32_t,
                0 as libc::c_int != 0,
            );
        }
        70 | 102 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z).interrupt_mode = 0 as libc::c_int as uint8_t;
        }
        86 | 118 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z).interrupt_mode = 1 as libc::c_int as uint8_t;
        }
        94 | 126 => {
            cyc = cyc.wrapping_add(8 as libc::c_int as libc::c_uint);
            (*z).interrupt_mode = 2 as libc::c_int as uint8_t;
        }
        103 => {
            cyc = cyc.wrapping_add(18 as libc::c_int as libc::c_uint);
            let mut a: uint8_t = (*z).c2rust_unnamed.c2rust_unnamed.a;
            let mut val_0: uint8_t = rb(z, (*z).c2rust_unnamed_2.hl);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (a as libc::c_int & 0xf0 as libc::c_int
                | val_0 as libc::c_int & 0xf as libc::c_int) as uint8_t;
            wb(
                z,
                (*z).c2rust_unnamed_2.hl,
                (val_0 as libc::c_int >> 4 as libc::c_int
                    | (a as libc::c_int) << 4 as libc::c_int) as uint8_t,
            );
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .f = (f_szpxy[(*z).c2rust_unnamed.c2rust_unnamed.a as usize]
                as libc::c_int | flag_val(cf, flag_get(z, cf)) as libc::c_int
                | flag_val(nf, 0 as libc::c_int != 0) as libc::c_int
                | flag_val(hf, 0 as libc::c_int != 0) as libc::c_int) as uint8_t;
            (*z)
                .mem_ptr = ((*z).c2rust_unnamed_2.hl as libc::c_int + 1 as libc::c_int)
                as uint16_t;
        }
        111 => {
            cyc = cyc.wrapping_add(18 as libc::c_int as libc::c_uint);
            let mut a_0: uint8_t = (*z).c2rust_unnamed.c2rust_unnamed.a;
            let mut val_1: uint8_t = rb(z, (*z).c2rust_unnamed_2.hl);
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .a = (a_0 as libc::c_int & 0xf0 as libc::c_int
                | val_1 as libc::c_int >> 4 as libc::c_int) as uint8_t;
            wb(
                z,
                (*z).c2rust_unnamed_2.hl,
                ((val_1 as libc::c_int) << 4 as libc::c_int
                    | a_0 as libc::c_int & 0xf as libc::c_int) as uint8_t,
            );
            (*z)
                .c2rust_unnamed
                .c2rust_unnamed
                .f = (f_szpxy[(*z).c2rust_unnamed.c2rust_unnamed.a as usize]
                as libc::c_int | flag_val(cf, flag_get(z, cf)) as libc::c_int
                | flag_val(nf, 0 as libc::c_int != 0) as libc::c_int
                | flag_val(hf, 0 as libc::c_int != 0) as libc::c_int) as uint8_t;
            (*z)
                .mem_ptr = ((*z).c2rust_unnamed_2.hl as libc::c_int + 1 as libc::c_int)
                as uint16_t;
        }
        _ => {}
    }
    return cyc;
}
