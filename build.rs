extern crate fs_extra;
use fs_extra::dir::{copy, CopyOptions};
use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    let binding = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let target_dir = binding.parent().unwrap().parent().unwrap().parent().unwrap();//.join("static");
    // fs::create_dir_all(&target_dir).unwrap();
    let mut options = CopyOptions::new();
    options.overwrite = true;
    copy("static", &target_dir, &options).unwrap();
    fs::copy("SDL2.lib", target_dir.join("SDL2.lib")).unwrap();
    fs::copy("SDL2.dll", target_dir.join("SDL2.dll")).unwrap();
}