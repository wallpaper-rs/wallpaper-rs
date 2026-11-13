#!/usr/bin/env bash
set -euo pipefail
declare URL
f() { DBUS_SESSION_BUS_ADDRESS=unix:path=$WORK/f/bus "$BIN/wprs-ctl" "$@"; }
device f
f server add "$URL" --wait >/dev/null
link=$(f server connect --name Pocket)
api -d "{\"user_code\": \"$(sed -n 's/^code = //p' <<<"$link")\", \"approve\": true}" "$URL/api/v1/auth/device/approve" >/dev/null
await f "$(tail -n 1 <<<"$link")"
f_id=$(sql "SELECT id FROM devices ORDER BY number DESC LIMIT 1")
[ "$(value f "['state'], r['account']['warning'], r['account']['encrypted'], r['account']['records']")" = "('connected', None, None, 0)" ] ||
  fail "a connected device: $(f server status --json)"
f server status | grep -q "connected, not syncing" || fail "ctl does not show the level: $(f server status)"
[ "$(f remote ls --json | jq_ "len(r['repos'])")" = 1 ] || fail "a connected device got no repository: $(f remote ls)"
f remote refresh --wait >/dev/null
f pkg install org.example.dusk --wait >/dev/null || fail "a connected device cannot install"
f remote votes org.example.dusk | grep -q '"shown":true' || fail "a connected device sees no votes"
if f server sync --wait 2>"$WORK/connected.err"; then fail "a connected device synced"; fi
grep -q "syncing is off" "$WORK/connected.err" || fail "sync without syncing: $(cat "$WORK/connected.err")"
if f server join "Device 1" 2>/dev/null; then fail "a connected device asked for the key"; fi
sleep 7
[ "$(sql "SELECT count(*) FROM sync_records WHERE device_id = '$f_id'")" = 0 ] || fail "a connected device sent records"
[ "$(f playlist ls --json | jq_ "len(r['playlists'])")" = 0 ] || fail "a connected device received playlists"
f server syncing on --wait >/dev/null
[ "$(value f "['state']")" = needs_key ] || fail "syncing on an account with a key: $(f server status --json)"
f_name=$(value f "['device']")
a_name=$(value a "['device']")
read -r job held joined < <(compare f a "$a_name" "$f_name")
a server share "$f_name" --code "$joined" --wait >/dev/null
f server join "$a_name" --code "$held" --wait >/dev/null
await f "$job"
f server sync --wait >/dev/null
[ "$(value f "['state']")" = ready ] || fail "syncing turned on later: $(f server status --json)"
[ "$(f playlist ls --json | jq_ "len(r['playlists'])")" -ge 1 ] || fail "nothing arrived once syncing was on"
f server syncing off --wait >/dev/null
[ "$(value f "['state'], r['account']['device']")" = "('connected', '$f_name')" ] || fail "turning syncing off: $(f server status --json)"
if f server sync 2>/dev/null; then fail "a device with syncing off synced"; fi
f remote votes org.example.dusk | grep -q '"shown":true' || fail "turning syncing off lost the connection"
f server syncing on --wait >/dev/null
f server sync --wait >/dev/null
[ "$(value f "['state']")" = ready ] || fail "syncing on again with the kept key: $(f server status --json)"
f server remove
echo "connect without syncing checks passed"
