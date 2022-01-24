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


## Build this project :

    cargo build
    espflash /dev/ttyUSB0 target/xtensa-esp32-espidf/debug/rust-esp32-waveshare-4_2

