#include <stdio.h>
#include <lualib.h>
#include <lauxlib.h>
#include <lua.h>

extern void rust_log(char *s);

//void pwd() {
//    char cwd[1024];
//    getcwd(cwd, sizeof(cwd));
//    printf("Current working dir: %s\n", cwd);
//}

static int wrapper_log(lua_State *L) {
   char* m = lua_tostring(L, -1);
   rust_log(m);
   return 1;
}

void call_lua(int* state, const char* script) {
  //  pwd();
    //rust_log("DEU BOM PORRA");
    lua_State *L;
    L = luaL_newstate();
    printf("C: loading lua script %s\n", script);
    luaL_openlibs(L);
    luaL_loadfile(L, script);
   
    lua_pushcfunction(L, wrapper_log);
    lua_setglobal(L, "rust_log");
    
    printf("C: state is %d\n", (int) *state);
    lua_pushinteger(L, (int) *state); // pass state to lua script
    lua_setglobal(L, "state");

    if (lua_pcall(L, 0, 0, 0))
        printf("C: falhou: %s\n", lua_tostring(L, -1));

    lua_getglobal(L, "state");

    *state = lua_tointeger(L, -1); // assign pointer to the lua return
    printf("C: new state is %d\n", (int) *state);
    lua_close(L);
}
