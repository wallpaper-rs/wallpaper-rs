import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: part

    required property var page
    readonly property var options: App.state.options ?? {}
    readonly property bool several: page.outputs.length > 1
    readonly property var info: (App.state.outputs ?? {})[page.output] ?? {}
    readonly property string fps: String(options["playback.max_fps"] ?? "display")
    readonly property var rates: ["display", "15", "24", "30", "60", "120"].concat(["display", "15", "24", "30", "60", "120"].includes(fps) ? [] : [fps])
    readonly property var levels: (several ? ["none"] : []).concat(SettingsWords.levels)
    readonly property string level: several ? String(info.quality_choice ?? "none") : String(options["playback.quality"] ?? "auto")

    implicitHeight: filas.height

    Filas {
        id: filas

        Fila {
            text: App.tr("Every quality level")
            note: part.several ? part.page.output : ""
            kind: "elige"
            options: part.levels.map(value => SettingsWords.level(value))
            current: Math.max(0, part.levels.indexOf(part.level))
            explain: App.tr("Automatic levels lower themselves when they must; the fixed ones are kept even on battery.")
            onChosen: index => App.act("SetOption", [part.several ? "playback.quality." + part.page.output : "playback.quality", part.levels[index]])
        }

        Fila {
            text: App.tr("Highest frames per second")
            note: App.tr("On every screen")
            kind: "elige"
            options: part.rates.map(value => value === "display" ? App.tr("According to the screen") : App.tr("%1 per second", value))
            current: Math.max(0, part.rates.indexOf(part.fps))
            explain: App.tr("Fewer frames per second use less battery. According to the screen follows its refresh rate.")
            onChosen: index => App.act("SetOption", ["playback.max_fps", part.rates[index]])
        }

        Fila {
            text: App.tr("When a scene is heavy")
            note: App.tr("On every screen")
            kind: "elige"
            options: [App.tr("Keep the detail"), App.tr("Keep it smooth")]
            current: part.options["playback.auto_pace"] === "smooth" ? 1 : 0
            explain: App.tr("Keep the detail lowers the frames per second first; keep it smooth lowers the quality first.")
            onChosen: index => App.act("SetOption", ["playback.auto_pace", ["detail", "smooth"][index]])
        }

        Fila {
            text: App.tr("Lowest quality on battery below")
            note: App.tr("On every screen")
            kind: "riel"
            from: 0
            to: 100
            stepSize: 5
            at: Number(part.options["playback.lowest_below_percent"] ?? 0)
            number: at > 0 ? App.tr("%1 %", Math.round(at)) : App.tr("Never")
            explain: App.tr("Below this charge every scene plays at the lowest quality, whatever you fixed.")
            onMoved: value => App.act("SetOption", ["playback.lowest_below_percent", String(Math.round(value))])
        }

        Fila {
            text: App.tr("Length of the transition")
            note: App.tr("On every screen")
            kind: "riel"
            from: 0
            to: 3000
            stepSize: 100
            at: Number(part.options["transition.duration_ms"] ?? 600)
            number: App.tr("%1 ms", Math.round(at))
            onMoved: value => App.act("SetOption", ["transition.duration_ms", String(Math.round(value))])
        }

        Seccion {
            text: App.tr("Wallpaper of %1", part.page.output)
        }

        OutputRow {
            width: parent.width
            daemon: App
            name: part.page.output
            info: part.info
            playlists: part.page.playlists
        }
    }
}
