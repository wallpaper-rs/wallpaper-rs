import QtQuick
import QtQuick.Layouts
import io.github.wallpaper_rs.app

Column {
    id: ficha

    property string title
    property string caption
    default property alias actions: row.data

    width: Math.min(implicitWidth, Look.tapaFoco + Look.tapa)
    Accessible.role: Accessible.Grouping
    Accessible.name: title

    Text {
        width: Math.min(implicitWidth, ficha.width)
        text: ficha.title
        elide: Text.ElideRight
        font.family: Look.letraTitulo
        font.pixelSize: Look.tFoco
        font.weight: Font.Bold
        font.letterSpacing: Look.tFoco * Look.apretado
        color: Look.escarcha
    }

    Item {
        width: 1
        height: Look.pieArriba
    }

    Text {
        visible: ficha.caption !== ""
        width: Math.min(implicitWidth, ficha.width)
        text: ficha.caption
        elide: Text.ElideRight
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
        color: Look.bruma
    }

    Item {
        width: 1
        height: Look.accionesArriba
        visible: row.children.length > 0
    }

    RowLayout {
        id: row

        spacing: Look.e3
    }
}
