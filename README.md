# Aiken Playground

The Aiken playground is a fully client side rust web app using [leptos](https://github.com/leptos-rs/leptos).

<img src="https://raw.githubusercontent.com/aiken-lang/play/main/public/screenshot.png?token=GHSAT0AAAAAABWMRD6S6LBUC763EVAL55Q4ZCIID4Q" />

## Features

- [x] Check code
  - [x] Display warnings
  - [x] Display Errors
- [x] Run tests
- [x] Format code
- [x] Share
- [ ] Build validators (working but not surfaced in UI yet)

## Development

- `cargo install trunk`
- `cargo install --locked cargo-make`
- `rustup target add wasm32-unknown-unknown`
- `cargo make dev`

## Deployment

Cloudflare should only host the generated static site. The Rust/WASM build now runs on GitHub Actions via [`.github/workflows/continuous-deployment.yml`](/Users/ktorz/Documents/Projects/aiken-lang/play/.github/workflows/continuous-deployment.yml), which:

- installs Node.js and Rust on `ubuntu-latest`
- builds the site with [`scripts/build-dist.sh`](/Users/ktorz/Documents/Projects/aiken-lang/play/scripts/build-dist.sh)
- deploys the generated `dist/` directory to Cloudflare Pages with `wrangler pages deploy`

Required GitHub secrets:

- `CLOUDFLARE_API_TOKEN`
- `CLOUDFLARE_ACCOUNT_ID`
