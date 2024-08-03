apt update
apt -y install libz3-4 clang libclang-dev libclang1 liblldb-dev

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

source $HOME/.cargo/env

rustup install 1.80.0
rustup default 1.80.0

rustup target add wasm32-unknown-unknown

cargo install --locked trunk

npx tailwindcss -i ./styles.css -o ./output.css --minify

mkdir stdlib

curl -L \
  -H "Accept: application/vnd.github+json" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  https://api.github.com/repos/aiken-lang/stdlib/tarball/main \
  -o stdlib.tar

tar -xvf stdlib.tar --strip-components 1 -C stdlib

rm stdlib.tar

trunk build --release
