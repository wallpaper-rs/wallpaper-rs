import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

RowLayout {
    id: thumb

    required property var publish
    readonly property string preview: Publishing.thumbnail(publish.manifest)
    readonly property bool outdated: publish.editing && publish.shots >= 0 && packer.outdated(publish.page.editor.folder, preview)

    function refresh() {
        publish.reload();
        publish.shots++;
    }

    visible: outdated
    spacing: Look.e3

    Packer {
        id: packer
    }

    Connections {
        target: thumb.publish.editing ? thumb.publish.page.editor : null

        function onBusyChanged() {
            if (!thumb.publish.page.editor.busy)
                Qt.callLater(() => thumb && thumb.refresh());
        }
    }

    Text {
        Layout.fillWidth: true
        text: thumb.preview === "" ? App.tr("It has no thumbnail: the library draws one by itself, but a server does not take it without one.") : App.tr("The thumbnail is older than the last change.")
        wrapMode: Text.WordWrap
        font.family: Look.letraTexto
        font.pixelSize: Look.tPie
        color: Look.bruma
    }

    Loader {
        active: thumb.publish.editing
        sourceComponent: PackageCapture {
            host: ({
                    "field": {
                        "widget": ""
                    },
                    "page": thumb.publish.page
                })
            display: QQC2.AbstractButton.TextBesideIcon
            text: App.tr("Update the thumbnail")
        }
    }
}
