#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn rewind(__stream: *mut FILE);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn z80_init(z: *mut z80);
    fn z80_step(z: *mut z80) -> libc::c_uint;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
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
    pub c2rust_unnamed: C2RustUnnamed_13,
    pub c2rust_unnamed_0: C2RustUnnamed_11,
    pub c2rust_unnamed_1: C2RustUnnamed_9,
    pub c2rust_unnamed_2: C2RustUnnamed_7,
    pub c2rust_unnamed_3: C2RustUnnamed_5,
    pub c2rust_unnamed_4: C2RustUnnamed_3,
    pub c2rust_unnamed_5: C2RustUnnamed_1,
    pub c2rust_unnamed_6: C2RustUnnamed,
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
pub union C2RustUnnamed {
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub h_l_: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub l_: uint8_t,
    pub h_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub d_e_: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub e_: uint8_t,
    pub d_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub b_c_: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub c_: uint8_t,
    pub b_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub c2rust_unnamed: C2RustUnnamed_6,
    pub a_f_: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub f_: uint8_t,
    pub a_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub c2rust_unnamed: C2RustUnnamed_8,
    pub hl: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub l: uint8_t,
    pub h: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub c2rust_unnamed: C2RustUnnamed_10,
    pub de: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub e: uint8_t,
    pub d: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub c2rust_unnamed: C2RustUnnamed_12,
    pub bc: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub c: uint8_t,
    pub b: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub c2rust_unnamed: C2RustUnnamed_14,
    pub af: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub f: uint8_t,
    pub a: uint8_t,
}
static mut memory: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
static mut test_finished: bool = 0 as libc::c_int != 0;
static mut cyc: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
unsafe extern "C" fn rb(mut userdata: *mut libc::c_void, mut addr: uint16_t) -> uint8_t {
    !userdata.is_null();
    return *memory.offset(addr as isize);
}
unsafe extern "C" fn wb(
    mut userdata: *mut libc::c_void,
    mut addr: uint16_t,
    mut val: uint8_t,
) {
    !userdata.is_null();
    *memory.offset(addr as isize) = val;
}
unsafe extern "C" fn load_file(
    mut filename: *const libc::c_char,
    mut addr: uint16_t,
) -> libc::c_int {
    let mut f: *mut FILE = fopen(filename, b"rb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        fprintf(
            stderr,
            b"error: can't open file '%s'.\n\0" as *const u8 as *const libc::c_char,
            filename,
        );
        return 1 as libc::c_int;
    }
    fseek(f, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
    let mut file_size: size_t = ftell(f) as size_t;
    rewind(f);
    if file_size.wrapping_add(addr as libc::c_ulong)
        >= 0x10000 as libc::c_int as libc::c_ulong
    {
        fprintf(
            stderr,
            b"error: file %s can't fit in memory.\n\0" as *const u8
                as *const libc::c_char,
            filename,
        );
        return 1 as libc::c_int;
    }
    let mut result: size_t = fread(
        &mut *memory.offset(addr as isize) as *mut uint8_t as *mut libc::c_void,
        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        file_size,
        f,
    );
    if result != file_size {
        fprintf(
            stderr,
            b"error: while reading file '%s'\n\0" as *const u8 as *const libc::c_char,
            filename,
        );
        return 1 as libc::c_int;
    }
    fclose(f);
    return 0 as libc::c_int;
}
unsafe extern "C" fn in_0(z: *mut z80, mut port: uint16_t) -> uint8_t {
    port != 0;
    let mut operation: uint8_t = (*z).c2rust_unnamed_0.c2rust_unnamed.c;
    if operation as libc::c_int == 2 as libc::c_int {
        printf(
            b"%c\0" as *const u8 as *const libc::c_char,
            (*z).c2rust_unnamed_1.c2rust_unnamed.e as libc::c_int,
        );
    } else if operation as libc::c_int == 9 as libc::c_int {
        let mut addr: uint16_t = (((*z).c2rust_unnamed_1.c2rust_unnamed.d as libc::c_int)
            << 8 as libc::c_int | (*z).c2rust_unnamed_1.c2rust_unnamed.e as libc::c_int)
            as uint16_t;
        loop {
            let fresh0 = addr;
            addr = addr.wrapping_add(1);
            printf(
                b"%c\0" as *const u8 as *const libc::c_char,
                rb(z as *mut libc::c_void, fresh0) as libc::c_int,
            );
            if !(rb(z as *mut libc::c_void, addr) as libc::c_int != '$' as i32) {
                break;
            }
        }
    }
    return 0xff as libc::c_int as uint8_t;
}
unsafe extern "C" fn out(z: *mut z80, mut port: uint16_t, mut val: uint8_t) {
    !z.is_null() || port as libc::c_int != 0 || val as libc::c_int != 0;
    test_finished = 1 as libc::c_int != 0;
}
unsafe extern "C" fn run_test(
    z: *mut z80,
    mut filename: *const libc::c_char,
    mut cyc_expected: libc::c_ulong,
) -> libc::c_int {
    z80_init(z);
    (*z)
        .read_byte = Some(
        rb as unsafe extern "C" fn(*mut libc::c_void, uint16_t) -> uint8_t,
    );
    (*z)
        .write_byte = Some(
        wb as unsafe extern "C" fn(*mut libc::c_void, uint16_t, uint8_t) -> (),
    );
    (*z).port_in = Some(in_0 as unsafe extern "C" fn(*mut z80, uint16_t) -> uint8_t);
    (*z).port_out = Some(out as unsafe extern "C" fn(*mut z80, uint16_t, uint8_t) -> ());
    memset(
        memory as *mut libc::c_void,
        0 as libc::c_int,
        0x10000 as libc::c_int as libc::c_ulong,
    );
    cyc = 0 as libc::c_int as libc::c_ulong;
    if load_file(filename, 0x100 as libc::c_int as uint16_t) != 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    printf(b"*** TEST: %s\n\0" as *const u8 as *const libc::c_char, filename);
    (*z).pc = 0x100 as libc::c_int as uint16_t;
    *memory.offset(0 as libc::c_int as isize) = 0xd3 as libc::c_int as uint8_t;
    *memory.offset(0x1 as libc::c_int as isize) = 0 as libc::c_int as uint8_t;
    *memory.offset(0x5 as libc::c_int as isize) = 0xdb as libc::c_int as uint8_t;
    *memory.offset(0x6 as libc::c_int as isize) = 0 as libc::c_int as uint8_t;
    *memory.offset(0x7 as libc::c_int as isize) = 0xc9 as libc::c_int as uint8_t;
    let mut nb_instructions: libc::c_long = 0 as libc::c_int as libc::c_long;
    test_finished = 0 as libc::c_int != 0;
    while !test_finished {
        nb_instructions += 1 as libc::c_int as libc::c_long;
        cyc = cyc.wrapping_add(z80_step(z) as libc::c_ulong);
    }
    let mut diff: libc::c_longlong = cyc_expected.wrapping_sub(cyc) as libc::c_longlong;
    printf(
        b"\n*** %lu instructions executed on %lu cycles (expected=%lu, diff=%lld)\n\n\0"
            as *const u8 as *const libc::c_char,
        nb_instructions,
        cyc,
        cyc_expected,
        diff,
    );
    return (cyc_expected != cyc) as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    memory = malloc(0x10000 as libc::c_int as libc::c_ulong) as *mut uint8_t;
    if memory.is_null() {
        return 1 as libc::c_int;
    }
    let mut cpu: z80 = z80 {
        read_byte: None,
        write_byte: None,
        port_in: None,
        port_out: None,
        userdata: 0 as *mut libc::c_void,
        pc: 0,
        sp: 0,
        ix: 0,
        iy: 0,
        mem_ptr: 0,
        c2rust_unnamed: C2RustUnnamed_13 {
            c2rust_unnamed: C2RustUnnamed_14 { f: 0, a: 0 },
        },
        c2rust_unnamed_0: C2RustUnnamed_11 {
            c2rust_unnamed: C2RustUnnamed_12 { c: 0, b: 0 },
        },
        c2rust_unnamed_1: C2RustUnnamed_9 {
            c2rust_unnamed: C2RustUnnamed_10 { e: 0, d: 0 },
        },
        c2rust_unnamed_2: C2RustUnnamed_7 {
            c2rust_unnamed: C2RustUnnamed_8 { l: 0, h: 0 },
        },
        c2rust_unnamed_3: C2RustUnnamed_5 {
            c2rust_unnamed: C2RustUnnamed_6 { f_: 0, a_: 0 },
        },
        c2rust_unnamed_4: C2RustUnnamed_3 {
            c2rust_unnamed: C2RustUnnamed_4 { c_: 0, b_: 0 },
        },
        c2rust_unnamed_5: C2RustUnnamed_1 {
            c2rust_unnamed: C2RustUnnamed_2 { e_: 0, d_: 0 },
        },
        c2rust_unnamed_6: C2RustUnnamed {
            c2rust_unnamed: C2RustUnnamed_0 { l_: 0, h_: 0 },
        },
        i: 0,
        r: 0,
        iff_delay: 0,
        interrupt_mode: 0,
        irq_data: 0,
        irq_pending: 0,
        nmi_pending: 0,
        iff1_iff2_halted: [0; 1],
        c2rust_padding: [0; 6],
    };
    let mut r: libc::c_int = 0 as libc::c_int;
    r
        += run_test(
            &mut cpu,
            b"roms/prelim.com\0" as *const u8 as *const libc::c_char,
            8721 as libc::c_ulong,
        );
    r
        += run_test(
            &mut cpu,
            b"roms/zexdoc.cim\0" as *const u8 as *const libc::c_char,
            46734978649 as libc::c_ulong,
        );
    r
        += run_test(
            &mut cpu,
            b"roms/zexall.cim\0" as *const u8 as *const libc::c_char,
            46734978649 as libc::c_ulong,
        );
    free(memory as *mut libc::c_void);
    return (r != 0 as libc::c_int) as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
