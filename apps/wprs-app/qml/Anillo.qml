import QtQuick
import io.github.wallpaper_rs.app

Rectangle {
    property real corner: Look.radio
    property int gap: Look.anilloHueco

    anchors.fill: parent
    anchors.margins: -(gap + Look.anillo)
    radius: corner + gap + Look.anillo
    color: "transparent"
    border.width: Look.anillo
    border.color: Look.escarcha
}
