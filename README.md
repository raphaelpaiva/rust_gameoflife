Rust Game of Life
=================

A Rust implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) using SDL2 bindings for Rust.

I used Windows for this project. So if you want to build it, you'll need to download and unpack these two files to your library folder.

1. https://www.libsdl.org/release/SDL2-devel-2.0.14-VC.zip
2. https://www.libsdl.org/projects/SDL_ttf/release/SDL2_ttf-devel-2.0.15-VC.zip

For rustup, its usually: `C:\Users\{Your Username}\.rustup\toolchains\{current toolchain}\lib\rustlib\{current toolchain}\lib`

Usage
-----
* `rust_gol.exe` - Creates a random board
* `rust_gol.exe Path\to\scenario.bmp` - Creates a game from a bitmap scenario.

Dependencies
------------
There are SDL and TTF .dll dependencies. These are available in the [releases](https://github.com/raphaelpaiva/rust_gameoflife/releases) section of this repository.

Due Credits
-----------
* [D-DIN font by Datto Inc.](https://www.1001fonts.com/d-din-font.html)