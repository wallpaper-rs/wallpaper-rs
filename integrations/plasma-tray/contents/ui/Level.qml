import QtQuick
import QtQuick.Templates as T
import "../look"

T.Slider {
    id: level

    implicitWidth: Look.rielAncho
    implicitHeight: Look.altoOpcion
    focusPolicy: Qt.StrongFocus

    background: Rectangle {
        x: level.leftPadding
        y: level.topPadding + (level.availableHeight - height) / 2
        width: level.availableWidth
        height: Look.rielAlto
        radius: height / 2
        color: Look.nocheLinea

        Rectangle {
            width: level.visualPosition * parent.width
            height: parent.height
            radius: height / 2
            color: Look.escarcha
        }
    }

    handle: Rectangle {
        x: level.leftPadding + level.visualPosition * (level.availableWidth - width)
        y: level.topPadding + (level.availableHeight - height) / 2
        width: Look.rielPunto
        height: Look.rielPunto
        radius: width / 2
        color: Look.escarcha

        Ring {
            visible: level.visualFocus
            corner: width / 2
        }
    }
}
