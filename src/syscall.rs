// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2022-2023 SUSE LLC
//
// Author: Roy Hopkins <rhopkins@suse.de>

use core::arch::asm;

pub struct Syscall {}

impl Syscall {
    pub fn syscall(index: u32, param1: u64) -> u64 {
        unsafe {
            let ret: u64;
            asm!("syscall",
            in("rdi") index,
            in("rsi") param1,
            lateout("rax") ret, 
            options(att_syntax));
            ret
        }
    }
}
