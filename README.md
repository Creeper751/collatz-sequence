# Collatz Sequence CLI

## Description 📃
Hello and welcome to the project! 👋

In this CLI, I have implemented a version of the collatz sequence in Rust, to be accessible via the command line.

## Downloading ⬇️
To download the binary, download it from the releases page, then see how to use below.

## Building 🔨
To build the project directly, first ensure you have Rust installed, clone this repo, and build it using the following command: `git clone https://github.com/Creeper751/collatz-sequence.git && cd collatz-sequence && cargo build --release`. The binary will be in the `target/release` folder.

## How to use 💻
To run, execute the binary with the `-n` or `--number` flag, followed by the number you want to calculate the collatz sequence for. For example, to calculate the collatz sequence for 10, run the following command: `./collatz-sequence -n 10`.

## Final Thoughts 💭
This is one of my first projects in Rust, and I hope you enjoy it! If you want to contribute, feel free to open a pull request. I used the [clap](https://github.com/clap-rs/clap) crate in my code, so go give them a star! This is also published under GPLv3, so feel free to fork and modify it.
