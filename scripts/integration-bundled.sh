#!/usr/bin/env bash
set -euo pipefail
declare daemon
shipped=$XDG_DATA_HOME/wallpaper-rs/examples
mkdir -p "$shipped"
count=0
for scene in "$ROOT"/scenes/*/; do
  ctl pack build "$scene" -o "$shipped/$(basename "$scene").wpak" >/dev/null
  count=$((count + 1))
done
stop_daemon "$daemon" "$WORK/wprsd.log"
start_daemon "$WORK/bundled.log"
examples() { ctl remote ls --json | jq_ "[(x['packages'], x['broken']) for x in r['repos'] if x['bundled']]"; }
ready() { [ "$(examples)" = "[($count, None)]" ]; }
wait_for 30 ready || fail "the example repository was not added at the first start: $(ctl remote ls --json)"
id=$(ctl remote ls --json | jq_ "[x['repo_id'] for x in r['repos'] if x['bundled']][0]")
[ "$(ctl remote ls --json | jq_ "[x['name'] for x in r['repos'] if x['bundled']][0]")" = "Example wallpapers" ] ||
  fail "the example repository has no name"
[ "$(ctl pkg search pond --json | jq_ "[p['id'] for p in r['packages']]")" = "['io.github.wallpaper_rs.example.pond']" ] ||
  fail "the examples cannot be browsed: $(ctl pkg search pond --json)"
if ctl remote examples --wait 2>"$WORK/bundled.err"; then fail "the examples were added twice"; fi
ctl remote rm "$id"
stop_daemon "$daemon" "$WORK/bundled.log"
start_daemon "$WORK/bundled2.log"
sleep 2
[ "$(examples)" = "[]" ] || fail "the removed example repository came back: $(ctl remote ls --json)"
ctl remote examples --wait >/dev/null
wait_for 30 ready || fail "the example repository was not added again: $(ctl remote ls --json)"
[ "$(ctl remote ls --json | jq_ "[x['repo_id'] for x in r['repos'] if x['bundled']][0]")" = "$id" ] ||
  fail "the example repository changed identity"
ctl remote rm "$id"
rm -rf "$shipped"
stop_daemon "$daemon" "$WORK/bundled2.log"
start_daemon "$WORK/bundled3.log"
echo "example repository checks passed"
