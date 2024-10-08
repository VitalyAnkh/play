bt := '0'

export RUST_BACKTRACE := bt
export LLVM_INSTALL_DIR := "/home/vitalyr/sdk/lib/llvm"

log := "warn"

set shell := ["fish", "-c"]

export JUST_LOG := log

list_bin:
  #!/usr/bin/env bash
  echo "==== build hands on rust ===="
  cargo run
  
hands_on_rust:
  #!/usr/bin/env bash
  echo "==== build hands on rust ===="
  export HANDS_ON_RUST_SRC=$HOME/projects/dev/rust-projects/play/hands_on_rust
  cd $HANDS_ON_RUST_SRC
  cargo xwin run --bin hello_bterm --target x86_64-pc-windows-msvc
  cargo xwin run --bin hello_your_name --target x86_64-pc-windows-msvc
