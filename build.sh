curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -- -y

source $HOME/.cargo/env

rustup target add wasm32-unknown-unknown

cargo install --locked trunk

cargo install --locked cargo-make

cargo make
