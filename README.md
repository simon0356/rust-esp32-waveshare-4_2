# Simple example of Rust executing on e-Paper ESP32 Driver Board and waveshare 4.2" epaper

## Setup Build Environment

Follow https://github.com/esp-rs/rust-build installation  
Install esp-id : https://github.com/espressif/esp-idf  
Install Espressif LLVM : https://github.com/espressif/llvm-project  


    cd llvm-project
    mkdir build
    cd build
    cmake -G Ninja -DLLVM_ENABLE_PROJECTS='clang' -DCMAKE_BUILD_TYPE=Release ../llvm
    cmake --build .
    export PATH="$HOME/esp32/rust/src/llvm-project/build/bin:$PATH"


## Build and flash this project :

    cargo build
    espflash /dev/ttyUSB0 target/xtensa-esp32-espidf/debug/rust-esp32-waveshare-4_2

## Result

![Result](photo_result.jpg)


## Dependancy 
https://docs.rs/epd-waveshare/latest/epd_waveshare/  
https://github.com/esp-rs/esp-idf-hal  

## Thanks

Thanks [ivmarkov](https://github.com/ivmarkov) for the simplest example on how to build and flash rust on ESP32 and all dependancy crate maintainer 
