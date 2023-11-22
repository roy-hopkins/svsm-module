// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2022-2023 SUSE LLC
//
// Author: Roy Hopkins <rhopkins@suse.de>

fn main() {
    println!("cargo:rustc-link-arg-bin=svsm-module=-nostdlib");
    println!("cargo:rustc-link-arg-bin=svsm-module=-Wl,--build-id=none");
    println!("cargo:rustc-link-arg-bin=svsm-module=-Wl,-Tmodule.lds");
}
