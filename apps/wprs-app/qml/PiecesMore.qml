import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: part

    readonly property alias shortcut: shortcut
    readonly property alias named: named
    readonly property alias battery: battery

    function enter() {
        shortcut.forceActiveFocus(Qt.TabFocusReason);
    }

    Filas {
        width: Look.filasAncho

        Fila {
            text: qsTr("Shortcut")
            kind: "captura"
        }

        Fila {
            id: shortcut

            text: qsTr("Shortcut")
            kind: "captura"
            keys: ["Meta", "Alt", "P"]
            explain: qsTr("Press Enter, then the keys you want.")
        }

        Fila {
            id: named

            text: qsTr("Name")
            kind: "campo"
            placeholder: qsTr("No name")
        }

        Fila {
            text: qsTr("Advanced")
            kind: "pliega"
            value: qsTr("Open")
        }

        Fila {
            id: battery

            text: qsTr("Pause on battery")
            value: qsTr("not detected")
            off: true
            explain: qsTr("This computer has no battery, so there is nothing to save here.")
        }

        Fila {
            text: qsTr("Folder")
            kind: "texto"
            note: qsTr("Not found")
            noteTone: "coral"
            value: qsTr("Videos")
            status: qsTr("Could not set it: the file is missing")
            statusTone: "coral"
            explain: qsTr("Where the wallpapers of this list come from.")
        }
    }
}
