#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")"/integration-env.sh "$@"
start_daemon "$WORK/wprsd.log"
source "$(dirname "$0")"/integration-web.sh
stop_daemon "$daemon" "$WORK/wprsd.log"
status=0
echo "web test passed"
