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
ctl playlist new Browsing >/dev/null
ctl playlist add Browsing "$cyan" "$pink" "$gold"
ctl playlist start Browsing -o TEST-8
shown="s['outputs']['TEST-8']['item']"
expect "$shown" "$cyan"
look() { js "r['active'], r['source'], (r['candidate'] or {}).get('uid'), r['steps'], r['position'], r['count']"; }
got=$(ctl next -o TEST-8 --preview --json | look)
[ "$got" = "(True, 'playlist', '$pink', 1, 1, 3)" ] || fail "browse start: $got"
ctl next -o TEST-8 --preview >/dev/null
got=$(ctl browse show -o TEST-8 --json | look)
[ "$got" = "(True, 'playlist', '$gold', 2, 2, 3)" ] || fail "browse next: $got"
ctl prev -o TEST-8 --preview >/dev/null
expect "$shown" "$cyan"
got=$(ctl browse cancel -o TEST-8 --json | js "r['active'], r['ended']")
[ "$got" = "(False, 'cancelled')" ] || fail "browse cancel: $got"
[ "$(ctl browse show -o TEST-8 --json | js "r['active']")" = False ] || fail "cancel kept the session"
expect "$shown" "$cyan"
if ctl browse apply -o TEST-8 2>/dev/null; then fail "apply without browsing"; fi
ctl next -o TEST-8 --preview >/dev/null && ctl next -o TEST-8 --preview >/dev/null
if ctl browse transition wobble -o TEST-8 2>/dev/null; then fail "an unknown transition was taken"; fi
ctl browse transition none -o TEST-8 >/dev/null
got=$(ctl browse apply -o TEST-8 --json | js "r['active'], r['ended']")
[ "$got" = "(False, 'applied')" ] || fail "browse apply: $got"
expect "$shown" "$gold"
expect "s['outputs']['TEST-8']['playlist']['name']" Browsing
ctl next -o TEST-8 && expect "$shown" "$cyan"
got=$(ctl next -o TEST-8 --preview --from library --json | js "r['source'], r['count'] >= 3")
[ "$got" = "('library', True)" ] || fail "browse the library: $got"
ctl browse cancel -o TEST-8 >/dev/null
expect "$shown" "$cyan"
ctl playlist stop -o TEST-8
ctl playlist rm Browsing
ctl lib unsource "$bro_source"
echo "browse checks passed"
