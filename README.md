# led-strip-stuff


<img src="https://media4.giphy.com/media/L73UffeZZ3nQcEOBvL/giphy.gif">

currently using a forked version of ws2812-spi from smart-leds to make my strip work

configured for STM32F401CEUx, modify `.cargo/config` and/or `memory.x` for other variants





### Running via ST-Link

Using the ST-link with probe-run to flash and run with cargo run.
Install deps
```
$ rustup target install thumbv7em-none-eabihf
$ cargo install cargo-binutils
$ rustup component add llvm-tools-preview

$ apt install libftdi
$ cargo install cargo-flash
$ cargo install probe-run
```

### Run with probe-run
probe-run is already set as the default cargo runner - just run `cargo run`.
if your user is not set up to write to the /dev you may need something like `sudo -E env PATH=$PATH cargo run --release` 