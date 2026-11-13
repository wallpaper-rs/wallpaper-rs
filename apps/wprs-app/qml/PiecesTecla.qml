import QtQuick
import io.github.wallpaper_rs.app

Rectangle {
    property alias text: words.text

    implicitWidth: Math.max(Look.atajosAlto + Look.e1, words.implicitWidth + 2 * Look.e2)
    implicitHeight: Look.atajosAlto
    radius: Look.radioChico
    color: Look.nocheAlta
    border.width: Look.linea
    border.color: Look.nocheLinea

    Text {
        id: words

        anchors.centerIn: parent
        font.family: Look.letraTexto
        font.pixelSize: Look.tTecla
        font.weight: Font.DemiBold
        color: Look.escarcha
    }
}
