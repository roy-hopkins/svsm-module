// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2022-2023 SUSE LLC
//
// Author: Roy Hopkins <rhopkins@suse.de>

#![no_std]
#![no_main]

use core::panic::PanicInfo;

use crate::syscall::Syscall;

mod syscall;

fn log(msg: &str) {
    Syscall::syscall(1, msg.as_bytes().as_ptr() as u64);
}

fn exit() {
    Syscall::syscall(0, 0);
}

#[no_mangle]
pub extern "C" fn module_main() -> isize {
    for _i in 0..3 {
        log("**** Hello from the loadable module!! ****");
    }
    exit();
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
