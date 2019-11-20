
extern crate cc;

fn main() {
    cc::Build::new()
        .flag("-I")
        .flag("~/Downloads/lua-5.3.5/src/")
        .flag("-llua")
        .flag("-lshared")
        .flag("-L./shared/target/debug")
        .file("c/lua_vm.c")
        .compile("lua_vm");
}
