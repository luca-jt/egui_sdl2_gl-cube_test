# Description
This is a simple example for linking static C/C++ libraries with Rust.\
This project also uses the [egui_sdl2_gl](https://github.com/ArjunNair/egui_sdl2_gl) crate to combine egui GUI elements, SDL2 sound and events, and OpenGL rendering. It is just for testing purposes.

# Getting started
### Download Rust
- Download rustup from https://www.rust-lang.org/tools/install
- Test your install via `rustc --version` and `cargo --version` and `rustup --version` to check if PATH env variables are set correctly
- Run `rustup update` in your terminal
### Download sdl2-dev and sdl2-mixer-dev
Linux (Ubuntu):
- Run `sudo apt-get install libsdl2-dev`
- Run `sudo apt-get install libsdl2-mixer-dev`

\
Windows (assuming your Rust distro is `stable-x86_64-pc-windows-msvc`):
- Go to https://github.com/libsdl-org/SDL/releases/latest
- Download and unpack the latest `SDL2-devel-X.XX.X-VC.zip`
- Copy all the `*.lib` files from `SDL2-devel-2.30.2-VC/SDL2-2.30.2/lib/x64/` to your `.rustup/toolchains/stable-x86_64-pc-windows-msvc/lib/rustlib/x86_64-pc-windows-msvc/lib/` folder in your user directory
- Go to https://github.com/libsdl-org/SDL_mixer/releases/latest
- Repeat the steps from before with `SDL2_mixer-devel-X.X.X-VC.zip`
