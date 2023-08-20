# WasmEdge FFMPEG plugin SDK
- `src/lib.rs`: contains all wrapper functions and structs for FFMPEG C-API
- `src/main.rs`: a simple app converts MP4 to TS
- `wasmedge_ffmpeg.wit`: WIT file

## Installation

### WasmEdge FFMPEG Plugin
- Build `WasmEdge` with `FFMPEG` plugin enabled.
```bash
export LLVM_DIR=$(brew --prefix)/opt/llvm/lib/cmake
export CC=$(brew --prefix)/opt/llvm/bin/clang
export CXX=$(brew --prefix)/opt/llvm/bin/clang++

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

### WasmEdge FFMPEG SDK
- Compile as `wasm32-wasi` target
```bash
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
