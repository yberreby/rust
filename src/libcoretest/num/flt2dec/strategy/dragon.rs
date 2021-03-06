// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::prelude::v1::*;
use std::{i16, f64};
use super::super::*;
use core::num::flt2dec::*;
use core::num::flt2dec::bignum::Big32x40 as Big;
use core::num::flt2dec::strategy::dragon::*;

#[test]
fn test_mul_pow10() {
    let mut prevpow10 = Big::from_small(1);
    for i in 1..340 {
        let mut curpow10 = Big::from_small(1);
        mul_pow10(&mut curpow10, i);
        assert_eq!(curpow10, *prevpow10.clone().mul_small(10));
        prevpow10 = curpow10;
    }
}

#[test]
fn shortest_sanity_test() {
    f64_shortest_sanity_test(format_shortest);
    f32_shortest_sanity_test(format_shortest);
    more_shortest_sanity_test(format_shortest);
}

#[test]
fn exact_sanity_test() {
    // This test ends up running what I can only assume is some corner-ish case
    // of the `exp2` library function, defined in whatever C runtime we're
    // using. In VS 2013 this function apparently had a bug as this test fails
    // when linked, but with VS 2015 the bug appears fixed as the test runs just
    // fine.
    //
    // The bug seems to be a difference in return value of `exp2(-1057)`, where
    // in VS 2013 it returns a double with the bit pattern 0x2 and in VS 2015 it
    // returns 0x20000.
    //
    // For now just ignore this test entirely on MSVC as it's tested elsewhere
    // anyway and we're not super interested in testing each platform's exp2
    // implementation.
    if !cfg!(target_env = "msvc") {
        f64_exact_sanity_test(format_exact);
    }
    f32_exact_sanity_test(format_exact);
}

#[bench]
fn bench_small_shortest(b: &mut Bencher) {
    let decoded = decode_finite(3.141592f64);
    let mut buf = [0; MAX_SIG_DIGITS];
    b.iter(|| format_shortest(&decoded, &mut buf));
}

#[bench]
fn bench_big_shortest(b: &mut Bencher) {
    let decoded = decode_finite(f64::MAX);
    let mut buf = [0; MAX_SIG_DIGITS];
    b.iter(|| format_shortest(&decoded, &mut buf));
}

#[bench]
fn bench_small_exact_3(b: &mut Bencher) {
    let decoded = decode_finite(3.141592f64);
    let mut buf = [0; 3];
    b.iter(|| format_exact(&decoded, &mut buf, i16::MIN));
}

#[bench]
fn bench_big_exact_3(b: &mut Bencher) {
    let decoded = decode_finite(f64::MAX);
    let mut buf = [0; 3];
    b.iter(|| format_exact(&decoded, &mut buf, i16::MIN));
}

#[bench]
fn bench_small_exact_12(b: &mut Bencher) {
    let decoded = decode_finite(3.141592f64);
    let mut buf = [0; 12];
    b.iter(|| format_exact(&decoded, &mut buf, i16::MIN));
}

#[bench]
fn bench_big_exact_12(b: &mut Bencher) {
    let decoded = decode_finite(f64::MAX);
    let mut buf = [0; 12];
    b.iter(|| format_exact(&decoded, &mut buf, i16::MIN));
}

#[bench]
fn bench_small_exact_inf(b: &mut Bencher) {
    let decoded = decode_finite(3.141592f64);
    let mut buf = [0; 1024];
    b.iter(|| format_exact(&decoded, &mut buf, i16::MIN));
}

#[bench]
fn bench_big_exact_inf(b: &mut Bencher) {
    let decoded = decode_finite(f64::MAX);
    let mut buf = [0; 1024];
    b.iter(|| format_exact(&decoded, &mut buf, i16::MIN));
}

#[test]
fn test_to_shortest_str() {
    to_shortest_str_test(format_shortest);
}

#[test]
fn test_to_shortest_exp_str() {
    to_shortest_exp_str_test(format_shortest);
}

#[test]
fn test_to_exact_exp_str() {
    to_exact_exp_str_test(format_exact);
}

#[test]
fn test_to_exact_fixed_str() {
    to_exact_fixed_str_test(format_exact);
}

