import QtQuick
import io.github.wallpaper_rs.app

Row {
    property string text
    property string small

    spacing: Look.e2
    Accessible.ignored: true

    Text {
        id: main

        text: parent.text
        font.family: Look.letraTitulo
        font.pixelSize: Look.tNombre
        font.weight: Font.DemiBold
        color: Look.escarcha
    }

    Text {
        anchors.baseline: main.baseline
        text: parent.small
        font.family: Look.letraTexto
        font.pixelSize: Look.tPie
        color: Look.bruma
    }
}
