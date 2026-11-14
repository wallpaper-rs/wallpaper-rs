import QtQuick
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

Item {
    id: dibujo

    property string icon

    clip: true

    Rectangle {
        anchors.fill: parent
        radius: Look.radio

        gradient: Gradient {
            GradientStop {
                position: 0
                color: Look.nocheLinea
            }

            GradientStop {
                position: 0.55
                color: Look.nocheAlta
            }

            GradientStop {
                position: 1
                color: Look.noche
            }
        }
    }

    Repeater {
        model: 3

        Rectangle {
            required property int index
            readonly property real side: dibujo.height * (0.7 + index * 0.45)

            x: dibujo.width * 0.78 - side / 2
            y: dibujo.height * 0.22 - side / 2
            width: side
            height: side
            radius: side / 2
            color: "transparent"
            border.width: Look.filete
            border.color: index === 0 ? Look.lunaHonda : Look.nocheLinea
        }
    }

    IconImage {
        anchors.centerIn: parent
        name: dibujo.icon
        sourceSize: Qt.size(Look.icono * 2, Look.icono * 2)
        color: Look.luna
    }
}
