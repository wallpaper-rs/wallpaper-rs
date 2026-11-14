import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: field

    required property var page
    readonly property string key: "surfaces.layer_namespace"
    readonly property var options: App.state.options ?? {}
    readonly property string chosen: String(options[key] ?? "auto")
    readonly property string own: chosen === "auto" ? "" : chosen
    readonly property var shell: App.state.layer_shell ?? {}
    readonly property bool software: String(options["render.gpu"] ?? "auto") === "software"
    readonly property var sources: ({
            "option": App.tr("the option the daemon was started with"),
            "environment": App.tr("the WPRS_LAYER_NAMESPACE variable"),
            "config": App.tr("this setting"),
            "automatic": App.tr("the automatic choice for this desktop"),
            "default": App.tr("the usual name")
        })

    function apply(text) {
        App.act("SetOption", [key, text.replace(/[^A-Za-z0-9._-]/g, "").slice(0, 64) || "auto"]);
    }

    implicitHeight: filas.height

    Filas {
        id: filas

        Fila {
            text: App.tr("Surface name")
            kind: "campo"
            fieldText: field.own
            placeholder: App.tr("Automatic")
            explain: App.tr("The name the wallpaper surfaces give the compositor. Change it if the overview of your compositor shows the wallpaper black: some only show it there under a name they know. Empty chooses automatically. It takes effect the next time the daemon starts, and only when the wallpaper is shown with layer-shell surfaces.")
            status: [field.shell.namespace ? App.tr("In use: %1, from %2.", field.shell.namespace, field.sources[field.shell.from] ?? field.shell.from ?? "") : "", field.shell.restart === true ? App.tr("The new name takes effect the next time the daemon starts.") : "", field.shell.ignored ? App.tr("A name given when the daemon started was refused, so the next source was used: %1", field.shell.ignored) : ""].filter(t => t).join(" ")
            statusTone: field.shell.restart === true || field.shell.ignored ? "ambar" : ""
            onEdited: text => field.apply(text)
        }

        Fila {
            text: App.tr("Draw with the processor")
            note: App.tr("Instead of a graphics card, slow")
            kind: "llave"
            checked: field.software
            value: checked ? App.tr("Yes") : App.tr("No")
            explain: App.tr("Drawn by the processor instead of a graphics card, for tests")
            onToggled: App.act("SetOption", ["render.gpu", checked ? "software" : "auto"])
        }

        Fila {
            text: App.tr("Depth drawn by the processor")
            kind: "llave"
            checked: field.options["playback.software_parallax"] === true
            value: checked ? App.tr("Yes") : App.tr("No")
            explain: App.tr("Depth parallax with software rendering")
            onToggled: App.act("SetOption", ["playback.software_parallax", String(checked)])
        }

        Fila {
            text: App.tr("Restart the daemon now")
            note: App.tr("A change waits for the next start")
            visible: App.state.gpu_restart === true || field.shell.restart === true
            kind: "pliega"
            onOpened: App.act("Restart", [], "Daemon1")
        }
    }
}
