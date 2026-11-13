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
            text: qsTr("Performance")
        }

        Fila {
            id: quality

            text: qsTr("Quality")
            kind: "elige"
            options: [qsTr("Low"), qsTr("Medium"), qsTr("High")]
            current: 2
            explain: qsTr("How much detail each scene draws. Lower uses less battery.")
        }

        Fila {
            text: qsTr("Frame rate")
            kind: "riel"
            from: 0
            to: 60
            stepSize: 5
            at: 30
            number: String(Math.round(at))
        }

        Fila {
            text: qsTr("Pause when a window covers the screen")
            kind: "llave"
            checked: true
            value: checked ? qsTr("On") : qsTr("Off")
        }

        Fila {
            text: qsTr("Pause on battery")
            note: qsTr("This computer has no battery")
            value: qsTr("not detected")
            off: true
            explain: qsTr("This computer has no battery, so there is nothing to save here.")
        }

        Fila {
            text: qsTr("Graphics card")
            kind: "pliega"
            value: qsTr("Automatic")
        }

        Seccion {
            text: qsTr("Library")
        }

        Fila {
            id: folders

            text: qsTr("Library folders")
            kind: "despliega"
            open: true
            number: String(list.model.length)
        }

        Sublista {
            id: list

            width: parent.width
            visible: folders.open
            addText: qsTr("Add a folder…")
            model: [
                {
                    "name": qsTr("Wallpapers"),
                    "where": qsTr("Pictures")
                },
                {
                    "name": qsTr("Animated wallpapers"),
                    "where": qsTr("Videos"),
                    "problem": qsTr("Not found")
                }
            ]
            onRemoved: index => model = model.filter((entry, at) => at !== index)
        }
    }
}
