#!/usr/bin/env bash
set -euo pipefail
name=${1:?usage: GH_TOKEN=<token> $0 <artifact name> <job name>}
job=${2:?usage: GH_TOKEN=<token> $0 <artifact name> <job name>}
run=repos/$GITHUB_REPOSITORY/actions/runs/$GITHUB_RUN_ID
while [ "$SECONDS" -lt 2400 ]; do
  [ "$(gh api "$run/artifacts?name=$name" --jq .total_count)" -eq 0 ] || exit 0
  state=$(gh api "$run/jobs?per_page=100" --jq ".jobs[] | select(.name == \"$job\") | .conclusion // \"\"")
  case $state in
    failure | cancelled | skipped) echo "::error::$job ended as $state, so $name will not exist"; exit 1 ;;
  esac
  sleep 15
done
echo "::error::$name was not uploaded in time"
exit 1
