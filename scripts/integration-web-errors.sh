#!/usr/bin/env bash
set -euo pipefail
declare BIN ROOT pages web
ctl pack build "$ROOT/tests/fixtures/boom" -o "$pages/boom.wpak" >/dev/null && ctl lib import "$pages/boom.wpak" --wait >/dev/null
boom=$(uid org.example.boom)
ctl lib apply "$boom" -o TEST-W 2>/dev/null
raised() { [[ "$(field "s['outputs']['TEST-W']['page_errors']")" == *"boom at start"* ]]; }
wait_for 20 raised || fail "the error of a page that threw at start was not reported: $(ctl state)"
ctl lib show "$boom" | grep "^error = The page reported an error and may not be drawing: .*boom at start" >/dev/null ||
  fail "the library does not say the page has an error: $(ctl lib show "$boom")"
echo "web page error checks passed"
