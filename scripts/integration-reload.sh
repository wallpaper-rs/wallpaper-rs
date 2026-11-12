#!/usr/bin/env bash
set -euo pipefail
declare scene
reload=$scene/reload
ctl scene new "$reload" >/dev/null
printf '%s\n' 'export function init() { console.log("first version of the code"); }' >"$WORK/reload.js"
ctl scene import-script "$reload" "$WORK/reload.js" >/dev/null || fail "the reloading script was not imported"
shown() {
  timeout 8 "$BIN/wprs-ctl" scene preview "$reload" -o preview:1 --console >"$WORK/reload.out" 2>&1 || true
  grep -q "$1" "$WORK/reload.out"
}
shown "first version of the code" || fail "the script did not run in the preview: $(cat "$WORK/reload.out")"
printf '%s\n' 'export function init() { console.log("second version of the code"); }' >"$reload/scripts/reload.js"
shown "second version of the code" || fail "the preview kept the old code after the file changed: $(cat "$WORK/reload.out")"
