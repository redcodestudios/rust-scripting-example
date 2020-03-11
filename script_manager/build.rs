
extern crate cc;
use std::env;
use std::process::Command;

fn main() {
    if cfg!(target_os = "linux") {
        
        Command::new("sh")
        .arg("configure")
        .current_dir("python/src/cpython")
        .status()
        .expect("failed to execute process");
        
        Command::new("make")
        .arg("install")
        .current_dir("python/src/cpython")
        .status()
        .expect("failed to execute process");
        
        let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        cc::Build::new()
        .include("python/src/cpython/Include")
        .include("python/src/cpython")
        .flag("-lpython3.9")
        .flag(format!("-Wl, -rpath={}/python/src/cpython", crate_dir).as_str())
        .object("../target/debug/libscripting_api.so")
        .file("drivers/python_vm.c")
        .compile("python_vm");
        
        Command::new("make")
        .current_dir("lua/src")
        .status()
        .expect("failed to execute process");
        
        cc::Build::new()
        .flag("-I")
        .flag("lua/src/")
        .flag("-llua")
        .flag("-lscripting_api")
        .flag(format!("-Wl, -rpath={}/lua/src", crate_dir).as_str())
        .flag(format!("-L{}/../target/debug",crate_dir).as_str())
        .file("drivers/lua_vm.c")
        .compile("lua_vm");
        
        cc::Build::new()
        .flag(format!("-L{}/scripts",crate_dir).as_str())
        .flag(format!("-L{}../target/debug",crate_dir).as_str())
        .flag("-lrust_scripts")
        .flag("-lscripting_api")
        .file("drivers/rust_vm.c")
        .compile("rust_vm");
        
        println!("cargo:rustc-flags=-L {}/script_manager/lua/src -l lua", crate_dir);
        println!("cargo:rustc-flags=-L {}/target/debug -l scripting_api", crate_dir);
        println!("cargo:rustc-flags=-L {}/script_manager/python/src/cpython -l python3.9", crate_dir);
        println!("cargo:rustc-flags= -l rust_scripts");
    }
}
