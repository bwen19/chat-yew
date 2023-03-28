# Yew-Natter

A blazing fast chat application built with Yew.rs.

## Features

- Realtime chat application
- Blazing fast, completely built on Rust.
- Create individual and group chats and delete them
- View and add messages
- Dark mode

## Getting Started

You will need a couple of tools to compile, build, package and debug your Yew application. When getting started, we recommend using Trunk. Trunk is a WASM web application bundler for Rust.

### Installing Rust

To install Rust, follow the [official instructions](https://www.rust-lang.org/tools/install).

### Install WebAssembly target

The compilation target for browser-based WebAssembly is called `wasm32-unknown-unknown`. The following command will add the WebAssembly target to your development environment.

```bash
rustup target add wasm32-unknown-unknown
```

### Install Trunk

```bash
# note that this might take a while to install because it compiles everything from scratch
# Trunk also provides prebuilt binaries for a number of major package managers
# See https://trunkrs.dev/#install for further details
cargo install --locked trunk
```

### View your web application

Run the following command to build and serve the application locally.

```bash
trunk serve --open
```

## License

[Apache-2.0](/LICENSE)
