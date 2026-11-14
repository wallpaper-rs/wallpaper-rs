import QtQuick
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

Row {
    id: estado

    property string text
    property string tone
    property string icon: Tono.icon(tone)
    property bool lit: false
    property real wrap: 0

    spacing: Look.huecoEstado
    Accessible.role: Accessible.StaticText
    Accessible.name: text

    IconImage {
        anchors.verticalCenter: parent.verticalCenter
        name: estado.icon
        sourceSize: Qt.size(Look.iconoEstado, Look.iconoEstado)
        color: Tono.of(estado.tone, estado.lit, estado.lit ? Look.noche : Look.bruma)
    }

    Text {
        anchors.verticalCenter: parent.verticalCenter
        width: estado.wrap > 0 ? Math.min(implicitWidth, estado.wrap - Look.iconoEstado - Look.huecoEstado) : implicitWidth
        text: estado.text
        wrapMode: Text.WordWrap
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
        font.weight: Font.Medium
        color: estado.lit ? Look.noche : Look.escarcha
    }
}
