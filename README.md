# WasmEdge FFMPEG Plugin SDK
- `src/lib.rs`: contains all wrapper functions and structs for FFMPEG C-API
- `src/main.rs`: a simple app converts MP4 to TS
- `wasmedge_ffmpeg.wit`: WIT file

## Installation
### 1. Install FFMPEG
```bash
git clone https://github.com/FFmpeg/FFmpeg --depth 1 && cd FFmpeg
./configure --prefix=$HOME/.ffmpeg/ --enable-gpl --enable-nonfree --disable-static --enable-shared
sudo make install
```

### 2. Install LLVM & CMake
- MacOS
```bash
brew install cmake ninja llvm

export LLVM_DIR=$(brew --prefix)/opt/llvm/lib/cmake
export CC=$(brew --prefix)/opt/llvm/bin/clang
export CXX=$(brew --prefix)/opt/llvm/bin/clang++
```

- Ubuntu 20.04
```bash
sudo apt install -y \
   software-properties-common \
   cmake \
   libboost-all-dev

# And you will need to install llvm for the AOT runtime
sudo apt install -y \
   llvm-12-dev \
   liblld-12-dev

# WasmEdge supports both clang++ and g++ compilers.
# You can choose one of them to build this project.
# If you prefer GCC, then:
sudo apt install -y gcc g++
# Or if you prefer clang, then:
sudo apt install -y clang-12
```

### 2. WasmEdge FFMPEG Plugin
- Build `WasmEdge` with `FFMPEG` plugin enabled.
```bash
git clone https://github.com/blueyellowpink/wasmedge-ffmpeg && cd wasmedge-ffmpeg

cmake -Bbuild \
      -GNinja \
      -DWASMEDGE_BUILD_TESTS=OFF \
      -DWASMEDGE_PLUGIN_FFMPEG=ON \
      -DFFMPEG_INCLUDE_DIR=$HOME/.ffmpeg/include \
      -DFFMPEG_LIB_DIR=$HOME/.ffmpeg/lib \
      .

cmake --build build
```

- Install `WasmEdge`
```bash
sudo cmake --install build --prefix /usr/local
```

### 3. Install Witc
- https://github.com/second-state/witc

### 4. WasmEdge FFMPEG SDK
- Compile as `wasm32-wasi` target
```bash
git clone https://github.com/blueyellowpink/wasmedge-ffmpeg-sdk && cd wasmedge-ffmpeg-sdk
cargo build --target wasm32-wasi --release
```

## Run
- Convert `test_video.mp4` to `test_video.ts`
```bash
wasmedge target/wasm32-wasi/release/wasmedge-ffmpeg-sdk.wasm
```

- (Optional) Or
```bash
wasmedgec target/wasm32-wasi/release/wasmedge-ffmpeg-sdk.wasm wasmedge-ffmpeg-sdk.wasm
wasmedge wasmedge-ffmpeg-sdk.wasm
```
