#!/bin/sh
set -eu
out=${1:?usage: skeleton.sh <output directory>}
mkdir -p "$out"
cp Cargo.toml Cargo.lock "$out/"
cp -r .cargo vendor "$out/"
for manifest in crates/*/Cargo.toml apps/*/Cargo.toml; do
  dir=$(dirname "$manifest")
  mkdir -p "$out/$dir"
  cp "$manifest" "$out/$dir/"
  find "$dir" -name '*.rs' ! -name build.rs | while read -r source; do
    mkdir -p "$out/$(dirname "$source")"
    echo 'fn main() {}' > "$out/$source"
  done
done
