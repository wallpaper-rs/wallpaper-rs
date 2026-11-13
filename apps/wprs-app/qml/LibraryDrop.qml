import QtQuick
import io.github.wallpaper_rs.app

DropArea {
    keys: ["text/uri-list"]
    onDropped: drop => {
        const paths = drop.urls.map(url => App.path(url)).filter(p => p);
        if (paths.length > 0)
            App.act("Import", [paths, false], "Library1");
    }
}
