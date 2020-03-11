
extern crate cc;
use std::env;
use std::process::Command;
use std::io::{self, Write};

fn main() {
    if cfg!(target_os = "linux") {
        Command::new("make")
        .current_dir("lua/src")
        .status()
        .expect("failed to execute process");
        
        let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        cc::Build::new()
        .flag("-I")
        .flag("lua/src/")
        .flag("-llua")
        .flag("-lscripting_api")
        .flag("-L/home/pablo/repositorioGit/stainless-experiments/target/debug")
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
        println!("cargo:rustc-flags= -l rust_scripts");
    }
}
