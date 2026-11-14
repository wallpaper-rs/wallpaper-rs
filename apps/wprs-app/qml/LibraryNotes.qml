import QtQuick
import io.github.wallpaper_rs.app

Column {
    required property var page

    y: Math.max(0, page.bar.y - height - Look.e3)
    width: Math.min(parent.width, Look.tapaFoco * 2)
    visible: !page.all

    SyncedFiles {
        width: parent.width
    }

    AwayNote {
        width: parent.width
    }
}
