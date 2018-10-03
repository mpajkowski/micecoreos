# Mice Core OS
State-of-the-art, general purpose operating system written in Rust.\
Just following "Writing an OS in Rust" by Philipp Oppermann (right now).

# Build and test
## Needed components (assuming Arch Linux)
- Rust nightly
```
git clone https://github.com/mpajkowski/micecoreos
cd micecoreos
rustup override set nightly
cargo install cargo-xbuild
cargo install bootimage --version "^0.5.0"
bootimage build
bootimage run
```
