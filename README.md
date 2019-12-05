# Rust-Scripting
Repository with boilerplate for experimenting with scripting in Rust.

# Before running
We are using Lua language as scripting language, and for that we build the lua source code before interpreting it, so after cloning the repo you should update our git submodule too.
```bash
$ git submodule init
$ git submodule update
```
Now you will have Lua lang cloned in version 5.3.5 inside our `script_manager/lua/src` folder.

# Running the example

To run and build the experiment you should use Cargo:
```bash
$ cargo build
$ cargo run
```
