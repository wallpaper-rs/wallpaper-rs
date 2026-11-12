#!/usr/bin/env bash
set -euo pipefail
declare desktop
for output in HEADLESS-1 HEADLESS-2; do
  expect "s['outputs']['$output']['released']" True
  expect "s['outputs']['$output']['blank']" True
done
check HEADLESS-1 "0.5,0.5,$desktop" "0.9,0.9,$desktop"
swaymsg -q reload
echo "first start checks passed"
