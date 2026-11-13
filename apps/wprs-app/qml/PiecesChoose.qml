import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Flickable {
    id: page

    readonly property bool es: Qt.locale().name.startsWith("es")
    readonly property var files: Qt.application.arguments.filter(a => /\.(jpg|png)$/.test(a))
    property alias filter: filter
    property alias search: search
    property alias shelf: shelf
    property alias grid: grid
    property alias chip: chip

    function say(en, es) {
        return page.es ? es : en;
    }

    function pic(name) {
        const found = files.find(f => f.indexOf("/" + name + "/") >= 0);
        return found ? "file://" + found : "";
    }

    function cover(name, en, es, extra) {
        return Object.assign({
                "picture": pic(name),
                "name": say(en, es)
            }, extra ?? {});
    }

    contentHeight: sheet.height + 2 * Look.margen
    clip: true

    Column {
        id: sheet

        x: Look.margen
        y: Look.margen
        width: page.width - 2 * Look.margen
        spacing: Look.e6

        Row {
            spacing: Look.e7

            Filtro {
                id: filter

                label: page.say("Kind", "Tipo")
                current: 0
                words: [{
                        "text": page.say("All", "Todos"),
                        "count": 11
                    }, {
                        "text": page.say("Scenes", "Escenas"),
                        "count": 6
                    }, {
                        "text": page.say("Videos", "Vídeos"),
                        "count": 2
                    }, {
                        "text": page.say("Pictures", "Imágenes"),
                        "count": 3
                    }]
                onPicked: index => current = index
            }

            Filtro {
                sections: true
                label: page.say("Sections", "Secciones")
                words: [{
                        "text": page.say("Wallpapers", "Fondos")
                    }, {
                        "text": page.say("Screens", "Pantallas")
                    }, {
                        "text": page.say("Computer", "Equipo")
                    }]
                onPicked: index => current = index
            }

            Buscar {
                id: search

                placeholder: page.say("Search your library", "Buscar en tu biblioteca")
            }

            Quitable {
                id: chip

                anchors.verticalCenter: parent.verticalCenter
                text: page.say("Mature", "Para adultos")
            }
        }

        Estante {
            id: shelf

            width: parent.width + Look.margen
            label: page.say("Your library", "Tu biblioteca")
            currentIndex: 1
            model: [page.cover("snowfall", "Snowfall", "Nevada", {
                        "inUse": true
                    }), page.cover("pond", "Pond", "Estanque", {
                        "caption": page.say("Scene · Examples · Follows the pointer", "Escena · Ejemplos · Sigue el puntero")
                    }), page.cover("orbits", "Orbits", "Órbitas", {
                        "note": page.say("Shows the time", "Muestra la hora"),
                        "noteIcon": "clock"
                    }), page.cover("lights", "Lights", "Luces", {
                        "off": true,
                        "note": page.say("Has flashes, turned off", "Tiene destellos, deshabilitado"),
                        "noteIcon": "warning",
                        "noteTone": "ambar"
                    }), page.cover("pulse", "Pulse", "Pulso"), page.cover("solids", "Solids", "Sólidos")]
            actions: Row {
                spacing: Look.e3

                QQC2.Button {
                    highlighted: true
                    icon.name: "image"
                    text: page.say("Set as wallpaper", "Poner de fondo")
                }

                PiecesTecla {
                    anchors.verticalCenter: parent.verticalCenter
                    text: App.trc("key", "Enter")
                }

                QQC2.Button {
                    flat: true
                    anchors.verticalCenter: parent.verticalCenter
                    text: page.say("Options", "Opciones")
                    icon.name: "caret-right"
                    LayoutMirroring.enabled: true
                }
            }
        }

        Rejilla {
            id: grid

            width: parent.width
            height: 2 * (Math.round(Look.tapaRejilla * 9 / 16) + 3 * Look.e5) + Look.e6
            label: page.say("All your wallpapers", "Todos tus fondos")
            choosing: true
            chosen: [1, 2, 4]
            currentIndex: 2
            model: [page.cover("snowfall", "Snowfall", "Nevada", {
                        "inUse": true
                    }), page.cover("dusk", "Hills at dusk", "Colinas al atardecer"), page.cover("pond", "Pond", "Estanque"), page.cover("orbits", "Orbits", "Órbitas"), page.cover("pulse", "Pulse", "Pulso"), page.cover("solids", "Solids", "Sólidos"), page.cover("lights", "Lights", "Luces")]
            onToggled: index => chosen = chosen.indexOf(index) >= 0 ? chosen.filter(i => i !== index) : chosen.concat([index])
        }

        FichaFija {
            width: parent.width

            Seleccion {
                count: grid.chosen.length

                QQC2.Button {
                    highlighted: true
                    icon.name: "playlist"
                    text: page.say("Add to playlist…", "Añadir a una lista…")
                }

                QQC2.Button {
                    text: page.say("Remove from the library", "Quitar de la biblioteca")
                }

                QQC2.Button {
                    flat: true
                    text: page.say("Stop choosing", "Dejar de elegir")
                }

                PiecesTecla {
                    text: App.trc("key", "Esc")
                }
            }
        }

        FichaFija {
            width: parent.width
            title: grid.currentName
            caption: page.say("Scene · Examples", "Escena · Ejemplos")

            QQC2.Button {
                highlighted: true
                icon.name: "image"
                text: page.say("Set as wallpaper", "Poner de fondo")
            }
        }
    }
}
