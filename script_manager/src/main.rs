use std::{thread, time, fs};
use std::ffi::CString;
use std::os::raw::{c_char, c_int};

//#[link(name = "python", kind = "static")]
extern{
    fn call_lua(state: *mut c_int, script: *const c_char);
    fn call_python(state: *mut c_int, script: *const c_char);
}

fn exec_script(state: *mut i32, script_path: &str) {
    println!("RUST: loading {} script", script_path);
    unsafe {
        let scripts_entries = fs::read_dir(script_path).unwrap();
        for script_entry in scripts_entries {
            let script_path = String::from(script_entry.unwrap().path().to_str().unwrap());
            
            if script_path.ends_with(".lua") {
                call_lua(state, CString::new(script_path).expect("CString::new failed").as_ptr());
            } else {
                call_python(state, CString::new(script_path).expect("CString::new failed").as_ptr());
            }
        }
        //let raw_ptr = &mut state as *mut i32;
        //call_lua(state, CString::new(script_path).expect("CString::new failed").as_ptr());
        println!("RUST: new_ptr value {}", *state);
        //call_python(state, CString::new(script_path).expect("CString::new failed").as_ptr());
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
        exec_script(&mut state, "script_manager/scripts");
        output(state);
        thread::sleep(time::Duration::from_millis(1000)); 
    }
}
