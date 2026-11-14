import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

ColumnLayout {
    id: error

    property string icon: "warning-circle"
    property string title
    property string text
    property string more
    property string details
    default property alias actions: row.data

    Layout.fillWidth: true
    spacing: Look.e2
    Accessible.role: Accessible.AlertMessage
    Accessible.name: title
    Accessible.description: text

    IconImage {
        name: error.icon
        sourceSize: Qt.size(Look.iconoError, Look.iconoError)
        color: Look.coral
    }

    Text {
        Layout.fillWidth: true
        text: error.title
        wrapMode: Text.WordWrap
        font.family: Look.letraTitulo
        font.pixelSize: Look.tAparte
        font.weight: Font.Bold
        color: Look.escarcha
    }

    Text {
        Layout.fillWidth: true
        visible: error.text !== ""
        text: error.text
        wrapMode: Text.WordWrap
        lineHeight: 1.3
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
        color: Look.bruma
    }

    Text {
        Layout.fillWidth: true
        visible: error.more !== ""
        text: error.more
        wrapMode: Text.WordWrap
        lineHeight: 1.3
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
        color: Look.bruma
    }

    RowLayout {
        id: row

        Layout.topMargin: Look.e2
        spacing: Look.e3
    }

    Detalles {
        Layout.fillWidth: true
        visible: error.details !== ""
        details: error.details
    }
}
