# rust_sfml_hello_world

## Rationale
This is a bare-bones SFML "hello world" to show how to create an SFML application in Rust.

## Pre-requisites
Ensure you have `libcsfml-dev` and `libsfml-dev` installed.

If you are running this from WSL2, and you see errors like the following,

```
Failed to create an OpenGL context for this window
X Error of failed request:  BadValue (integer parameter out of range for operation)
```

then try doing `export LIBGL_ALWAYS_INDIRECT=0` in your bash shell first.

## Building
Just the usual `cargo build`.