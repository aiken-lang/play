curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

source $HOME/.cargo/env

rustup target add wasm32-unknown-unknown

cargo install --locked trunk

npx tailwindcss -i styles.css -o output.css --minify

trunk build --release

