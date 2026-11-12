#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
shards=(
  "wprs-render wprs-scene wprs-core wprs-ktx2 wprs-depth wprs-video wprs-mix wprs-audio wprs-script wprs-rgb wprs-schedule wprs-editor wprs-type"
  "wprsd wprs-ctl wprs-web wprs-pipewire wprs-wayland"
  "wprs-app wprs-android wprs-server wprs-sync wprs-library wprs-format wprs-providers wprs-client"
)
case ${1:?usage: $0 <shard number>|check} in
  check)
    listed=$(printf '%s\n' "${shards[@]}" | tr ' ' '\n' | sort)
    workspace=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[].name' | sort)
    if [ "$listed" != "$workspace" ]; then
      diff <(echo "$listed") <(echo "$workspace") >&2 || true
      echo "the test shards and the workspace packages differ" >&2
      exit 1
    fi
    echo "${#shards[@]} shards cover $(echo "$workspace" | wc -l) packages"
    ;;
  [1-9])
    for package in ${shards[$(($1 - 1))]}; do printf '%s\n%s\n' -p "$package"; done
    ;;
  *) echo "unknown shard $1" >&2; exit 2 ;;
esac
