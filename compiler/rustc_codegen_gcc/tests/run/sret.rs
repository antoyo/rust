// Compiler:
//
// Run-time:
//   status: 0
//   stdout: 7 8 9
//     3 2 1
//     4 5 6
//     10 11 12
//     20 21 22

#![feature(no_core)]
#![no_std]
#![no_core]
#![no_main]

extern crate mini_core;
use mini_core::*;

// 24 bytes: returned indirectly (sret) under both the Rust and the C ABI.
struct Big {
    a: u64,
    b: u64,
    c: u64,
}

// 12 bytes: returned indirectly under the Rust ABI only (the C ABI would
// return it in registers), so this exercises the forced memory return.
struct Mid {
    a: u32,
    b: u32,
    c: u32,
}

struct Pair {
    first: u64,
    second: Big,
}

#[inline(never)]
fn make_big(a: u64, b: u64, c: u64) -> Big {
    Big { a, b, c }
}

#[inline(never)]
extern "C" fn make_big_c(a: u64, b: u64, c: u64) -> Big {
    Big { a, b, c }
}

#[inline(never)]
fn make_mid(a: u32, b: u32, c: u32) -> Mid {
    Mid { a, b, c }
}

// The shape that used to fail in getopts: a 24-byte tuple.
#[inline(never)]
fn make_tuple(a: usize, b: u64, c: u64) -> (usize, u64, u64) {
    (a, b, c)
}

#[no_mangle]
extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    unsafe {
        let big = make_big(7, 8, 9);
        libc::printf(b"%ld %ld %ld\n\0" as *const u8 as *const i8, big.a, big.b, big.c);

        let big_c = make_big_c(3, 2, 1);
        libc::printf(b"%ld %ld %ld\n\0" as *const u8 as *const i8, big_c.a, big_c.b, big_c.c);

        let mid = make_mid(4, 5, 6);
        libc::printf(b"%d %d %d\n\0" as *const u8 as *const i8, mid.a, mid.b, mid.c);

        let tuple = make_tuple(10, 11, 12);
        libc::printf(b"%ld %ld %ld\n\0" as *const u8 as *const i8, tuple.0, tuple.1, tuple.2);

        // Store an indirect return value into a field of a local.
        let mut pair = Pair { first: 0, second: make_big(1, 2, 3) };
        pair.second = make_big(20, 21, 22);
        libc::printf(
            b"%ld %ld %ld\n\0" as *const u8 as *const i8,
            pair.second.a,
            pair.second.b,
            pair.second.c,
        );
    }
    0
}
