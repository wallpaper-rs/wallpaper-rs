import QtQuick
import io.github.wallpaper_rs.app

DropArea {
    required property var tools

    keys: ["text/uri-list"]
    onDropped: drop => {
        const paths = drop.urls.map(url => App.path(url)).filter(p => p);
        tools.dialogs.importer.bring(paths, false);
    }

    Rectangle {
        anchors.fill: parent
        visible: parent.containsDrag
        color: Look.nocheAlta
        opacity: 0.94

        Vacio {
            anchors.centerIn: parent
            title: App.tr("Drop to add them to your library")
            text: App.tr("Pictures, videos, scenes or a whole folder. Then we ask whether to copy them or leave them where they are.")
        }
    }
}
