#!/usr/bin/env bash
set -euo pipefail
declare WORK desktop
for output in HEADLESS-1 HEADLESS-2; do
  expect "s['outputs']['$output']['released']" True
  expect "s['outputs']['$output']['blank']" True
  check "$output" "0.5,0.5,$desktop" "0.9,0.9,$desktop"
done
if grep -q "layer surface created" "$WORK/wprsd.log"; then fail "a fresh start took the screens over"; fi
if grep -q "Layer surface created (wallpaper)" "$WORK/sway.log"; then fail "the compositor got a surface at the start"; fi
echo "first start checks passed"
