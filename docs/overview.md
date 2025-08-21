# RustKLD Overview
This document serves to assist in learning about this environment to utilize, contribute, and extend as see fit.
Please note that this documentation assumes you know the basics of FreeBSD device drivers. Some resources that helped me for this project are:
* [FreeBSD Device Drivers by Joseph Kong](https://www.amazon.com/FreeBSD-Device-Drivers-Guide-Intrepid/dp/1593272049)
* [Writing FreeBSD Device Drivers](https://docs.freebsd.org/en/books/arch-handbook/driverbasics/)

## Structure
This project is split into two main directories: **kernel/** and **drivers/**

**Note:** In each crate you will see a rust-toolchain.toml file that defines our version as **nightly-2025-05-18**. This is because when compiling with newer versions, the compiler-builtins crate assumes f16-f128 support and we get a link_elf_obj: symbol \_\_gnu_h2f_ieee undefined error when trying to kldload the .ko file.

### kernel/
The kernel/ directory houses our generated bindings, wrappers, and other structures that aid us in safe FreeBSD driver creation. Is is the heart of this project.
See [architecture.md](architecture.md) for more information

### drivers/
The drivers/ directory unsurprisingly contains the code for the drivers that I have created for this repo. So far, I have created two drivers: a **Hello World! driver** and a **Echo driver**. Each driver folder  contains build scripts to compile the rust code and create the desired KLD.

#### Hello World! Driver (drivers/hello) 
This driver simple prints a message when loading and unloading
```bash
ace@yuan:~/RustKLD/drivers/hello $ sudo kldload ./hello.ko
Greetings from hello.rs!
ace@yuan:~/RustKLD/drivers/hello $ sudo kldunload hello
Goodbye from hello.rs!
```

#### Echo Driver (drivers/char)
This driver transfers data to and from userspace
```bash
ace@yuan:~/RustKLD/drivers/char $ sudo kldload ./char_dev.ko
[module_events.rs] Echo device loaded
ace@yuan:~/RustKLD/drivers/char $ echo "Hello :D" > /dev/echo
[char_device.rs] character device opened
[char_device.rs] 9 bytes read into buffer
[char_device.rs] character device closed
ace@yuan:~/RustKLD/drivers/char $ cat /dev/echo
[char_device.rs] character device opened
[char_device.rs] 9 bytes written from buffer
Hello :D
[char_device.rs] 0 bytes written from buffer
[char_device.rs] character device closed
ace@yuan:~/RustKLD/drivers/char $ sudo kldunload char_dev
[module_events.rs] Quiesce!
[module_events.rs] Echo device unloaded
```

See [drivers.md](drivers.md) for more information
