use std::{thread, time};
use std::ffi::CString;
use std::os::raw::{c_char, c_int};

extern{
    fn call_lua(state: *mut c_int, script: *const c_char);
}

fn exec_script(state: *mut i32, script_path: &str) {
    println!("RUST: loading {} script", script_path);
    unsafe {
        //let raw_ptr = &mut state as *mut i32;
        call_lua(state, CString::new(script_path).expect("CString::new failed").as_ptr());
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
        exec_script(&mut state, "src/script/calc.lua");
        output(state);
        thread::sleep(time::Duration::from_millis(1000)); 
    }
}
