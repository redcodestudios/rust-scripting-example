build:
	cbindgen --config cbindgen.toml --crate calculator --output include/calculator.h

remove-so:
	rm libinterpreter.so*
generate-so:
	gcc -shared -fPIC -Wl,-soname,libinterpreter.so.1 -o libinterpreter.so.1.0   *.o
	ln -sf libinterpreter.so.1.0 libinterpreter.so.1
	ln -sf libinterpreter.so.1.0 libinterpreter.so
