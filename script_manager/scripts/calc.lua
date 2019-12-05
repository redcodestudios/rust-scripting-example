print("LUA: initialized calc.lua")
print("LUA: state is " .. state)

function mult(a)
	return a * 3
end

-- Function defined only in Rust, but shared via dynamic library to Lua.
rust_log("LUA: executing rust_log function")

state = mult(state)

print("LUA: state updated to " .. state)
