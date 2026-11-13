import QtQuick
import QtQuick.Layouts
import io.github.wallpaper_rs.app

Item {
    id: fija

    property string title
    property string caption
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

        visible: fija.title !== ""
        y: Look.e4 + (parent.height - Look.e4 - height) / 2
        width: parent.width - row.width - Look.e5

        Text {
            width: parent.width
            text: fija.title
            elide: Text.ElideRight
            font.family: Look.letraTitulo
            font.pixelSize: Look.tFoco
            font.weight: Font.Bold
            font.letterSpacing: Look.tFoco * Look.apretado
            color: Look.escarcha
        }

        Text {
            visible: fija.caption !== ""
            width: parent.width
            text: fija.caption
            elide: Text.ElideRight
            font.family: Look.letraTexto
            font.pixelSize: Look.tCuerpo
            color: Look.bruma
        }
    }

    RowLayout {
        id: row

        x: fija.title !== "" ? parent.width - width : 0
        y: Look.e4 + (parent.height - Look.e4 - height) / 2
        spacing: Look.e3
    }
}
