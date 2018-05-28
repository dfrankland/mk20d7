#! /bin/bash

if ! command -v svd2rust 2>/dev/null; then
  echo "You may need to install svd2rust: `cargo install svd2rust`";
fi

if ! command -v form 2>/dev/null; then
  echo "You may need to install form: `cargo install form`";
fi

if ! cargo list | grep -q fmt; then
  echo "You may need to install rustfmt: `rustup component add rustfmt-preview`";
fi

svd2rust -i ./MK20D7.svd && \
rm -rf ./src && \
form -i ./lib.rs -o ./src/ && \
rm ./lib.rs && \
cargo fmt
