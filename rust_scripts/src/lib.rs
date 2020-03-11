use std::os::raw::c_int;

#[no_mangle]
fn sum_three(number:c_int) -> c_int{
    println!("Rust: calculating {} + 3.\n", number);
    return number + 3;
}
