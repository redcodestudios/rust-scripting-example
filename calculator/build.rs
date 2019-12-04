extern crate cc;
fn main() {
    cc::Build::new()
        .flag("-I")
        .flag("/home/pablo/Downloads/lua-5.1/src/")
        .flag("-llua5.1")
        .file("lua_interpreter.c")
        .shared_flag(true)
        .compile("interpreter");
    
    // println!("{}",env!("OUT_DIR"));
        // .compile("lua_interpreter")
    
    // gcc -shared -Wl,-soname,libctest.so.1 -o libctest.so.1.0   *.o
    // cc::Build::new()
    //     .file("lua_interpreter.c")
    //     .flag("-I")
    //     .flag("/home/pablo/Downloads/lua-5.1/src/")
    //     .flag("-llua5.1")
    //     .shared_flag(true)
}