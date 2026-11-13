#!/usr/bin/env bash
set -euo pipefail
declare site dusk
release() {
  (
    cd "$site"
    rm -rf bump
    cp -r dusk bump
    python3 -c "import json, sys; m = json.load(open(sys.argv[1])); m['version'] = sys.argv[2]
m['inputs'] = m.get('inputs', []) + sys.argv[3:]
json.dump(m, open(sys.argv[1], 'w'))" bump/manifest.json "$@"
    ctl pack build bump -o "dusk-$1.wpak" >/dev/null
    ctl repo add repo "dusk-$1.wpak" >/dev/null
    ctl repo publish repo --key online.key >/dev/null
  )
  ctl remote refresh --wait >/dev/null
}
versions() { ctl pkg ls --json | jq_ "sorted(p['version'] for p in r['packages'])"; }
newest() { [ "$(versions)" = "['$1']" ]; }
ctl pkg install "$dusk" --wait >/dev/null
expect "s['options']['repos.auto_update']" False
release 1.2.0
[ "$(versions)" = "['1.1.0']" ] || fail "a package was updated without the setting: $(versions)"
[ "$(ctl pkg ls --json | jq_ "[p['update'] for p in r['packages']]")" = "['1.2.0']" ] ||
  fail "the update is not listed: $(ctl pkg ls --json)"
ctl config repos.auto_update true
wait_for 20 newest 1.2.0 || fail "the setting did not update the package: $(versions)"
ctl lib apply "$(ctl pkg ls --json | jq_ "r['packages'][0]['item']")" >/dev/null
release 1.2.1
wait_for 20 newest 1.2.1 || fail "the version shown on a screen was kept after the update: $(versions)"
grep -q "1.2.1.wpak" "$XDG_CONFIG_HOME/wallpaper-rs/config.toml" || fail "the screen was not moved to the new version"
release 1.3.0 audio
held() { [ "$(ctl pkg ls --json | jq_ "any(p.get('asks') for p in r['packages'])")" = True ]; }
wait_for 20 held || fail "no note for the new permission: $(ctl pkg ls --json)"
[ "$(ctl pkg ls --json | jq_ "[(p['version'], p['update'], p['asks']) for p in r['packages'] if p['update']]")" = \
  "[('1.2.1', '1.3.0', ['audio'])]" ] || fail "the held update: $(ctl pkg ls --json)"
ctl pkg ls | grep -q "asks for new permissions (audio)" || fail "the listing does not say what the update asks for"
ctl pkg upgrade >/dev/null
newest 1.2.1 || fail "an update with a new permission was installed by the bulk action: $(versions)"
ctl pkg install "$dusk" --wait >/dev/null
newest 1.3.0 || fail "the update asked for by hand was not installed: $(versions)"
ctl config repos.auto_update false
ctl pkg uninstall "$dusk"
echo "update checks passed"
