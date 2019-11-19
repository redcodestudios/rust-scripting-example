
#include <lua.h>
#include <lauxlib.h>
#include <lualib.h> 


#include <stdio.h>


void call_lua() {
    lua_State *L;
    L = luaL_newstate();
    printf("loading lua script\n");
    luaL_openlibs(L);
    luaL_loadfile(L, "lixo.lua");

    if (lua_pcall(L, 0, 0, 0))
        printf("falhou: %s\n", lua_tostring(L, -1));


    lua_close(L);

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