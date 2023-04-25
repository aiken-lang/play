# Aiken Playground

The Aiken playground is a fully client side rust web app using [leptos](https://github.com/leptos-rs/leptos).

<img src="https://raw.githubusercontent.com/aiken-lang/play/main/public/screenshot.png?token=GHSAT0AAAAAABWMRD6S6LBUC763EVAL55Q4ZCIID4Q" />

## Features

- [x] Check code
  - [x] Display warnings
  - [x] Display Errors  
- [x] Run tests
- [ ] Format code
- [ ] Build validators (working but not surfaced in UI yet)
- [ ] Share

## Development

* `cargo install trunk`
* `cargo install --locked cargo-make`
* `rustup target add wasm32-unknown-unknown`
* `cargo make dev`
