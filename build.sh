brew install llvm zlib bzip2

export PATH=/opt/homebrew/opt/llvm/bin:$PATH

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

source $HOME/.cargo/env

rustup install 1.80.0
rustup default 1.80.0

rustup target add wasm32-unknown-unknown

cargo install trunk

npx tailwindcss -i ./styles.css -o ./output.css --minify

# Unpack stdlib
rm -rf stdlib && mkdir -p stdlib
curl -L \
  -H "Accept: application/vnd.github+json" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  https://api.github.com/repos/aiken-lang/stdlib/tarball/v2.1.0 \
  -o stdlib.tar
tar -xvf stdlib.tar --strip-components 1 -C stdlib
rm stdlib.tar

# Unpack fuzz
rm -rf fuzz && mkdir -p fuzz
curl -L \
  -H "Accept: application/vnd.github+json" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  https://api.github.com/repos/aiken-lang/fuzz/tarball/v2.1.0 \
  -o fuzz.tar
tar -xvf fuzz.tar --strip-components 1 -C fuzz
rm fuzz.tar

trunk build --release
