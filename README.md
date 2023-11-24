# Brief presentation

This project is my portfolio. I decided to use rust as the primary language because I like it. It's just awesome.

I am using the framework yew rs in addition with different libraries

# Requirements

Rust verion >= 1.64.0
```bash
# The following command will add the WebAssembly target to your development environment.
rustup target add wasm32-unknown-unknown
# note that this might take a while to install because it compiles everything from scratch
# Trunk also provides prebuilt binaries for a number of major package managers
# See https://trunkrs.dev/#install for further details
cargo install --locked trunk
```

# How to run on local

```bash
trunk serve
```

# Credits

Design was done by [Cathy Dolle](https://www.cathydolle.com/)