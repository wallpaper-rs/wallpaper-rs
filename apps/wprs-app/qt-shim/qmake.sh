#!/bin/sh
here=$(dirname "$0")
if [ "$1" = "-query" ]; then
    case "$2" in
        QT_HOST_LIBEXECS | QT_HOST_LIBEXECS/get | QT_INSTALL_LIBEXECS | QT_INSTALL_LIBEXECS/get)
            echo "$here/libexec"
            exit 0
            ;;
    esac
fi
read -r real < "$here/real-qmake"
exec "$real" "$@"
