import QtQuick
import "../look"

Rectangle {
    property real corner: Look.radioChico

    anchors.fill: parent
    anchors.margins: -Look.anilloHueco
    color: "transparent"
    radius: corner + Look.anilloHueco
    border.width: Look.anillo
    border.color: Look.escarcha
}
