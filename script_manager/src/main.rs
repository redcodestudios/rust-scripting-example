use std::{thread, time};
use std::ffi::CString;
use std::os::raw::{c_char, c_int};

extern{
    fn call_lua(state: *mut c_int, script: *const c_char);
    fn call_rust(state: *mut c_int, script: *const c_char);
}

fn exec_script(state: *mut i32, script_path: &str) {
    println!("RUST: loading {} script", script_path);
    let mut iter = script_path.split(".");
    iter.next();
    let extension = iter.next().unwrap();
    println!("RUST:extension : {}", extension);
    unsafe {
        if extension == String::from("rs"){
            call_rust(state, CString::new(script_path).expect("CString::new failed").as_ptr());
        }else if extension == String::from("lua"){
            call_lua(state, CString::new(script_path).expect("CString::new failed").as_ptr());
        }
        println!("RUST: new_ptr value {}", *state);
    }
}

fn output(state: i32) {
    println!("RUST: the actual state is {}", state);
}

fn main() {
    println!("RUST: starting engine");
    
    let mut state: i32 = 2;

    loop {
        print!("\n");
        exec_script(&mut state, "script_manager/scripts/calc.lua");
        exec_script(&mut state, "rust_scripts/src/lib.rs");
        output(state);
        thread::sleep(time::Duration::from_millis(1000));
    }
}
