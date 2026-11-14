import QtQuick
import io.github.wallpaper_rs.app

Row {
    id: thumbs

    property string source
    property bool stale: false

    spacing: Look.e2
    Component.onCompleted: {
        model.search("source:" + source + " missing:no", "", false, "");
        model.visible(0, Look.fuenteFotos - 1);
    }

    ItemModel {
        id: model

        size: 256
        onReloaded: model.visible(0, Look.fuenteFotos - 1)
    }

    Repeater {
        model: model

        delegate: Item {
            required property int index
            required property string thumb

            visible: index < Look.fuenteFotos
            width: visible ? height * 16 / 9 : 0
            height: thumbs.height

            Rectangle {
                anchors.fill: parent
                radius: Look.radioChico
                color: Look.nocheAlta
                clip: true

                Image {
                    anchors.fill: parent
                    source: parent.parent.thumb
                    asynchronous: true
                    fillMode: Image.PreserveAspectCrop
                }

                Rectangle {
                    anchors.fill: parent
                    color: Look.noche
                    opacity: thumbs.stale ? Look.fuenteVelo * 2 : Look.fuenteVelo
                }
            }
        }
    }
}
