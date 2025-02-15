# Chip-8 Emulator in Rust

---

Written as an exercise in Rust, emulation and system level programming. 


### Dependencies


* [vcpkg](https://github.com/microsoft/vcpkg) Package

* [SDL2](https://github.com/Rust-SDL2/rust-sdl2) Graphics lib


### Build & Run

#### Windows

To enable logging include flag ```RUST_LOG=emu```
```
cargo install cargo-vcpkg
cargo vcpkg build
cargo build
cargo run
```

### Test

```
cargo test
```
### Docs

To open in browser run with ``--open`` flag.

```
cargo doc
```