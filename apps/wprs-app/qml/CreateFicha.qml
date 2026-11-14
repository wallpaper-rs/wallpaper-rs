import QtQuick
import QtQuick.Layouts
import io.github.wallpaper_rs.app

Item {
    id: ficha

    property string title
    property string caption
    property string tone
    property string icon
    property bool fields
    property alias middle: mid.data
    default property alias actions: row.data

    implicitHeight: Math.max(words.implicitHeight, row.implicitHeight) + Look.e4
    Accessible.role: Accessible.Grouping
    Accessible.name: title

    Rectangle {
        width: parent.width
        height: Look.linea
        color: Look.nocheLinea
    }

    Column {
        id: words

        y: Look.e4 + (parent.height - Look.e4 - height) / 2
        width: ficha.fields ? implicitWidth : parent.width - row.width - Look.e5

        Text {
            width: parent.width
            text: ficha.title
            elide: Text.ElideRight
            font.family: Look.letraTitulo
            font.pixelSize: Look.tFoco
            font.weight: Font.Bold
            font.letterSpacing: Look.tFoco * Look.apretado
            color: Look.escarcha
        }

        Nota {
            visible: ficha.caption !== "" && !ficha.fields
            text: ficha.caption
            tone: ficha.tone
            icon: ficha.icon
        }
    }

    Row {
        id: mid

        visible: ficha.fields
        x: words.x + words.width + Look.e5
        y: Look.e4 + (parent.height - Look.e4 - height) / 2
        spacing: Look.e4
    }

    RowLayout {
        id: row

        x: parent.width - width
        y: Look.e4 + (parent.height - Look.e4 - height) / 2
        spacing: Look.e3
    }
}
