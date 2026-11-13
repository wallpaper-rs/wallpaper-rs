import QtQuick
import io.github.wallpaper_rs.app

Row {
    id: hints

    property var model: []
    readonly property string said: model.map(hint => hint.keys.join(" ") + " " + hint.text).join(", ")

    spacing: Look.e2
    Accessible.role: Accessible.StaticText
    Accessible.name: said

    Repeater {
        model: hints.model

        delegate: Row {
            id: hint

            required property var modelData
            required property int index

            spacing: Look.e2

            Row {
                anchors.verticalCenter: parent.verticalCenter
                spacing: Look.e1

                Repeater {
                    model: hint.modelData.keys

                    delegate: KeyCap {
                        required property string modelData

                        key: modelData
                    }
                }
            }

            Text {
                anchors.verticalCenter: parent.verticalCenter
                rightPadding: hint.index < hints.model.length - 1 ? Look.e4 - Look.e2 : 0
                text: hint.modelData.text
                color: Look.bruma
                font.family: Look.letraTexto
                font.pixelSize: Look.tPie
            }
        }
    }
}
