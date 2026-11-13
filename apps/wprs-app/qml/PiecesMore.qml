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
            text: App.tr("Shortcut")
            kind: "captura"
        }

        Fila {
            id: shortcut

            text: App.tr("Shortcut")
            kind: "captura"
            keys: ["Meta", "Alt", "P"]
            explain: App.tr("Press Enter, then the keys you want.")
        }

        Fila {
            id: named

            text: App.tr("Name")
            kind: "campo"
            placeholder: App.tr("No name")
        }

        Fila {
            text: App.tr("Advanced")
            kind: "pliega"
            value: App.tr("Open")
        }

        Fila {
            id: battery

            text: App.tr("Pause on battery")
            value: App.tr("not detected")
            off: true
            explain: App.tr("This computer has no battery, so there is nothing to save here.")
        }

        Fila {
            text: App.tr("Folder")
            kind: "texto"
            note: App.tr("Not found")
            noteTone: "coral"
            value: App.tr("Videos")
            status: App.tr("Could not set it: the file is missing")
            statusTone: "coral"
            explain: App.tr("Where the wallpapers of this list come from.")
        }
    }
}
