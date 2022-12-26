This code will flash the LEDs in the circle on an STM32F3 Discovery board, using the [Embassy](https://embassy.dev) Rust libraries.

To use, you may need to add the target. I'm using Rustup, so this was just `rustup target add thumbv7em-none-eabihf`. You also need [probe-run](https://github.com/knurling-rs/probe-run) available on your path. This may be provided by your distribution, or installable via `cargo install probe-run`.

After that, `cargo run` should compile the code and, assuming you have the board connected via USB to the ST-Link port, flash it and run it with debug messages visible in your console.
