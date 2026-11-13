import QtQuick
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

Rectangle {
    id: step

    property string name
    property bool held: false
    property bool over: false
    property bool live: true

    implicitWidth: Look.altoHerr
    implicitHeight: Look.altoControl
    radius: Look.radioChico
    color: !live ? "transparent" : held ? Look.noche : over ? Look.nocheLinea : "transparent"

    IconImage {
        anchors.centerIn: parent
        width: Look.icono
        height: Look.icono
        sourceSize: Qt.size(Look.icono, Look.icono)
        name: step.name
        color: !step.live ? Look.brumaBaja : step.over ? Look.escarcha : Look.bruma
    }
}
