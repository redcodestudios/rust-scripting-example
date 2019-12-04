#include <lua.h>
#include <lauxlib.h>
#include <lualib.h> 


#include <stdio.h>


void call_lua() {
    lua_State *L;
    L = luaL_newstate();
    printf("loading lua script\n");
    luaL_openlibs(L);
    luaL_loadfile(L, "teste.lua");

    if (lua_pcall(L, 0, 0, 0))
        printf("falhou: %s\n", lua_tostring(L, -1));


    lua_close(L);

}
int diguilixo(int a, int b){
    return a + b;
}

static int luadiguilixo(lua_State *L){              /* Internal name of func */
	float a = lua_tonumber(L, -1);      /* Get the single number arg */
	float b = lua_tonumber(L, -2);      /* Get the single number arg */
	printf("Top of square(), nbr=%d\n",diguilixo(a,b));
	lua_pushnumber(L, a+b);           /* Push the return */
	return 1;                              /* One return value */
}

int luaopen_interpreter(lua_State *L){
	lua_register(
			L,               /* Lua state variable */
			"diguilixo",        /* func name as known in Lua */
			luadiguilixo          /* func name in this file */
			);  
	return 0;
}
// int main() {
// 	lua_State *L;
//     L = luaL_newstate();
//     printf("loading lua script\n");
//     luaL_openlibs(L);
//     luaL_loadfile(L, "lixo.lua");

//      if (lua_pcall(L, 0, 0, 0))
//         printf("falhou: %s\n", lua_tostring(L, -1));


// 	lua_close(L);

//     return 0;
// }