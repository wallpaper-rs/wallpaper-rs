#!/usr/bin/env bash
set -euo pipefail
declare BIN ROOT pages tint web
ctl pack build "$ROOT/tests/fixtures/keep" -o "$pages/keep.wpak" >/dev/null && ctl lib import "$pages/keep.wpak" --wait >/dev/null
keep=$(uid org.example.keep)
ctl lib apply "$keep" -o TEST-W 2>/dev/null
seen "$web" "255, 0, 0"
remembered() {
  ctl lib apply "$tint" -o TEST-W 2>/dev/null
  sleep 2
  ctl lib apply "$keep" -o TEST-W 2>/dev/null
  wait_for 8 pixel "$web" "0, 255, 0"
}
for _ in 1 2 3 4 5; do remembered && break; done
pixel "$web" "0, 255, 0" || fail "the page did not find what it stored on an earlier run: $("$BIN/probe" "$web" 3 2>&1 | tail -n 1)"
ctl lib clear-data "$keep"
wait_for 20 pixel "$web" "255, 0, 0" || fail "the page still remembered after its data was cleared"
echo "web page storage checks passed"
