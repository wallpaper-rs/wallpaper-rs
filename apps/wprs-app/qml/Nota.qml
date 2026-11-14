import QtQuick
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

Row {
    id: nota

    property string text
    property string tone
    property string icon: Tono.icon(tone)
    property bool lit: false
    property real fitWidth: 0
    property bool wrap: false
    property bool wraps: false

    spacing: Look.listaHueco
    Accessible.role: Accessible.StaticText
    Accessible.name: text

    IconImage {
        id: mark

        anchors.verticalCenter: parent.verticalCenter
        visible: nota.icon !== ""
        name: nota.icon
        sourceSize: Qt.size(Look.iconoNota, Look.iconoNota)
        color: Tono.of(nota.tone, nota.lit, nota.lit ? Look.noche : Look.bruma)
    }

    Text {
        anchors.verticalCenter: parent.verticalCenter
        width: nota.wraps ? nota.width - (nota.icon !== "" ? Look.iconoNota + nota.spacing : 0) : nota.fitWidth > 0 ? Math.min(implicitWidth, nota.fitWidth - (mark.visible ? mark.width + nota.spacing : 0)) : implicitWidth
        text: nota.text
        wrapMode: nota.wrap || nota.wraps ? Text.Wrap : Text.NoWrap
        elide: nota.wrap || nota.wraps ? Text.ElideNone : Text.ElideRight
        font.family: Look.letraTexto
        font.pixelSize: Look.tPie
        color: nota.lit ? Look.noche : Look.bruma
    }
}
