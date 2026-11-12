#!/usr/bin/env bash
set -euo pipefail
IN=${1:?usage: VERSION=<semver> $0 <bundles-dir>}
TAG=v${VERSION:?}
GH_REPO=${GH_REPO:?}
OUT=$(mktemp -d)
trap 'rm -rf "$OUT"' EXIT
gh release view "$TAG" >/dev/null || { echo "$TAG is not published, so the bundles are not attached"; exit 0; }
mkdir "$OUT/files"
gh release download "$TAG" --pattern SHA256SUMS --dir "$OUT/files"
bundles=(wallpaper-rs.flatpak wallpaper-rs-aarch64.flatpak)
for bundle in "${bundles[@]}"; do
  test -s "$IN/$bundle"
  cp "$IN/$bundle" "$OUT/files/$bundle"
done
sed -i '/[ *]wallpaper-rs\(-aarch64\)\?\.flatpak$/d' "$OUT/files/SHA256SUMS"
(cd "$OUT/files" && sha256sum -- "${bundles[@]}" >> SHA256SUMS && sha256sum -c --ignore-missing SHA256SUMS)
upload=("$OUT/files/SHA256SUMS" "${bundles[@]/#/$OUT/files/}")
if [ "${SIGN:-}" = true ]; then
  "$("$(dirname "$0")"/tools.sh cosign "$OUT/tools")" sign-blob --yes --bundle "$OUT/files/SHA256SUMS.sigstore.json" "$OUT/files/SHA256SUMS"
  upload+=("$OUT/files/SHA256SUMS.sigstore.json")
fi
gh release upload "$TAG" "${upload[@]}" --clobber
