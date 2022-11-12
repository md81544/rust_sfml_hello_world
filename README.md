# rust_sfml_hello_world

## Rationale
This is a bare-bones SFML "hello world" to show how to create an SFML application in Rust. Warning: there may well be glaring Rust errors here, I am using this as a learning project :)

## Pre-requisites
### Linux (including WSL)

Ensure you have `libcsfml-dev` and `libsfml-dev` installed.

If you are running this from WSL, and you see errors like the following,

```
Failed to create an OpenGL context for this window
X Error of failed request:  BadValue (integer parameter out of range for operation)
```

then try doing `export LIBGL_ALWAYS_INDIRECT=0` in your bash shell first.
### Mac

`brew install sfml` and `brew install csfml` (install homebrew if you haven't already)

then `export SFML_INCLUDE_DIR=/System/Volumes/Data/opt/homebrew/include` and `export SFML_LIBS_DIR=/System/Volumes/Data/opt/homebrew/lib`


## Building
Just the usual `cargo build`.

## Running
Just the usual `cargo run`.