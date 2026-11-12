#!/usr/bin/env bash
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
export WPRS_VERSION=${WPRS_VERSION:-$(sed -n 's/^version = "\(.*\)"/\1/p' "$ROOT/Cargo.toml" | head -1)-local}
CARGO_HOME=${CARGO_HOME:-$HOME/.cargo}
RUSTUP_HOME=${RUSTUP_HOME:-$HOME/.rustup}
unset CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_RUSTFLAGS
export RUSTFLAGS="--remap-path-prefix=$CARGO_HOME=/cargo --remap-path-prefix=$RUSTUP_HOME=/rustup \
  --remap-path-prefix=$HOME=/build --remap-path-prefix=$ROOT=/src"
export CFLAGS="-ffile-prefix-map=$CARGO_HOME=/cargo -ffile-prefix-map=$HOME=/build -ffile-prefix-map=$ROOT=/src"
export CXXFLAGS=$CFLAGS
only=()
for bin in "$@"; do only+=(--bin "$bin"); done
env -u SOURCE_DATE_EPOCH cargo build --release --locked --timings -p wprsd -p wprs-ctl -p wprs-app -p wprs-web \
  "${only[@]}" --manifest-path "$ROOT/Cargo.toml"
