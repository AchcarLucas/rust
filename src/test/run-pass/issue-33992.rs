// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-windows
// ignore-macos

#![feature(linkage)]

#[linkage = "available_externally"]
pub static TEST2: bool = true;

#[linkage = "common"]
pub static mut TEST3: u32 = 0u32;

#[linkage = "extern_weak"]
pub static TEST4: bool = true;

#[linkage = "external"]
pub static TEST5: bool = true;

#[linkage = "internal"]
pub static TEST6: bool = true;

#[linkage = "linkonce"]
pub static TEST7: bool = true;

#[linkage = "linkonce_odr"]
pub static TEST8: bool = true;

#[linkage = "private"]
pub static TEST9: bool = true;

#[linkage = "weak"]
pub static TEST10: bool = true;

#[linkage = "weak_odr"]
pub static TEST11: bool = true;

fn main() {}