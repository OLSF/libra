#!/bin/bash

# targeting ububtu
apt update
apt install -y git vim zip unzip jq build-essential cmake clang llvm libgmp-dev secure-delete pkg-config libssl-dev lld
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
PATH="$HOME/.cargo/bin:$PATH" cargo install toml-cli
PATH="$HOME/.cargo/bin:$PATH" cargo install sccache

echo "Rust is installed now. Great!\
\
To get started you need Cargo's bin directory ($HOME/.cargo/bin) in your PATH\
environment variable. Next time you log in this will be done\
automatically.\n\
To configure your current shell, run: \n\
source $HOME/.cargo/env\
"

echo "Manually add 'export RUSTC_WRAPPER=sccache' to your bash profile ~/.bashrc"