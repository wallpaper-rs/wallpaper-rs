import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: part

    required property var page
    readonly property var options: App.state.options ?? {}

    function flag(key, on) {
        App.act("SetOption", [key, String(on)]);
    }

    implicitHeight: filas.height

    Filas {
        id: filas

        Fila {
            text: App.tr("Let scenes play their sounds")
            kind: "llave"
            checked: part.options["playback.scene_sound"] !== false
            value: checked ? App.tr("Yes") : App.tr("No")
            onToggled: part.flag("playback.scene_sound", checked)
        }

        Fila {
            text: App.tr("Let scenes react to the sound that plays")
            kind: "llave"
            checked: part.options["playback.scene_capture"] !== false
            value: checked ? App.tr("Yes") : App.tr("No")
            explain: App.tr("Each scene still asks you first. The sound is turned into levels on this computer and never recorded, kept or sent.")
            onToggled: part.flag("playback.scene_capture", checked)
        }

        Fila {
            text: App.tr("Silence wallpapers while another application plays sound")
            kind: "llave"
            checked: part.options["playback.duck"] === true
            value: checked ? App.tr("Yes") : App.tr("No")
            onToggled: part.flag("playback.duck", checked)
        }
    }
}
