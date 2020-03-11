#include <stdio.h>

// Function declared in rust, received as -lscripting_api during cc::build of rust_vm.c
extern void rust_log(char *s);

// Function declared in rust_scritps, received as -lrust_scripts during cc::build of rust_vm.c
extern int multii(int);

static int wrapper_log(char* msg) {
   rust_log(msg);
   return 1;
}

void call_rust(int* state,const char* script) {
    rust_log("C: DEU BOM PORRA");
   
    printf("C: state is %d\n", (int) *state);
    *state = multii(*state);
    printf("C: new state is %d\n", (int) *state);
}