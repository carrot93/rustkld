# RustKLD
FreeBSD Rust Driver

Based on the works of Johannes Lundberg and David Young. 
Planning on expanding this with different drivers as I learn more.
Will also add documentation on how to use this to build your own driver in rust.

## Setup
1. Clone the repo
2. Install rust via Rustup: https://rustup.rs/
3. Install llvm19
4. do ```rustup component add rust-src```
5. do ```cargo install cargo-make``` 

## Running
1. Build and compile by: ```cargo make build-kmod```
2. Load the kld: ```kldload ./hello.ko``` 
3. Check its existance: ```kldstat | grep "hello"```
4. Unload the kld: ```kldunload hello```
