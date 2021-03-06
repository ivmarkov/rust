# A fork of the Rust compiler for ESP, with STD support

This fork enables projects to be built for the ESP32 and ESP8266 using [espressif's llvm fork](https://github.com/espressif/llvm-project).
Moreover, the fork contains a port of [Rust](https://github.com/rust-lang/rust)'s STD lib on top of the ESP-IDF libraries.

The repo is essentially a copy of the work of [MabezDev](https://github.com/MabezDev/rust-xtensa) which enables support in Rust for the Xtensa/ESP8266/ESP32 targets.
<br>... plus a set of extensions of the std library so that it can compile and run on top of ESP-IDF.

## Background

A few words on the approach taken for bringing STD support to the ESP platform:
* Even though the ESPs are considered "bare metal", the ESP-IDF framework of Espressif is actually a relatively complete Posix layer
* If you squint a little, you can even pretend that an ESP running the ESP-IDF framework - from the point of view of the app on top - looks like a real Unix. That's because ESP-IDF has a libc (newlib), BSD sockets layer (lwIP), pthread support (running on top of FreeRTOS) and quite a few other unix/posix APIs. It also uses GCC, elf and does have a port of LLVM (as per above)
* So besides the process and env APIs which are obviously not available on the ESP, everything else looks like regular unix/posix C API layer

Therefore, the STD support for ESP is implemented inside Rust's standard std::sys::unix modules and more or less boils down to stubbing out functionality which is not available in ESP-IDF. The whole ESP STD patch-set is having currently the miniscule size of ~500-1000 LOCs.

## Disclaimer

STD for ESP **does** require the ESP-IDF toolkit and **does** call into ESP-IDF. This is contrary to [other efforts (esp-rs) related to running Rust on ESP](https://github.com/esp-rs), which are trying to avoid any dependencies on the vendor-provided software stack.

* ... but what you get as a result is the full power of ESP-IDF and the ability to interoperate with other libraries built on top of ESP-IDF!
* ... and type-safe wrappers for various ESP-IDF services: [WiFi](https://github.com/ivmarkov/esp-idf-svc/blob/master/src/wifi.rs), [Network](https://github.com/ivmarkov/esp-idf-svc/blob/master/src/netif.rs), [HTTP Server](https://github.com/ivmarkov/esp-idf-svc/blob/master/src/httpd.rs), [Ping](https://github.com/ivmarkov/esp-idf-svc/blob/master/src/ping.rs), [Logging](https://github.com/ivmarkov/esp-idf-svc/blob/master/src/log.rs), [Flash Storage (soon)](https://github.com/ivmarkov/esp-idf-svc/blob/master/src/nvs_storage.rs). These come as a set of [abstractions](https://github.com/ivmarkov/embedded-svc) similar in spirit to [embedded-hal](https://github.com/rust-embedded/embedded-hal) and are designed to be portable to other boards (**RPI0 anyone**?). Of course, all of those with [implementations for the  ESP32/ESP-IDF](https://github.com/ivmarkov/esp-idf-svc/).

## Forum

Rust on ESP seems to be discussed here: https://matrix.to/#/#esp-rs:matrix.org

## ["Hello, World" demo app](https://github.com/ivmarkov/rust-esp32-std-hello)

[Here](https://github.com/ivmarkov/rust-esp32-std-hello)

## Installation

* Espressif was kind enough to offer [pre-built binaries of this repo](https://github.com/espressif/rust-esp32-example/blob/main/docs/rust-on-xtensa.md) (including the custom Clang LLVM necessary for building the "Hello, World" demo app from above).
* Download & install the binaries for your operating system by following the instructions in their repo.

**NOTE**: The Linux binaries will **NOT** work on older operating systems, like Ubuntu 18.04 LTS. For these, you DO need to build this Rust fork and the Espressif's LLVM fork yourself as described below.

## Building

Install [Rustup](https://rustup.rs/).

Build using these steps:
```sh
$ cd <some directory where you'll keep the compiler binaries and its sources; you'll need to keep the whole GIT repo, because xargo/cargo need those when building your ESP32 crates>
$ git clone https://github.com/ivmarkov/rust
$ cd rust
$ git checkout stable
$ ./configure --experimental-targets=Xtensa
$ ./x.py build --stage 2
```

* **NOTE 1**: Building might take **close to an hour**!
* **NOTE 2**: Make sure you are using the `stable` GIT branch of the fork (the default), which is based on MabezDev Stable, which in turn is based on Rust 1.50.0):
* **NOTE 3**: Do NOT rename the directory ('rust') where you've cloned the Rust fork. It must be 'rust' or you might have strange issues later on when using it. You can however place it anywhere in your file tree

### Fix toolchain vendor/ directory, so that building STD with Cargo does work

(Assuming you are still in the rust/ directory):

```sh
$ mkdir vendor
$ cd vendor
$ ln -s ../library/rustc-std-workspace-alloc/ rustc-std-workspace-alloc
$ ln -s ../library/rustc-std-workspace-core/ rustc-std-workspace-core
$ ln -s ../library/rustc-std-workspace-std/ rustc-std-workspace-std
```

Make Rustup aware of the newly built compiler:

```sh
$ rustup toolchain link xtensa ~/<...>/rust/build/x86_64-unknown-linux-gnu/stage2
```

Switch to the new compiler in Rustup:

```sh
$ rustup default xtensa
```

Check the compiler:
```sh
$ rustc --print target-list
```

At the end of the printed list of targets you should see:
```
...
xtensa-esp32-none-elf
xtensa-esp8266-none-elf
xtensa-none-elf
```

## Building LLVM clang

You'll need the custom LLVM clang based on the Espressif LLVM fork for the ["Hello, World" demo app](https://github.com/ivmarkov/rust-esp32-std-hello). Build as follows:
```sh
$ git clone https://github.com/espressif/llvm-project
$ cd llvm-project
$ mkdir build
$ cd build
$ cmake -G Ninja -DLLVM_ENABLE_PROJECTS='clang' -DCMAKE_BUILD_TYPE=Release ../llvm
$ cmake --build .
$ export PATH=`pwd`/bin:$PATH
```

Check that you have the custom clang on your path:
```sh
$ which clang
$ which llvm-config
```

The above should output locations pointing at your custom-built clang toolchain.

* **NOTE 1**: Building LLVM clang might take **even longer** time than building the Rustc toolchain!
* **NOTE 2**: You might want to make the PATH modification step from above permanent. Please make sure that the custom clang compiler is the first on your PATH so that it takes precedence over any clang compiler you might have installed using your distro / OS. Otherwise, you might be greeted with a strange build error when building the ["Hello, World" demo app](https://github.com/ivmarkov/rust-esp32-std-hello):
```
  thread 'main' panicked at 'libclang error; possible causes include:
  - Invalid flag syntax
  - Unrecognized flags
  - Invalid flag arguments
  - File I/O errors
  - Host vs. target architecture mismatch
  If you encounter an error missing from this list, please file an issue or a PR!', ~/.cargo/registry/src/github.com-1ecc6299db9ec823/bindgen-0.57.0/src/ir/context.rs:531:15
```


## Updating this fork

TBD: https://github.com/ivmarkov/rust-xtensa-patches
