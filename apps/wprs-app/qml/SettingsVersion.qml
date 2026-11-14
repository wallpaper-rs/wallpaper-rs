import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: part

    required property var page
    readonly property string problem: [App.state.config_error, App.state.library_error].filter(text => !!text).join("\n")

    implicitHeight: filas.height

    Filas {
        id: filas

        Fila {
            text: App.tr("Version")
            value: App.state.version ?? ""
        }

        Fila {
            text: App.tr("Graphics")
            value: App.state.adapter ?? ""
            visible: value !== ""
        }

        Fila {
            text: App.tr("Data model")
            value: String(App.state.model ?? "")
        }

        Fila {
            text: App.tr("Configuration")
            value: part.problem === "" ? App.tr("No problems") : App.tr("Has a problem")
            explain: part.problem
            status: part.problem === "" ? "" : App.tr("Fix the file and use “Reload the settings”.")
            statusTone: "coral"
        }
    }
}
