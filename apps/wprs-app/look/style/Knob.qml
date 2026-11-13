import QtQuick
import io.github.wallpaper_rs.app

Rectangle {
    id: knob

    property bool live: true
    property bool lit: false
    property bool held: false

    implicitWidth: Look.rielPunto
    implicitHeight: Look.rielPunto
    radius: width / 2
    color: live ? (held ? Look.bruma : Look.escarcha) : Look.brumaBaja
    border.width: live ? 0 : Look.linea
    border.color: Look.nocheLinea

    FocusRing {
        corner: knob.radius
        visible: knob.lit
    }
}
