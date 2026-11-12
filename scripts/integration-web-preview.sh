#!/usr/bin/env bash
set -euo pipefail
declare pages rgbs openrgb
folder=$pages/authored
ctl scene new "$folder" --from web --id org.example.authored --title Authored --locale en >/dev/null || fail "no page was started"
cp "$ROOT/tests/fixtures/green-64x32.png" "$folder/cover.png"
cat >"$folder/index.html" <<'PAGE'
<!doctype html>
<html>
<body style="margin:0;background:#ff0000">
<script src="app.js"></script>
</body>
</html>
PAGE
cat >"$folder/app.js" <<'PAGE'
console.log("page-console-line");
setTimeout(() => { throw new Error("page-error-line"); }, 300);
const red = new Uint8Array([255, 0, 0, 255, 255, 0, 0, 255]);
setInterval(() => wprs.rgb.setImage(red, 2, 1), 100);
wprs.onMedia(async (m) => {
  if (!m.enabled || !m.title) return;
  if (m.title === "Flood") {
    for (let i = 0; i < 300; i++) console.log("page-flood-line");
    setTimeout(() => console.log("page-late-line"), 4000);
    return;
  }
  const size = m.cover ? (await (await fetch(m.cover)).blob()).size : 0;
  console.log("track " + JSON.stringify({ title: m.title, artist: m.artist, position: m.position, cover: size > 0, colors: !!m.colors }));
});
PAGE
ctl scene set "$folder" /manifest/inputs '["media"]' >/dev/null
ctl scene set "$folder" /manifest/permissions '{"rgb": true}' >/dev/null
printf '{"title": "Made-up song", "artist": "Nobody", "playing": true, "position": 12, "duration": 200, "cover": "cover.png"}\n' >"$folder/track.json"
read -r _ _ _ node < <(ctl stream preview:authored 320 180)
ctl scene preview "$folder" -o preview:authored --media "$folder/track.json" || fail "the page folder was not previewed"
seen "$node" "0, 0, 255"
consoled() { field "'$1' in ' '.join(s['outputs']['preview:authored']['console'])" | grep -q True; }
wait_for 20 consoled "page-console-line" || fail "console.log of the page did not reach the preview: $(field "s['outputs']['preview:authored']['console']")"
wait_for 20 consoled "page-error-line" || fail "the error of the page did not reach the preview: $(field "s['outputs']['preview:authored']['console']")"
wait_for 20 consoled '"title":"Made-up song","artist":"Nobody","position":12.0' ||
  fail "the made-up track did not reach onMedia: $(field "s['outputs']['preview:authored']['console']")"
consoled '"cover":true,"colors":true' || fail "the cover and its colors did not reach onMedia: $(field "s['outputs']['preview:authored']['console']")"
printf '{"title": "Flood", "playing": true, "position": 1, "duration": 200}\n' >"$folder/flood.json"
ctl scene preview "$folder" -o preview:authored --media "$folder/flood.json" || fail "the flooding track was not shown"
wait_for 20 consoled "page-flood-line" || fail "the flood never reached the preview"
wait_for 30 consoled "page-late-line" || fail "a late line was lost after a flood: $(field "s['outputs']['preview:authored']['console']")"
[ "$(field "len(s['outputs']['preview:authored']['console'])")" -le 50 ] || fail "the preview kept more than 50 lines"
ctl config rgb.server "127.0.0.1:$openrgb" >/dev/null
ctl config rgb.enabled true
sleep 1
sent=$(wc -l <"$rgbs/painted")
ctl scene preview "$folder" -o preview:authored --restart || fail "the page folder was not previewed again"
strip() { [ "$(ctl rgb strip -o preview:authored --json 2>/dev/null | from "r['width'], r['height'], r['colors']")" = "(2, 1, ['#ff0000', '#ff0000'])" ]; }
wait_for 20 strip || fail "the page folder showed no strip: $(ctl rgb strip -o preview:authored --json 2>&1)"
sleep 1
[ "$(wc -l <"$rgbs/painted")" = "$sent" ] || fail "a preview of a page folder painted the devices: $(tail -n 2 "$rgbs/painted")"
ctl config rgb.enabled false
ctl scene preview "$folder" -o preview:authored --stop || fail "the preview of the page folder did not stop"
ctl release preview:authored
echo "page folder preview checks passed"
