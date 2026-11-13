import QtQuick
import io.github.wallpaper_rs.app

Rectangle {
    id: field

    property bool lit: false
    property bool over: false
    property bool live: true

    implicitWidth: 200
    implicitHeight: Look.altoControl
    radius: Look.radioChico
    color: live ? Look.nocheAlta : "transparent"
    border.width: Look.linea
    border.color: live && over && !lit ? Look.brumaBaja : Look.nocheLinea

    FocusRing {
        corner: field.radius
        visible: field.lit
    }
}
