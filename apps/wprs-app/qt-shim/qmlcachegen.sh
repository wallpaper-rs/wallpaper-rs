#!/bin/sh
here=$(cd "$(dirname "$0")/.." && pwd)
read -r REAL_QMLCACHEGEN < "$here/real-qmlcachegen"
read -r mode < "$here/mode"
run="$here/run"
case "$*" in
    *--resource-path*) ;;
    *--resource-name*)
        if [ "$mode" = check ]; then
            while [ $# -gt 0 ]; do
                case "$1" in
                    --resource-name) name=$2 ;;
                    -o) out=$2 ;;
                esac
                shift
            done
            echo "int qInitResources_$name() { return 1; }" > "$out"
            exit 0
        fi
        exec "$REAL_QMLCACHEGEN" "$@"
        ;;
    *) exec "$REAL_QMLCACHEGEN" "$@" ;;
esac
while [ $# -gt 0 ]; do
    case "$1" in
        -i) qmldir=$2 ;;
        --resource) qrc=$2 ;;
        --resource-path) path=$2 ;;
        -o) out=$2 ;;
    esac
    last=$1
    shift
done
file=$last
if mkdir "$run/started" 2> /dev/null; then
    alias=$(sed -n "s|^ *<file alias=\"\([^\"]*\)\">$file</file>\$|\1|p" "$qrc" | head -n 1)
    prefix=${path%"$alias"}
    mkdir -p "$run/discard"
    sed -n 's|^ *<file alias="\(qml/[^"]*\.qml\)">\(.*\)</file>$|\1 \2|p' "$qrc" |
        while read -r name source; do
            target=$(dirname "$out")/$(basename "$name").cpp
            [ "$mode" = check ] && target="$run/discard/$(basename "$name").cpp"
            printf '%s\0%s\0%s\0' "$prefix$name" "$target" "$source"
        done > "$run/jobs"
    export REAL_QMLCACHEGEN qmldir qrc
    if xargs -0 -n 3 -P "$(nproc)" sh -c \
        "exec \"\$REAL_QMLCACHEGEN\" -i \"\$qmldir\" --resource \"\$qrc\" --resource-path \"\$1\" -o \"\$2\" \"\$3\"" sh \
        < "$run/jobs" 2> "$run/errors"; then
        : > "$run/done"
    else
        : > "$run/failed"
    fi
fi
if [ -e "$run/failed" ]; then
    cat "$run/errors" >&2
    exit 1
fi
[ "$mode" = check ] && : > "$out"
exit 0
