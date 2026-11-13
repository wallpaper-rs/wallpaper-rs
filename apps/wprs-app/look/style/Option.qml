import QtQuick
import io.github.wallpaper_rs.app

Rectangle {
    property bool lit: false
    property bool over: false
    property bool down: false

    implicitWidth: 200
    implicitHeight: Look.altoOpcion
    radius: Look.radioChico
    visible: lit || over || down
    color: lit ? (down ? Look.bruma : Look.escarcha) : down ? Look.noche : Look.nocheAlta
}
