#!/usr/bin/env bash
set -euo pipefail
declare WORK
bro=$WORK/browse
mkdir -p "$bro"
png "$bro/a-cyan.png" 00ffff
png "$bro/b-pink.png" ff00ff
png "$bro/c-gold.png" ffd700
bro_source=$(ctl lib add "$bro" --wait | sed -n 1p)
read -r cyan pink gold < <(ctl lib search "sort:title" --json |
  js "' '.join(i['uid'] for i in r['items'] if i['title'] in ('a-cyan', 'b-pink', 'c-gold'))")
ctl stream TEST-31 160 90 >/dev/null
ctl playlist new Browsing >/dev/null
ctl playlist add Browsing "$cyan" "$pink" "$gold"
ctl playlist start Browsing -o TEST-31
shown="s['outputs']['TEST-31']['item']"
expect "$shown" "$cyan"
look() { js "r['active'], r['source'], (r['candidate'] or {}).get('uid'), r['steps'], r['position'], r['count']"; }
got=$(ctl next -o TEST-31 --preview --json | look)
[ "$got" = "(True, 'playlist', '$pink', 1, 1, 3)" ] || fail "browse start: $got"
got=$(ctl browse show -o TEST-31 --json | js "r['now']['uid'], ' '.join(a['uid'] for a in r['ahead']), r['plays']['kind'], r['plays']['why']")
[ "$got" = "('$cyan', '$gold $cyan', 'fade', 'general')" ] || fail "browse strip: $got"
ctl next -o TEST-31 --preview >/dev/null
got=$(ctl browse show -o TEST-31 --json | look)
[ "$got" = "(True, 'playlist', '$gold', 2, 2, 3)" ] || fail "browse next: $got"
ctl prev -o TEST-31 --preview >/dev/null
expect "$shown" "$cyan"
got=$(ctl browse cancel -o TEST-31 --json | js "r['active'], r['ended']")
[ "$got" = "(False, 'cancelled')" ] || fail "browse cancel: $got"
[ "$(ctl browse show -o TEST-31 --json | js "r['active']")" = False ] || fail "cancel kept the session"
expect "$shown" "$cyan"
if ctl browse apply -o TEST-31 2>/dev/null; then fail "apply without browsing"; fi
ctl next -o TEST-31 --preview >/dev/null && ctl next -o TEST-31 --preview >/dev/null
if ctl browse transition wobble -o TEST-31 2>/dev/null; then fail "an unknown transition was taken"; fi
ctl browse transition none -o TEST-31 >/dev/null
got=$(ctl browse apply -o TEST-31 --json | js "r['active'], r['ended']")
[ "$got" = "(False, 'applied')" ] || fail "browse apply: $got"
expect "$shown" "$gold"
expect "s['outputs']['TEST-31']['playlist']['name']" Browsing
ctl next -o TEST-31 && expect "$shown" "$cyan"
got=$(ctl next -o TEST-31 --preview --from library --json | js "r['source'], r['count'] >= 3")
[ "$got" = "('library', True)" ] || fail "browse the library: $got"
ctl browse cancel -o TEST-31 >/dev/null
expect "$shown" "$cyan"
opened() { [ -e "$WORK/preview-opened" ] && wc -l <"$WORK/preview-opened" || echo 0; }
window() {
  dbus-send --session --print-reply --dest=org.freedesktop.DBus /org/freedesktop/DBus \
    org.freedesktop.DBus.NameHasOwner string:io.github.wallpaper_rs.Preview | grep -q "boolean $1"
}
before=$(opened)
ctl next -o TEST-31 --preview >/dev/null
[ "$(opened)" = $((before + 1)) ] || fail "beginning to browse did not open the preview window"
[ "$(tail -n 1 "$WORK/preview-opened")" = "--browse TEST-31" ] || fail "the window opened for another screen"
ctl next -o TEST-31 --preview >/dev/null
[ "$(opened)" = $((before + 1)) ] || fail "moving opened a second preview window"
ctl browse apply -o TEST-31 >/dev/null
expect "$shown" "$gold"
ctl next -o TEST-31 --preview >/dev/null
expired() { [ "$(ctl browse show -o TEST-31 --json | js "r['active']")" = False ]; }
wait_for 20 expired || fail "an idle session did not expire"
expect "$shown" "$gold"
[ "$(opened)" = $((before + 2)) ] || fail "an expired session left no trace of its window"
if [ -x "$BIN/wprs-app" ] && python3 -c 'import dbus' 2>/dev/null; then
  python3 -c 'import dbus, time; dbus.SessionBus().request_name("io.github.wallpaper_rs.Preview"); time.sleep(60)' &
  holder=$!
  wait_for 10 window true || fail "the stand-in window did not take its name"
  ctl next -o TEST-31 --preview >/dev/null
  [ "$(opened)" = $((before + 2)) ] || fail "a session opened a second window over the one shown"
  timeout 20 "$BIN/wprs-app" --browse TEST-31 || fail "a second preview window did not leave"
  window true || fail "the second window took the first one's place"
  ctl browse cancel -o TEST-31 >/dev/null
  kill "$holder" 2>/dev/null || true
  wait "$holder" 2>/dev/null || true
  echo "single preview window checks passed"
fi
kirigami=$(qtpaths6 --query QT_INSTALL_QML 2>/dev/null || true)/org/kde/kirigami
if [ -x "$BIN/wprs-app" ] && [ -d "$kirigami" ]; then
  touch "$WORK/preview-real"
  ctl next -o TEST-31 --preview >/dev/null
  wait_for 30 window true || fail "the preview window did not open"
  ctl browse apply -o TEST-31 >/dev/null
  wait_for 20 window false || fail "the preview window stayed after applying"
  rm -f "$WORK/preview-real"
  echo "preview window checks passed"
fi
ctl playlist stop -o TEST-31
ctl playlist rm Browsing
ctl lib unsource "$bro_source"
ctl release TEST-31 && expect "list(s['outputs'])" "[]"
echo "browse checks passed"
