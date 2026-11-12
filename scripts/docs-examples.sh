#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
ctl=${1:-target/debug}/wprs-ctl
count=0
failed=0
for dir in docs/crear/ejemplos/*/; do
  dir=${dir%/}
  count=$((count + 1))
  if [ ! -f "$dir/scene.json" ] && [ -f "$dir/manifest.json" ]; then
    if ! "$ctl" pack check "$dir" >/dev/null; then
      echo "does not pass wprs-ctl pack check: $dir" >&2
      "$ctl" pack check "$dir" >&2 || true
      failed=$((failed + 1))
    fi
  elif [ ! -f "$dir/scene.json" ]; then
    echo "neither scene.json nor manifest.json: $dir" >&2
    failed=$((failed + 1))
  elif ! "$ctl" scene validate "$dir" >/dev/null; then
    echo "does not pass wprs-ctl scene validate: $dir" >&2
    "$ctl" scene validate "$dir" >&2 || true
    failed=$((failed + 1))
  fi
done
if [ "$count" -eq 0 ]; then
  echo "no example folders under docs/crear/ejemplos" >&2
  exit 1
fi
if [ "$failed" -gt 0 ]; then
  echo "$failed of $count examples failed" >&2
  exit 1
fi
echo "$count examples pass"
