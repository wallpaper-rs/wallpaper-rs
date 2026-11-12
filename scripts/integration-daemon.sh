#!/usr/bin/env bash
set -euo pipefail
declare BIN log
name_free() { ! dbus-send --session --print-reply --dest=org.freedesktop.DBus /org/freedesktop/DBus \
  org.freedesktop.DBus.NameHasOwner string:io.github.wallpaper_rs.Daemon | grep -q "boolean true"; }
daemon_exited() {
  local state
  state=$(sed -n 's/^State:[[:space:]]*\(.\).*/\1/p' "/proc/$1/status" 2>/dev/null)
  [ -z "$state" ] || [ "$state" = Z ]
}

stop_daemon() {
  local pid=$1 out=${2:-} within=${3:-10} timer status=0
  kill "$pid" 2>/dev/null || true
  ( sleep "$within"; kill -KILL "$pid" 2>/dev/null ) & timer=$!
  wait "$pid" 2>/dev/null || status=$?
  pkill -P "$timer" 2>/dev/null || true
  kill "$timer" 2>/dev/null || true
  wait "$timer" 2>/dev/null || true
  if [ "$status" = 137 ]; then
    [ -n "$out" ] && tail -n 40 "$out"
    fail "wprsd $pid did not exit within $within s of the stop signal"
  fi
  wait_for 10 name_free || fail "wprsd $pid exited but io.github.wallpaper_rs.Daemon is still owned on the private session bus"
}

await_daemon() {
  local pid=$1 out=$2 ends=$((SECONDS + 30))
  shift 2
  while ! "$@" >/dev/null 2>&1; do
    if daemon_exited "$pid"; then
      [ -s "$out" ] && tail -n 40 "$out"
      fail "wprsd $pid exited while starting"
    fi
    [ "$SECONDS" -lt "$ends" ] || fail "wprsd $pid did not answer within 30 s: $(tail -n 5 "$out")"
    sleep 0.1
  done
}

start_daemon() {
  local out=$1
  shift
  wait_for 10 name_free || fail "io.github.wallpaper_rs.Daemon is still owned on the private session bus"
  RUST_LOG=$log "$BIN/wprsd" "$@" >>"$out" 2>&1 & daemon=$!
  pids+=("$daemon")
  await_daemon "$daemon" "$out" ctl state
}
