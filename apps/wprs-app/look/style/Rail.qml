import QtQuick
import io.github.wallpaper_rs.app

Rectangle {
    id: rail

    property real from: 0
    property real to: 1
    property bool upright: false
    property bool live: true

    implicitWidth: upright ? Look.rielAlto : 120
    implicitHeight: upright ? 120 : Look.rielAlto
    radius: Look.rielAlto / 2
    color: Look.nocheLinea

    Rectangle {
        x: rail.upright ? 0 : rail.from * rail.width
        y: rail.upright ? (1 - rail.to) * rail.height : 0
        width: rail.upright ? rail.width : (rail.to - rail.from) * rail.width
        height: rail.upright ? (rail.to - rail.from) * rail.height : rail.height
        radius: rail.radius
        color: rail.live ? Look.escarcha : Look.brumaBaja
    }
}
