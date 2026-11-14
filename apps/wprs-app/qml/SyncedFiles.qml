import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Layouts
import io.github.wallpaper_rs.app

ColumnLayout {
    id: synced

    property var files: []
    readonly property var missing: files.filter(f => f.stored && !f.item)

    function load() {
        App.query("ListFiles", [], "Sync1", reply => synced && (synced.files = reply.files ?? []), () => synced && (synced.files = []));
    }

    visible: missing.length > 0
    spacing: 0
    Component.onCompleted: load()

    Connections {
        target: App

        function onUpdated(topic) {
            if (["account", "library", "jobs"].includes(topic))
                synced.load();
        }
    }

    Repeater {
        model: synced.missing

        ColumnLayout {
            required property var modelData

            Layout.fillWidth: true
            spacing: Look.e1

            Nota {
                Layout.fillWidth: true
                wraps: true
                text: App.tr("%1 (%2) was sent to your server from another computer and is not in this library yet.", modelData.name, App.bytes(modelData.size))
            }

            QQC2.Button {
                Layout.alignment: Qt.AlignRight
                text: App.tr("Download")
                icon.name: "download"
                onClicked: App.act("FetchFile", [modelData.id], "Sync1")
            }
        }
    }
}
