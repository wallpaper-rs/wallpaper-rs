#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."
dir=android/app/src/androidTest/kotlin
bad=0
for file in "$dir"/*.kt; do
  grep -q '@Test' "$file" || continue
  if ! grep -q '@RunWith(Baselined::class)' "$file" || grep -q 'AndroidJUnit4::class' "$file"; then
    echo "$file does not run with Baselined"
    bad=1
  fi
done
if grep -n 'Native.power(' "$dir"/*.kt | grep -v 'Device.kt'; then
  echo "tests restore the power state through Device.powerReset, not by hand"
  bad=1
fi
if grep -nE 'dumpsys battery|cmd power set-mode|cmd notification (allow|disallow)_listener|settings put global' "$dir"/*.kt | grep -vE '/(Device|Probes|BaselineGuardTest)\.kt'; then
  echo "global device state is changed through Device helpers only"
  bad=1
fi
exit $bad
