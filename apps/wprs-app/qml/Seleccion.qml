import QtQuick
import QtQuick.Layouts
import io.github.wallpaper_rs.app

RowLayout {
    id: seleccion

    property int count
    default property alias actions: tail.data

    spacing: Look.e3
    Accessible.role: Accessible.Grouping
    Accessible.name: chosen.text

    Text {
        id: chosen

        rightPadding: Look.e2
        text: App.trc("covers", "%1 chosen", seleccion.count)
        font.family: Look.letraTitulo
        font.pixelSize: Look.tFoco
        font.weight: Font.Bold
        font.letterSpacing: Look.tFoco * Look.apretado
        color: Look.escarcha
    }

    RowLayout {
        id: tail

        spacing: Look.e3
    }
}
