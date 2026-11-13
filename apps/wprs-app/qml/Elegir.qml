import QtQuick
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

Rectangle {
    id: elegir

    property bool chosen

    implicitWidth: Look.elegirLado
    implicitHeight: Look.elegirLado
    radius: width / 2
    border.width: Look.anillo
    border.color: Look.escarcha
    color: chosen ? Look.escarcha : Qt.rgba(Look.noche.r, Look.noche.g, Look.noche.b, Look.veloElegir)
    Accessible.role: Accessible.CheckBox
    Accessible.checkable: true
    Accessible.checked: chosen
    Accessible.name: App.tr("Chosen")

    IconImage {
        visible: elegir.chosen
        anchors.centerIn: parent
        width: Look.icono
        height: Look.icono
        sourceSize.width: width
        sourceSize.height: height
        name: "check-bold"
        color: Look.noche
    }
}
