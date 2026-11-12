import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

DropArea {
    id: drop

    required property var page
    property bool layers: false
    readonly property var kinds: ({
            "image": ["png", "jpg", "jpeg", "webp", "gif", "bmp", "ktx2"],
            "video": ["webm", "mp4", "mkv"],
            "sound": ["ogg", "opus", "flac", "wav", "mp3"]
        })

    function kindOf(file) {
        const ext = file.split(".").pop().toLowerCase();
        return Object.keys(kinds).find(kind => kinds[kind].includes(ext)) ?? "";
    }

    keys: ["text/uri-list"]
    onDropped: event => {
        const files = event.urls.map(url => App.path(url)).filter(file => file);
        files.forEach(file => {
                const kind = layers ? kindOf(file) : "";
                if (kind)
                    page.run("add_layer", {
                            "kind": kind,
                            "file": file
                        });
                else
                    page.run("import", {
                            "file": file
                        });
            });
    }

    Rectangle {
        anchors.fill: parent
        visible: drop.containsDrag
        color: Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.2)
        border.color: Kirigami.Theme.highlightColor
        border.width: 2

        QQC2.Label {
            anchors.centerIn: parent
            text: drop.layers ? App.tr("Drop files to add them as layers") : App.tr("Drop files to import them")
        }
    }
}
