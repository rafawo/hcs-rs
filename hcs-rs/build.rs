//! Build script that takes care of making sure HCS APIs can be linked
//! to their corresponding .lib files when the crate is built.
//!
//! This script relies on the environment variables `WIN10SDK_PATH` and `WIN10SDK_VERSION`.
//! `WIN10SDK_PATH` defaults to `c:\Program Files (x86)\Windows Kits\10` if not set.
//! `WIN10SDK_VERSION` defaults to `10.0.17763.0` if not set.

use std::env::var;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=WIN10SDK_PATH");

    let root_win10_sdk_path = match var("WIN10SDK_PATH") {
        Ok(path) => path,
        Err(_) => String::from("c:\\Program Files (x86)\\Windows Kits\\10"),
    };

    let win10_sdk_version = match var("WIN10SDK_VERSION") {
        Ok(path) => path,
        Err(_) => String::from("10.0.17763.0"),
    };

    let lib_names = vec![
        String::from("computecore"),
        String::from("computenetwork"),
        String::from("computestorage"),
        String::from("vmdevicehost"),
    ];

    let lib_root_path = format!(
        "{}\\Lib\\{}\\um\\x64",
        root_win10_sdk_path, win10_sdk_version
    );

    for lib_name in lib_names {
        println!("cargo:rustc-link-lib=dylib={}", lib_name);
    }

    println!("cargo:rustc-link-search={}", lib_root_path);
}
