import QtQuick
import io.github.wallpaper_rs.app

DropArea {
    required property var tools

    keys: ["text/uri-list"]
    onDropped: drop => {
        const paths = drop.urls.map(url => App.path(url)).filter(p => p);
        tools.dialogs.importer.bring(paths, false);
    }
}
