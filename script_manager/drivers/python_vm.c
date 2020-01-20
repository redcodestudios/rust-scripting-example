#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include <stdio.h>

void call_python(int state,  const char* script) {
    printf("starting python\n");
    wchar_t *program = Py_DecodeLocale("", NULL);
    if (program == NULL) {
        fprintf(stderr, "Fatal error: cannot decode arg[0]\n");
        exit(1);
    }
    Py_SetProgramName(program);
    Py_Initialize();
    PyRun_SimpleString("print('hello from python!')");

    if(Py_FinalizeEx < 0) {
        exit(120);
    }
    PyMem_RawFree(program);
}
