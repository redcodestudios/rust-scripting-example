
extern crate cc;
use std::env;

fn main() {
    match pkg_config::find_library("lua5.3") {
        Ok(_) => return,
        Err(..) => {}
    };

    let mut build = cc::Build::new();

    if env::var("CARGO_CFG_TARGET_OS") == Ok("linux".to_string()) {
        // Enable `io.popen` support
        build.define("LUA_USE_LINUX", None);
    }

    build
        .file("lua/src/lapi.c")
        .file("lua/src/lcode.c")
        .file("lua/src/lctype.c")
        .file("lua/src/ldebug.c")
        .file("lua/src/ldo.c")
        .file("lua/src/ldump.c")
        .file("lua/src/lfunc.c")
        .file("lua/src/lgc.c")
        .file("lua/src/llex.c")
        .file("lua/src/lmem.c")
        .file("lua/src/lobject.c")
        .file("lua/src/lopcodes.c")
        .file("lua/src/lparser.c")
        .file("lua/src/lstate.c")
        .file("lua/src/lstring.c")
        .file("lua/src/ltable.c")
        .file("lua/src/ltm.c")
        .file("lua/src/lundump.c")
        .file("lua/src/lvm.c")
        .file("lua/src/lzio.c")
        .file("lua/src/lauxlib.c")
        .file("lua/src/lbaselib.c")
        .file("lua/src/lbitlib.c")
        .file("lua/src/lcorolib.c")
        .file("lua/src/ldblib.c")
        .file("lua/src/liolib.c")
        .file("lua/src/lmathlib.c")
        .file("lua/src/loslib.c")
        .file("lua/src/lstrlib.c")
        .file("lua/src/ltablib.c")
        .file("lua/src/loadlib.c")
        .file("lua/src/lutf8lib.c")
        .file("lua/src/linit.c")
        .define("LUA_COMPAT_ALL", None)
        .include("lua/src")
        .compile("liblua.a");
    
    cc::Build::new()
        .flag("-I")
        .flag("lua/src/")
        .flag("-llua")
        .object("../target/debug/libscripting_api.so")
        .file("drivers/lua_vm.c")
        .compile("lua_vm");
   
    //cc::Build::new()
        //.include("python/src/cpython/Include")
        //.include("python/src/cpython")
        //.file("python/src/cpython/Python/asdl.c")
        //.compile("libpython3.8.a");

    cc::Build::new()
        .include("python/src/cpython/Include")
        .include("python/src/cpython")
        .flag("-lpython3.9")
        .object("../target/debug/libscripting_api.so")
        .file("drivers/python_vm.c")
        .compile("python_vm");

}
