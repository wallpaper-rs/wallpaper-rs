#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

cache() {
  jq -n --argjson id "$1" --arg key "$2" --arg ref "$3" --arg created "$4" --arg used "$5" --argjson size "$6" \
    '{id: $id, key: $key, ref: $ref, created_at: $created, last_accessed_at: $used, size_in_bytes: $size}'
}

run() {
  jq -s . > "$work/listing.json"
  CACHE_LIST_FILE=$work/listing.json CACHE_RUN_START=$1 CACHE_NOW=$2 scripts/ci-cache-clean.sh |
    awk '$1 == "would" { print $3 }' | sort -n | tr '\n' ' '
}

expect() {
  local name=$1 got=$2 want=$3
  if [ "$got" != "$want" ]; then
    echo "FAIL $name: deleted [$got], wanted [$want]" >&2
    exit 1
  fi
  echo "ok $name"
}

m=refs/heads/master
old=v0-rust-dev-Linux-x64-aaaaaaaa-11111111
new=v0-rust-dev-Linux-x64-aaaaaaaa-22222222
android_old=v0-rust-arm64-v8a-android-native-Linux-x64-aaaaaaaa-11111111
android_new=v0-rust-arm64-v8a-android-native-Linux-x64-aaaaaaaa-22222222

got=$(
  {
    cache 1 "$old" $m 2026-11-10T10:00:00Z 2026-11-12T10:05:00Z 600
    cache 2 "$new" $m 2026-11-12T10:20:00Z 2026-11-12T10:20:00Z 600
    cache 3 "$android_old" $m 2026-11-10T10:00:00Z 2026-11-12T10:05:00Z 500
    cache 4 server-image-bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb $m 2026-11-11T10:00:00Z 2026-11-11T10:00:00Z 700
    cache 5 server-image-cccccccccccccccccccccccccccccccccccccccc $m 2026-11-12T10:15:00Z 2026-11-12T10:15:00Z 700
  } | run 2026-11-12T10:00:00Z 2026-11-12T10:30:00Z
)
expect "new generation saved: the previous one goes, a family without a new one stays" "$got" "1 4 "

got=$(
  {
    cache 1 "$old" $m 2026-11-10T10:00:00Z 2026-11-11T10:00:00Z 600
    cache 2 "$new" $m 2026-11-12T08:00:00Z 2026-11-12T10:05:00Z 600
    cache 3 "$android_old" $m 2026-11-10T10:00:00Z 2026-11-11T10:05:00Z 500
    cache 4 "$android_new" $m 2026-11-12T08:00:00Z 2026-11-12T10:05:00Z 500
  } | run 2026-11-12T10:00:00Z 2026-11-12T10:30:00Z
)
expect "warm run: only the generation in use stays" "$got" "1 3 "

got=$(
  {
    cache 1 "$old" $m 2026-11-10T10:00:00Z 2026-11-12T10:05:00Z 600
    cache 2 "$new" $m 2026-11-12T08:00:00Z 2026-11-12T08:00:00Z 600
  } | run 2026-11-12T10:00:00Z 2026-11-12T10:30:00Z
)
expect "rerun of an older commit keeps what it read" "$got" ""

got=$(
  {
    cache 1 "$old" $m 2026-11-01T10:00:00Z 2026-11-05T10:05:00Z 600
    cache 2 "$new" $m 2026-11-10T10:00:00Z 2026-11-12T10:05:00Z 600
    cache 3 depth-anything-v2-small-46c4e8ee-onnxruntime-1.31.0 $m 2026-11-01T10:00:00Z 2026-11-05T10:05:00Z 100
    cache 4 apt-build-dddddddddddddddddddddddddddddddd $m 2026-11-01T10:00:00Z 2026-11-11T10:05:00Z 100
    cache 5 setup-java-Linux-x64-gradle-eeeeeeeeeeeeeeee $m 2026-10-01T10:00:00Z 2026-11-01T10:05:00Z 100
  } | run 2026-11-12T10:00:00Z 2026-11-12T10:30:00Z
)
expect "older generation idle 7 days deleted, only generation of a family idle 10 days kept" "$got" "1 "

got=$(
  {
    cache 1 "$new" refs/pull/7/merge 2026-11-12T10:10:00Z 2026-11-12T10:10:00Z 600
    cache 2 "$old" $m 2026-11-10T10:00:00Z 2026-11-12T10:05:00Z 600
    cache 3 "$new" $m 2026-11-12T10:20:00Z 2026-11-12T10:20:00Z 600
  } | run 2026-11-12T10:00:00Z 2026-11-12T10:30:00Z
)
expect "generations are judged within one branch" "$got" "2 "

printf '#!/bin/sh\nexit 1\n' > "$work/gh"
chmod +x "$work/gh"
status=0
GITHUB_REPOSITORY=none/none GITHUB_RUN_ID=1 PATH="$work:$PATH" scripts/ci-cache-clean.sh > /dev/null 2>&1 || status=$?
expect "a failing API call does not fail the cleanup" "$status" "0"
