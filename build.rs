//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.
//!
//! The build script also sets the linker flags to tell it which link script to use.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

// fn print_env() {
//     let env_keys = ["TARGET", "OUT_DIR", "HOST"];
//     env::vars().for_each(|(key, val)| {
//         if key.starts_with("CARGO") {
//             println!("cargo:warning={}={}", key, val);
//         } else if env_keys.contains(&key.as_str()) {
//             println!("cargo:warning={}={}", key, val);
//         } else {
//             // println!("cargo:warning={}={}", key, val);
//         }
//     });
// }

fn main() {
    // print_env();

    let mut b = freertos_cargo_build::Builder::new();

    // Path to FreeRTOS kernel or set ENV "FREERTOS_SRC" instead
    b.freertos("/Users/cairo/embed-rs/FreeRTOS-Kernel");
    b.freertos_config("src");       // Location of `FreeRTOSConfig.h` 
    b.freertos_port("GCC/ARM_CM3".to_string()); // Port dir relativ to 'FreeRTOS-Kernel/portable' 
    // b.heap("heap_4.c".to_string());             // Set the heap_?.c allocator to use from 
                                    // 'FreeRTOS-Kernel/portable/MemMang' (Default: heap_4.c)       

    // b.get_cc().file("More.c");   // Optional additional C-Code to be compiled

    b.compile().unwrap_or_else(|e| { panic!("{}", e.to_string()) });

    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");

    // Specify linker arguments.

    // `--nmagic` is required if memory section addresses are not aligned to 0x10000,
    // for example the FLASH and RAM sections in your `memory.x`.
    // See https://github.com/rust-embedded/cortex-m-quickstart/pull/95
    println!("cargo:rustc-link-arg=--nmagic");

    // Set the linker script to the one provided by cortex-m-rt.
    println!("cargo:rustc-link-arg=-Tlink.x");


}

