print("LUA: calc from lua")
print("LUA: state is " .. state)

function mult(a)
	return a * 3
end

state = mult(state)
