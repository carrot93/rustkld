# RustKLD
FreeBSD Rust Drivers

Based on the works of Johannes Lundberg and David Young. 
Planning on expanding this with different drivers as I learn more.
Will also add documentation on the design of this repo and how to use it to build your own driver in rust.

## Setup
1. Clone the repo
2. Install rust via Rustup: https://rustup.rs/
3. Install llvm19
4. do ```rustup component add rust-src```
5. do ```cargo install cargo-make``` 

## Running
### Hello World Driver
1. cd into drivers/hello , generate the bindings & build the kld by: ```cargo make build-kmod``` 
2. Load the kld: ```kldload ./hello.ko```
3. Check its existance: ```kldstat | grep "hello"```
4. Unload the kld: ```kldunload hello```

### Character Driver
1. cd into drivers/hello , generate the bindings & build the kld by: ```cargo make build-kmod``` 
2. Load the kld: ```kldload ./charDev.ko``` 
3. Check its existance: ```kldstat | grep "charDev"```
4. Check the registration with devfs: ```ls /dev/echo```
5. Write to dev path: ```echo "Hello :D" > /dev/echo```
6. Read echo'ed message: ```cat /dev/echo```
7. Unload the kld: ```kldunload charDev```
