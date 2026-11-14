import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: sessions

    required property var account
    property var items: []

    function load() {
        App.query("ListUploads", [], "Sync1", reply => sessions && (sessions.items = reply.uploads ?? []), () => sessions && (sessions.items = []));
    }

    visible: sessions.items.length > 0
    Component.onCompleted: load()

    Connections {
        target: App

        function onUpdated(topic) {
            if (["account", "jobs"].includes(topic))
                sessions.load();
        }
    }

    Seccion {
        text: App.tr("Uploads that did not finish")
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
        text: App.tr("An upload that stopped half way holds one of the two open sessions the server allows; cancel it to free the place.")
    }

    Repeater {
        model: sessions.items

        delegate: RowLayout {
            required property var modelData

            Layout.fillWidth: true

            QQC2.Label {
                Layout.fillWidth: true
                elide: Text.ElideRight
                text: modelData.state + (modelData.errors?.length ? " · " + modelData.errors.join("; ") : "")
            }

            QQC2.Button {
                text: App.tr("Cancel")
                icon.name: "dialog-cancel"
                visible: modelData.state === "open"
                onClicked: App.act("CancelUpload", [modelData.id], "Sync1", () => sessions && sessions.load())
            }
        }
    }
}
