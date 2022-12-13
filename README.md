# ESP-IDF based or `std` Blinky

This is a very simple Rust sample for ESP32 family boards based on ESP-IDF.

The LED will blink or L チカ on ESP32 (Xtensa) and ESP32-C3 (RISC-V) series development boards.

### `std` or `no_std`

The Rust has two environments. One is `std` environment. In most cases, the program runs on OS and can get its resource or functions based on its manner. The Other is a bare metal or `no_std` environment. In this case, the program controls all functions through registers on the chip.

The latest Rust supports both `std` and `non_std` environments for ESP32 family. The `std` environment is based on its native development kit or ESP-IDF. Thus, a large storage space, a few gigabytes, is required for each project.

**This sample program runs on `std` environment.**

## Setting

| Board    | LED pin |
| -------- | ------- |
| ESP32    | GPIO2   |
| ESP32-C3 | GPIO8   |

These boards are my original design, and please change the pin number depending on your configurations.

## Compile

Confirm building target in .cargo/config.toml before compiling

```
$ cargo build # For ESP32-C3

# For ESP32
$ cargo +esp build
or
$ rustup override set esp
$ cargo build
```

## Flash

The port connected to the ESP32 or ESP32-C3 is found automatically, and you will be asked.

```
$ cargo espflash --monitor
```

## Get into the trouble

The `esp-idf-hal` or ESP32 hardware abstraction layer significantly changes between `0.38.1` and `0.39.3`.

To set the LED pin in the `0.38.1`

```
let pin = peripherals.pins.gpio2;
let mut led = pin.into_output()?;
```

Since `0.39.3`, `PinDriver` is required.

```
let pin = peripherals.pins.gpio2;
let mut led = PinDriver::output(pin)?;
```
