import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

FocusScope {
    id: page

    required property var sheet
    readonly property real bandLow: bar.height + lower.spacing + shelf.implicitHeight
    readonly property real bandHigh: bandLow
    readonly property real upperHeight: upperBlock.implicitHeight
    readonly property real bandFrom: Math.min(upper.y, band.y)
    property rect band
    property rect upper
    property alias upperBlock: upperBlock
    property alias filter: filter
    property alias search: search
    property alias shelf: shelf

    function layout() {
        bar.forceLayout();
        lower.forceLayout();
        upperBlock.forceLayout();
    }

    Column {
        id: upperBlock

        x: page.upper.x + Math.max(0, page.upper.width - width)
        y: page.upper.y
        spacing: Look.e3

        Text {
            text: page.sheet.say("Keep going with yours", "Seguir con lo tuyo")
            color: Look.escarcha
            font.family: Look.letraTitulo
            font.pixelSize: Look.tFila
            font.weight: Font.Bold
        }

        Row {
            spacing: Look.e4

            Repeater {
                model: [["dusk", "Hills at dusk", "Colinas al atardecer"], ["orbits", "Orbits", "Órbitas"]]

                Tapa {
                    required property var modelData

                    width: Look.tapaRejilla
                    picture: page.sheet.pic(modelData[0])
                    name: page.sheet.say(modelData[1], modelData[2])
                }
            }
        }
    }

    Column {
        id: lower

        x: page.band.x
        y: page.band.y
        width: page.band.width
        spacing: Look.e4

        Row {
            id: bar

            spacing: Look.e5

            Filtro {
                id: filter

                anchors.verticalCenter: parent.verticalCenter
                label: page.sheet.say("Kind", "Tipo")
                words: [{
                        "text": page.sheet.say("All", "Todos"),
                        "count": 6
                    }, {
                        "text": page.sheet.say("Scenes", "Escenas"),
                        "count": 5
                    }, {
                        "text": page.sheet.say("Pictures", "Imágenes"),
                        "count": 1
                    }]
                onPicked: index => current = index
            }

            Buscar {
                id: search

                placeholder: page.sheet.say("Search your library", "Buscar en tu biblioteca")
            }
        }

        Estante {
            id: shelf

            width: parent.width + Look.margen
            label: page.sheet.say("Your library", "Tu biblioteca")
            currentIndex: 1
            model: [page.sheet.cover("snowfall", "Snowfall", "Nevada", {
                        "inUse": true
                    }), page.sheet.cover("pond", "Pond", "Estanque", {
                        "caption": page.sheet.say("Scene · Examples", "Escena · Ejemplos")
                    }), page.sheet.cover("dusk", "Hills at dusk", "Colinas al atardecer"), page.sheet.cover("orbits", "Orbits", "Órbitas"), page.sheet.cover("pulse", "Pulse", "Pulso"), page.sheet.cover("solids", "Solids", "Sólidos")]
            actions: Row {
                spacing: Look.e3

                QQC2.Button {
                    highlighted: true
                    icon.name: "image"
                    text: page.sheet.say("Set as wallpaper", "Poner de fondo")
                }

                PiecesTecla {
                    anchors.verticalCenter: parent.verticalCenter
                    text: App.trc("key", "Enter")
                }
            }
        }
    }
}
