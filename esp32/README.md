# Rust example for xtensa esp32 architecture

Presented here is a straightforward Rust example utilizing Martos with lvp protocol.

A broadcast signal is periodically sent, and is also informed about the receipt of a broadcast signal from other microcontrollers. 

The mean of value on several microcontollers is calculating. [Here](https://github.com/IvanArkhipov1999/martos-esp-now/blob/main/esp32/src/main.rs#L20) you can change value of exact agent.

## How to install dependencies

For comprehensive guidance on installing the necessary dependencies for developing applications targeting the Xtensa ESP32 architecture,
please refer to [the official website](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html).
Below is an illustrative example demonstrating the installation of building toolchains on a Linux (Ubuntu/Debian):
```
apt-get -qq update
apt-get install -y -q build-essential curl
curl https://sh.rustup.rs -sSf | sh -s -- -y
cargo install espup
espup install
```

## How to build the example

For a thorough guide on developing projects for the Xtensa ESP32 architecture across various operating systems,
we recommend consulting [the official website](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html#3-set-up-the-environment-variables).
Below, you will find an illustrative example showcasing the building process on a Linux system (Ubuntu/Debian):
```
. $HOME/export-esp.sh
cargo build --release
```

## How to run the example
For detailed instructions on running projects for the Xtensa ESP32 architecture across various operating systems,
we recommend consulting [the official website](https://docs.esp-rs.org/book/tooling/espflash.html).
Below, you will find an illustrative example showcasing the running on a Linux system (Ubuntu/Debian):
```
cargo run --release
```
