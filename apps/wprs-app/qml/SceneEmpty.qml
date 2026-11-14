import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Item {
    id: empty

    required property var page
    readonly property var owner: ({
            "page": page,
            "current": null,
            "icon": kind => kind
        })

    function start(kind) {
        choice.kind = kind;
        if (kind === "clock") {
            starts.addClock(starts.formats[0]);
        } else if (kind === "image" && choice.assets.length === 0) {
            files.kind = "image";
            files.dialog.open();
        } else {
            choice.offer(kind);
        }
    }

    Rectangle {
        anchors.centerIn: parent
        width: Math.min(parent.width - 2 * Look.e5, 30 * Look.e4)
        height: card.implicitHeight + 2 * Look.e5
        radius: Look.radioPanel
        color: Look.nocheAlta
        border.width: Look.linea
        border.color: Look.nocheLinea

        ColumnLayout {
            id: card

            anchors.fill: parent
            anchors.margins: Look.e5
            spacing: Look.e3

            QQC2.Label {
                Layout.fillWidth: true
                text: App.tr("Add a layer")
                color: Look.escarcha
                font.pixelSize: Look.tSeccion
            }

            QQC2.Label {
                Layout.fillWidth: true
                wrapMode: Text.WordWrap
                text: App.tr("A scene is built with layers, one on top of the other. Start with the background picture.")
                color: Look.bruma
                font.pixelSize: Look.tCuerpo
            }

            GridLayout {
                Layout.fillWidth: true
                columns: 1
                columnSpacing: Look.e2
                rowSpacing: Look.e2

                Repeater {
                    model: [["image", "image", App.tr("Image"), App.tr("a photo or a drawing")], ["text", "text-t", App.tr("Text"), App.tr("fixed words")], ["clock", "clock", App.tr("Time or date"), App.tr("updates by itself")], ["particles", "snowflake", App.tr("Particles"), App.tr("snow, sparks, dust")]]

                    delegate: Opcion {
                        required property var modelData

                        Layout.fillWidth: true
                        icon: modelData[1]
                        text: modelData[2]
                        note: modelData[3]
                        onClicked: empty.start(modelData[0])
                    }
                }
            }

            QQC2.Label {
                Layout.fillWidth: true
                horizontalAlignment: Text.AlignHCenter
                text: App.tr("or drop a picture here")
                color: Look.brumaBaja
                font.pixelSize: Look.tPie
            }

            QQC2.Button {
                Layout.alignment: Qt.AlignHCenter
                flat: true
                text: App.tr("More layer kinds")
                icon.name: "caret-down"
                onClicked: {
                    empty.page.tabs.currentIndex = 0;
                    empty.page.layers.add();
                }
            }
        }
    }

    SceneLayerStarts {
        id: starts

        owner: empty.owner
    }

    SceneAssetChoice {
        id: choice

        owner: empty.owner
    }

    SceneLayerFiles {
        id: files

        owner: empty.owner
    }
}
