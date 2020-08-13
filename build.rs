/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate cmake;
extern crate pkg_config;

use cmake::Config;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if !target.contains("eabi") &&
        !target.contains("android") &&
        pkg_config::Config::new().atleast_version("18.5.12").find("freetype2").is_ok() {
        return
    }

    let mut config = Config::new("freetype2");
    if let Ok(s) = env::var("FREETYPE_CMAKE_GENERATOR") {
        config.generator(s);
    }
    let cfg = if target.contains("android") {
        let ndk_root = env::var("ANDROID_NDK_ROOT").or(env::var("ANDROID_NDK_HOME")).expect("`$ANDROID_NDK_ROOT` or `$ANDROID_NDK_ROOT` is not set.");
        let config = config
            .define("CMAKE_TOOLCHAIN_FILE", format!("{}/build/cmake/android.toolchain.cmake", ndk_root));
        if target.starts_with("aarch64") {
            config.define("ANDROID_ABI", "arm64-v8a")
        } else if target.starts_with("armv7") {
            config.define("ANDROID_ABI", "armeabi-v7a")
        } else if target.starts_with("i686") {
            config.define("ANDROID_ABI", "x86")
        } else if target.starts_with("x86_64") {
            config.define("ANDROID_ABI", "x86_64")
        } else {
            config
        }
    } else {
        &mut config
    };
    let dst = cfg
        .define("WITH_BZip2", "OFF")
        .define("WITH_HarfBuzz", "OFF")
        .define("WITH_PNG", "OFF")
        .define("WITH_ZLIB", "OFF")
        .profile("Release")
        .build();
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=freetype");
    println!("cargo:outdir={}", out_dir);
}
