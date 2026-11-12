#!/usr/bin/env bash
set -uo pipefail
max_age_days=${CACHE_MAX_AGE_DAYS:-6}
limit_gb=${CACHE_LIMIT_GB:-10}
dry=${CACHE_DRY_RUN:-}

skip() {
  echo "::warning::cache cleanup skipped: $*"
  exit 0
}

if [ -n "${CACHE_LIST_FILE:-}" ]; then
  dry=1
  listing=$(cat "$CACHE_LIST_FILE") || skip "cannot read $CACHE_LIST_FILE"
else
  repo=${GITHUB_REPOSITORY:?}
  listing=$(gh api --paginate "repos/$repo/actions/caches?per_page=100" --jq '.actions_caches[]' | jq -s .) ||
    skip "the cache listing failed"
fi
jq -e 'type == "array"' <<< "$listing" > /dev/null 2>&1 || skip "the cache listing is not a list"

started=${CACHE_RUN_START:-}
if [ -z "$started" ]; then
  started=$(gh api "repos/${GITHUB_REPOSITORY:?}/actions/runs/${GITHUB_RUN_ID:?}" --jq .run_started_at) ||
    skip "the start time of this run is unknown"
fi
now=${CACHE_NOW:-$(date -u +%Y-%m-%dT%H:%M:%SZ)}

plan=$(jq -c --arg started "$started" --arg now "$now" --argjson age "$max_age_days" '
  def family: gsub("(?<=-)[0-9a-f]{8,}(?=-|$)"; "#");
  def secs: sub("\\.[0-9]+Z$"; "Z") | fromdateiso8601;
  ($started | secs) as $run
  | (($now | secs) - $age * 86400) as $cutoff
  | map(. + {family: (.key | family), created: (.created_at | secs), used: (.last_accessed_at | secs)}) as $all
  | ($all | group_by([.ref, .family]) | map(
      (map(select(.created >= $run))) as $fresh
      | (map(select(.used >= $run))) as $touched
      | (max_by(.created)) as $newest
      | (if ($fresh | length) > 0 then $fresh else $touched + [$newest] end | map(.id)) as $keep
      | map(select(.id as $id | $keep | index($id) | not) | . + {reason: "older generation of the same cache"})
    ) | add // []) as $old
  | ($old | map(.id)) as $old_ids
  | [$all[] | select(.used < $cutoff and .created < $run and (.id as $id | $old_ids | index($id) | not))
      | . + {reason: "not read for more than \($age) days"}] as $idle
  | {total: ($all | map(.size_in_bytes) | add // 0), count: ($all | length), delete: ($old + $idle)}
' <<< "$listing") || skip "the cache plan could not be computed"

gb() { awk -v b="$1" 'BEGIN { printf "%.2f", b / 1e9 }'; }

echo "run started $started, caches before: $(jq .count <<< "$plan") using $(gb "$(jq .total <<< "$plan")") GB of $limit_gb GB"
deleted=0
freed=0
while IFS=$'\t' read -r id size key reason; do
  [ -n "$id" ] || continue
  if [ -n "$dry" ]; then
    echo "would delete $id $(gb "$size") GB $key ($reason)"
    deleted=$((deleted + 1))
    freed=$((freed + size))
  elif gh api --method DELETE "repos/${GITHUB_REPOSITORY:?}/actions/caches/$id" > /dev/null 2>&1; then
    echo "deleted $id $(gb "$size") GB $key ($reason)"
    deleted=$((deleted + 1))
    freed=$((freed + size))
  else
    echo "::warning::could not delete cache $id $key"
  fi
done < <(jq -r '.delete[] | [.id, .size_in_bytes, .key, .reason] | @tsv' <<< "$plan")

remaining=$(($(jq .total <<< "$plan") - freed))
echo "deleted $deleted caches, $(gb "$freed") GB; remaining $(($(jq .count <<< "$plan") - deleted)) caches, $(gb "$remaining") GB of $limit_gb GB"
exit 0
