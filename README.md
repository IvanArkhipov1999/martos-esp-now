# Rust example for xtensa esp32 architecture

Presented here is a straightforward 'Hello world' Rust example utilizing Martos.

Within the setup function, the phrase 'Setup hello world!' is printed once.
Additionally, within the loop function, the phrase 'Loop hello world!' along with the counter value is printed fifty times.

## How to install dependencies

For comprehensive guidance on installing the necessary dependencies for developing applications targeting the Xtensa ESP32 architecture,
please refer to [the official website](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html).
Below is an illustrative example demonstrating the installation of building toolchains on a Linux (Ubuntu/Debian):
```
cargo install espup
espup install
```

## How to build the example

For a thorough guide on developing projects for the Xtensa ESP32 architecture across various operating systems,
we recommend consulting [the official website](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html#3-set-up-the-environment-variables).
Below, you will find an illustrative example showcasing the building process on a Linux system (Ubuntu/Debian):
```
. $HOME/export-esp.sh
cargo build
```

## How to run the example
For detailed instructions on running projects for the Xtensa ESP32 architecture across various operating systems,
we recommend consulting [the official website](https://docs.esp-rs.org/book/tooling/espflash.html).
Below, you will find an illustrative example showcasing the running on a Linux system (Ubuntu/Debian):
```
cargo run
```
