#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include <stdio.h>

static PyModuleDef module_def = {
    PyModuleDef_HEAD_INIT, "engine", NULL, -1,
    NULL, NULL, NULL, NULL, NULL
};

static PyObject* PyInit_engine(void){
    PyObject* module = PyModule_Create(&module_def);
    PyState_AddModule(module, &module_def);
    return module;
}

void call_python(int* state,  const char* script) {
    printf("starting python\n");
    wchar_t *program = Py_DecodeLocale("", NULL);
    if (program == NULL) {
        fprintf(stderr, "Fatal error: cannot decode arg[0]\n");
        exit(1);
    }
    PyImport_AppendInittab("engine", &PyInit_engine);
    Py_SetProgramName(program);
   
    // the python vm should be initialized before this function
    Py_Initialize();
    
    PyObject* module = PyState_FindModule(&module_def);
    if (module == NULL){
        printf("fuck\n");
    }else{
        PyModule_AddObject(module, "state",
                Py_BuildValue("i", (int) *state)
                );
    }
     
    FILE *script_f = fopen(script, "r");
    PyRun_SimpleFile(script_f, script);

    if(Py_FinalizeEx < 0) {
        exit(120);
    }
    PyMem_RawFree(program);
}
