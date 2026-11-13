import QtQuick
import io.github.wallpaper_rs.app

Rectangle {
    required property real corner

    anchors.fill: parent
    anchors.margins: -(Look.anilloHueco + Look.anillo)
    radius: corner + Look.anilloHueco + Look.anillo
    color: "transparent"
    border.width: Look.anillo
    border.color: Look.escarcha
}
