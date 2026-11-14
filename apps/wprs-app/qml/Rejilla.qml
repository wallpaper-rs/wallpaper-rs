import QtQuick
import io.github.wallpaper_rs.app

Coleccion {
    id: rejilla

    property alias currentIndex: grid.currentIndex
    readonly property real room: grid.width - grid.leftMargin - grid.rightMargin
    readonly property int columns: Math.max(1, Math.floor(room / (Look.tapaRejilla + Look.huecoEstante)))
    readonly property int cell: Math.floor(room / columns)
    readonly property real cover: cell - Look.huecoEstante

    view: grid
    Accessible.role: Accessible.List
    Accessible.name: label

    FontMetrics {
        id: names

        font.family: Look.letraTitulo
        font.pixelSize: Look.tNombre
    }

    FontMetrics {
        id: notes

        font.family: Look.letraTexto
        font.pixelSize: Look.tPie
    }

    GridView {
        id: grid

        anchors.fill: parent
        anchors.rightMargin: -Look.huecoEstante
        topMargin: Look.filete + Look.anillo
        leftMargin: Look.filete + Look.anillo
        rightMargin: Look.filete + Look.anillo
        bottomMargin: Look.e4
        cellWidth: rejilla.cell
        cellHeight: Math.round(rejilla.cover * 9 / 16) + Look.e3 + Math.ceil(names.height) + Look.notaArriba + Math.max(Look.icono, Math.ceil(notes.height)) + Look.e5
        focus: true
        clip: true
        reuseItems: true
        cacheBuffer: cellHeight
        boundsBehavior: Flickable.StopAtBounds
        highlightFollowsCurrentItem: true
        highlightMoveDuration: Look.paso
        highlight: Item {
        }
        model: rejilla.model
        Keys.onPressed: event => rejilla.press(event)

        delegate: TapaVista {
            width: rejilla.cover
            even: true
            from: rejilla
        }
    }
}
