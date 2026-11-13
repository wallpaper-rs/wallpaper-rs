import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

RowLayout {
    id: pide

    property string icon
    property string text
    property string reason

    spacing: Look.e3
    Accessible.role: Accessible.StaticText
    Accessible.name: text
    Accessible.description: reason

    IconImage {
        Layout.alignment: Qt.AlignTop
        Layout.topMargin: 1
        name: pide.icon
        sourceSize: Qt.size(Look.iconoAviso, Look.iconoAviso)
        color: Look.bruma
    }

    Column {
        Layout.fillWidth: true

        Text {
            width: parent.width
            text: pide.text
            wrapMode: Text.WordWrap
            font.family: Look.letraTexto
            font.pixelSize: Look.tCuerpo
            font.weight: Font.DemiBold
            color: Look.escarcha
        }

        Text {
            width: parent.width
            text: pide.reason
            wrapMode: Text.WordWrap
            font.family: Look.letraTexto
            font.pixelSize: Look.tPie
            color: Look.bruma
        }
    }
}
