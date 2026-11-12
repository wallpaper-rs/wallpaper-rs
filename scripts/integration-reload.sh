#!/usr/bin/env bash
set -euo pipefail
declare scene
reload=$scene/reload
ctl scene new "$reload" >/dev/null
printf '%s\n' 'export function init() { console.log("first version of the code"); }' >"$WORK/reload.js"
ctl scene import-script "$reload" "$WORK/reload.js" >/dev/null || fail "the reloading script was not imported"
read -r _ _ _ reload_stream < <(ctl stream preview:1 160 90)
printed() {
  (while :; do "$BIN/probe" "$reload_stream" 1 >/dev/null 2>&1 || sleep 0.1; done) &
  local watcher=$!
  timeout 8 "$BIN/wprs-ctl" scene preview "$reload" -o preview:1 --console >"$WORK/reload.out" 2>&1 || true
  kill "$watcher" 2>/dev/null || true
  wait "$watcher" 2>/dev/null || true
  grep -q "$1" "$WORK/reload.out"
}
printed "first version of the code" || fail "the script did not run in the preview: $(cat "$WORK/reload.out")"
printf '%s\n' 'export function init() { console.log("second version of the code"); }' >"$reload/scripts/reload.js"
printed "second version of the code" || fail "the preview kept the old code after the file changed: $(cat "$WORK/reload.out")"
! grep -q "first version of the code" "$WORK/reload.out" ||
  fail "the console began with lines of the scene shown before: $(cat "$WORK/reload.out")"
ctl release preview:1
