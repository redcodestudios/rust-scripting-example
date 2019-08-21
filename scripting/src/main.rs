use rlua::{Lua, Result};
use std::fs;
use std::io::Read;


fn main() -> Result<()> {
    let lua_file = "/home/shammyz/Documents/repos/stainless-experiments/scripting/hello.lua";
    let mut lua_code: String = String::new();

    match fs::File::open(lua_file) {
        Ok(mut file) => {
            lua_code = String::new();
            file.read_to_string(&mut lua_code).unwrap();
        },

        Err(error) => {
            print!("Error opening the file {}: {}", lua_file, error);
        }
    }
    
    let lua_state = Lua::new();

    lua_state.context(|lua_ctx| {
        let globals = lua_ctx.globals();
        let script_log = lua_ctx.create_function(|_, msg: String | {
            print!("SCRIPT LOGS: {}", msg);
            Ok(())
        })?;
        globals.set("log", script_log);

        lua_ctx.load(&lua_code).exec();

        Ok(())
    })?;
    Ok(())
}
