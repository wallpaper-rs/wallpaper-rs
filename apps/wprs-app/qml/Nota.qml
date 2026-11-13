import QtQuick
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

Row {
    id: nota

    property string text
    property string tone
    property string icon: Tono.icon(tone)
    property bool lit: false

    spacing: Look.listaHueco
    Accessible.role: Accessible.StaticText
    Accessible.name: text

    IconImage {
        anchors.verticalCenter: parent.verticalCenter
        visible: nota.icon !== ""
        name: nota.icon
        sourceSize: Qt.size(Look.iconoNota, Look.iconoNota)
        color: Tono.of(nota.tone, nota.lit, nota.lit ? Look.noche : Look.bruma)
    }

    Text {
        anchors.verticalCenter: parent.verticalCenter
        text: nota.text
        font.family: Look.letraTexto
        font.pixelSize: Look.tPie
        color: nota.lit ? Look.noche : Look.bruma
    }
}
