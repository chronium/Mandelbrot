# Mandelbrot
A Mandelbrot fractal viewer written in Rust.

# Compiling

#### Dependencies
* This viewer uses the [rust_minifb](https://github.com/emoon/rust_minifb) library to display graphics to the screen.

You run `cargo build` and start it manually, or simply run `cargo run` and it will start automatically in debug mode.

If you want faster speed, you must use the `--release` flag.

**Example:** `cargo run --release` will build in release mode and them run the program.
