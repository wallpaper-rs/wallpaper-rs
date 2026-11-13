import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: part

    readonly property alias quality: quality
    readonly property alias folders: folders
    readonly property alias list: list

    function enter() {
        quality.forceActiveFocus(Qt.TabFocusReason);
    }

    Filas {
        width: Look.filasAncho

        Seccion {
            text: App.tr("Performance")
        }

        Fila {
            id: quality

            text: App.tr("Quality")
            kind: "elige"
            options: [App.tr("Low"), App.tr("Medium"), App.tr("High")]
            current: 2
            explain: App.tr("How much detail each scene draws. Lower uses less battery.")
        }

        Fila {
            text: App.tr("Frame rate")
            kind: "riel"
            from: 0
            to: 60
            stepSize: 5
            at: 30
            number: String(Math.round(at))
        }

        Fila {
            text: App.tr("Pause when a window covers the screen")
            kind: "llave"
            checked: true
            value: checked ? App.tr("On") : App.tr("Off")
        }

        Fila {
            text: App.tr("Pause on battery")
            note: App.tr("This computer has no battery")
            value: App.tr("not detected")
            off: true
            explain: App.tr("This computer has no battery, so there is nothing to save here.")
        }

        Fila {
            text: App.tr("Graphics card")
            kind: "pliega"
            value: App.tr("Automatic")
        }

        Seccion {
            text: App.tr("Library")
        }

        Fila {
            id: folders

            text: App.tr("Library folders")
            kind: "despliega"
            open: true
            number: String(list.model.length)
        }

        Sublista {
            id: list

            width: parent.width
            visible: folders.open
            addText: App.tr("Add a folder…")
            model: [
                {
                    "name": App.tr("Wallpapers"),
                    "where": App.tr("Pictures")
                },
                {
                    "name": App.tr("Animated wallpapers"),
                    "where": App.tr("Videos"),
                    "problem": App.tr("Not found")
                }
            ]
            onRemoved: index => model = model.filter((entry, at) => at !== index)
        }
    }
}
