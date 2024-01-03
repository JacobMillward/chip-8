[![GitHub](https://img.shields.io/github/license/jacobmillward/chip-8?label=License)](https://github.com/JacobMillward/chip-8/blob/main/LICENSE)
[![Build](https://github.com/JacobMillward/chip-8/actions/workflows/build.yml/badge.svg)](https://github.com/JacobMillward/chip-8/actions/workflows/build.yml)
[![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/jacobmillward/chip-8?label=Release)](https://github.com/JacobMillward/chip-8/releases/latest)

# Chip-8 Emulator

This project is a Chip-8 emulator written in rust. Mostly written via reading [Cowgod's Chip8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM).

## Usage

### Example

```sh
$ chip8 Pong.ch8
```

```
USAGE:
    chip8 <INPUT_ROM>

ARGS:
    <INPUT_ROM>    ROM file to play

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

## Building

Building this project requires the rust toolchain, which can be installed via [`rustup`](https://rustup.rs/). It can then be built with [`cargo`](https://doc.rust-lang.org/cargo/).

```sh
$ cargo build
```

# License

This is licensed under the MIT license. See [LICENSE](./LICENSE) for more details.
