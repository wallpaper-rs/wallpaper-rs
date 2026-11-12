#!/usr/bin/env bash
set -euo pipefail
declare WORK
made=$WORK/made
mkdir -p "$made"
ctl scene new "$made/clock" --from web --title Clock --locale en >/dev/null || fail "no web page was started"
{ [ -f "$made/clock/index.html" ] && [ -f "$made/clock/app.js" ]; } || fail "the web starter wrote no page"
ctl scene open "$made/clock" | grep -q "index.html: entry, page" || fail "the files of the page were not listed: $(ctl scene open "$made/clock")"
echo "console.log('ready')" >"$made/extra.js"
ctl scene manifest "$made/clock" add-file "$made/extra.js" --to lib || fail "no file was added to the page"
[ -f "$made/clock/lib/extra.js" ] || fail "the added file is not in the page"
if ctl scene manifest "$made/clock" add-file "$made/clock/manifest.json" 2>/dev/null; then fail "a file replaced the manifest"; fi
ctl scene validate "$made/clock" >/dev/null || fail "the web starter has problems: $(ctl scene validate "$made/clock")"
ctl pack build "$made/clock" -o "$made/clock.wpak" >/dev/null || fail "the web starter does not build"
echo "web page authoring checks passed"
ctl stream preview:pages 320 180 >/dev/null
printf 'console.log("page-preview-ready");\n' >>"$made/clock/app.js"
ctl scene preview "$made/clock" -o preview:pages --quality q1 --restart || fail "a web page was not previewed"
printf '{"title": "Made-up song", "artist": "Nobody", "playing": true, "position": 12, "duration": 200}\n' >"$made/track.json"
ctl scene preview "$made/clock" -o preview:pages --media "$made/track.json" || fail "a web page was not previewed with a made-up track"
ctl scene preview "$made/clock" -o preview:pages --stop || fail "the preview of a web page did not stop"
ctl release preview:pages
echo "web page preview checks passed"
