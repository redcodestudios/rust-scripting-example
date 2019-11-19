extern crate cc;

fn main() {
    println!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    cc::Build::new()
        .flag("-I")
        .flag("/home/pablo/Downloads/lua-5.1/src/")
        .flag("-llua5.1")
        .file("pablo.c")
        .compile("pablo");
}