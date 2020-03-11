use std::os::raw::c_int;

#[no_mangle]
fn multii(numero:c_int) -> c_int{
    println!("Rust: Calculando {} ao cubo.\n", numero);
    return numero.pow(2);
}
