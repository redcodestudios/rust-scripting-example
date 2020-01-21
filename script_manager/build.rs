
extern crate cc;
use std::env;
use std::process::Command;

fn main() {
    if cfg!(target_os = "linux") {
        Command::new("make")
            .current_dir("lua/src")
            .status()
            .expect("failed to execute process");
 
        Command::new("make")
            .current_dir("python/src/cpython")
            .status()
            .expect("failed to execute process");
        
        cc::Build::new()
            .flag("-I")
            .flag("lua/src/")
            .flag("-llua")
            .object("../target/debug/libscripting_api.so")
            .file("drivers/lua_vm.c")
            .compile("lua_vm");
   
        cc::Build::new()
            .include("python/src/cpython/Include")
            .include("python/src/cpython")
            .flag("-lpython3.9")
            .object("../target/debug/libscripting_api.so")
            .file("drivers/python_vm.c")
            .compile("python_vm");

        println!("cargo:rustc-flags=-l lua -L lua/src");
        println!("cargo:rustc-flags=-l python3.9 -L python/src/cpython");
    }
}
