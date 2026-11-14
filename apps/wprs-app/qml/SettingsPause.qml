import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: part

    required property var page
    readonly property var options: App.state.options ?? {}
    readonly property bool battery: App.state.on_battery === true || App.state.battery_percent !== null && App.state.battery_percent !== undefined
    readonly property string chosen: String(options["pause.on_battery"] ?? "always")
    readonly property var presets: ["below:10", "below:15", "below:20", "below:30"]
    readonly property var values: ["never", "always"].concat(presets.includes(chosen) || !chosen.startsWith("below:") ? presets : presets.concat([chosen]))

    implicitHeight: filas.height

    Filas {
        id: filas

        Fila {
            text: App.tr("Pause when a full-screen window covers the wallpaper")
            kind: "llave"
            checked: part.options["pause.when_covered"] !== false
            value: checked ? App.tr("Yes") : App.tr("No")
            explain: App.tr("When you play or watch a film in full screen, the wallpaper stops drawing and uses nothing.")
            onToggled: App.act("SetOption", ["pause.when_covered", String(checked)])
        }

        Fila {
            text: App.tr("Pause on battery")
            note: part.battery ? "" : App.tr("no battery detected")
            off: !part.battery
            kind: "elige"
            options: part.values.map(value => value === "never" ? App.tr("Never") : value === "always" ? App.tr("Always") : App.tr("Below %1 %", value.slice(6)))
            current: Math.max(0, part.values.indexOf(part.chosen))
            explain: App.tr("Your choice is kept even when the battery is not seen; it applies again as soon as there is one.")
            onChosen: index => App.act("SetOption", ["pause.on_battery", part.values[index]])
        }

        Fila {
            text: App.tr("Pause with the screen locked")
            kind: "llave"
            checked: part.options["pause.when_locked"] !== false
            value: checked ? App.tr("Yes") : App.tr("No")
            onToggled: App.act("SetOption", ["pause.when_locked", String(checked)])
        }
    }
}
