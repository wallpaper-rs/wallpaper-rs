import QtQuick
import io.github.wallpaper_rs.app

Column {
    required property var page
    readonly property alias away: away
    readonly property real room: !page.all && height > 0 ? height + Look.e3 : 0

    y: Math.max(0, page.bar.y - height - Look.e3)
    width: Math.min(parent.width, Look.tapaFoco * 2)
    visible: !page.all

    SyncedFiles {
        width: parent.width
    }

    AwayNote {
        id: away

        width: parent.width
    }
}
