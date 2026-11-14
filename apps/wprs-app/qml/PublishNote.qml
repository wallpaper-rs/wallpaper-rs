import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

RowLayout {
    id: note

    property string text
    property string tone
    property string icon: Tono.icon(tone)

    spacing: Look.listaHueco
    Accessible.role: Accessible.StaticText
    Accessible.name: text

    IconImage {
        Layout.alignment: Qt.AlignTop
        Layout.topMargin: (Look.tPie * 1.3 - Look.iconoNota) / 2
        visible: note.icon !== ""
        name: note.icon
        sourceSize: Qt.size(Look.iconoNota, Look.iconoNota)
        color: Tono.of(note.tone, false, Look.bruma)
    }

    Text {
        Layout.fillWidth: true
        text: note.text
        wrapMode: Text.WordWrap
        font.family: Look.letraTexto
        font.pixelSize: Look.tPie
        color: note.tone === "" ? Look.bruma : Tono.of(note.tone, false, Look.bruma)
    }
}
