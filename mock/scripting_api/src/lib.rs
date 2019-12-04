use std::os::raw::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn c_to_rust(c_str: *const c_char) -> String {
    unsafe{
        CStr::from_ptr(c_str).to_string_lossy().into_owned()
    }
}

#[no_mangle]
pub extern "C" fn rust_log(message: *const c_char) {
    println!(" --- RUST_LOG_: {} ---", c_to_rust(message));
}
