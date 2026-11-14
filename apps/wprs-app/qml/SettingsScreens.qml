import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: tab

    required property var page
    readonly property var options: App.state.options ?? {}
    readonly property bool several: page.outputs.length > 1
    readonly property var info: (App.state.outputs ?? {})[page.output] ?? {}
    readonly property bool battery: App.state.on_battery === true || App.state.battery_percent !== null && App.state.battery_percent !== undefined
    readonly property var profile: page.profiles.find(entry => entry.uid === page.active) ?? null
    readonly property var quality: SettingsWords.quality(several, page.output, info, options)
    readonly property var passage: SettingsWords.transition(several, page.output, info, options)
    readonly property var rows: [
        {
            "text": App.tr("Screen you adjust")
        },
        {
            "text": scenes.text,
            "value": scenes.options[scenes.current],
            "explain": scenes.explain
        },
        {
            "text": change.text,
            "value": change.options[change.current]
        },
        {
            "text": App.tr("Transition on every screen")
        },
        {
            "text": calm.text,
            "value": calm.options[calm.current],
            "explain": calm.explain
        },
        {
            "text": pause.text,
            "value": pause.value
        },
        {
            "text": App.tr("Pause when a full-screen window covers the wallpaper"),
            "place": "pause",
            "explain": App.tr("When you play or watch a film in full screen, the wallpaper stops drawing and uses nothing.")
        },
        {
            "text": App.tr("Pause on battery"),
            "place": "pause"
        },
        {
            "text": App.tr("Pause with the screen locked"),
            "place": "pause"
        },
        {
            "text": more.text,
            "value": more.value
        },
        {
            "text": App.tr("Every quality level"),
            "place": "screen"
        },
        {
            "text": App.tr("Highest frames per second"),
            "place": "screen"
        },
        {
            "text": App.tr("When a scene is heavy"),
            "place": "screen"
        },
        {
            "text": App.tr("Lowest quality on battery below"),
            "place": "screen"
        },
        {
            "text": App.tr("Length of the transition"),
            "place": "screen"
        },
        {
            "text": layouts.text,
            "value": layouts.value,
            "off": layouts.off
        },
        {
            "text": App.tr("Save this layout…"),
            "place": "layouts"
        },
        {
            "text": App.tr("No layout"),
            "place": "layouts"
        },
        {
            "text": App.tr("New preset…"),
            "place": "layouts"
        }
    ]

    implicitHeight: filas.height

    Filas {
        id: filas

        Fila {
            visible: tab.several
            text: App.tr("Screen you adjust")
            kind: "elige"
            note: tab.page.output
            options: tab.page.outputs.map(name => App.screen(name))
            current: tab.page.screen
            explain: App.tr("What says “On every screen” applies to all of them.")
            onChosen: index => tab.page.screen = index
        }

        Fila {
            id: scenes

            text: App.tr("Scene quality")
            note: tab.quality.note
            kind: "elige"
            options: tab.quality.words
            current: tab.quality.current
            explain: App.tr("It lowers itself on battery, when hot or if a scene weighs too much, and goes back up when it can. The other two: Save battery and Highest. Every level is in “More”.")
            onChosen: index => App.act("SetOption", [tab.quality.key, tab.quality.values[index]])
        }

        Fila {
            id: change

            text: App.tr("Transition when the wallpaper changes")
            note: tab.several ? App.screen(tab.page.output) : ""
            kind: "elige"
            options: tab.passage.words
            current: tab.passage.current
            status: tab.passage.why
            explain: App.tr("The passage from one wallpaper to the next. Its length is in “More”.")
            onChosen: index => App.act("SetOption", [tab.passage.key, tab.passage.values[index]])
        }

        Fila {
            visible: tab.several
            text: App.tr("Transition on every screen")
            note: App.tr("On every screen")
            kind: "elige"
            options: SettingsWords.passages.map(value => SettingsWords.passage(value))
            current: Math.max(0, SettingsWords.passages.indexOf(String(tab.options["transition.kind"] ?? "own")))
            onChosen: index => App.act("SetOption", ["transition.kind", SettingsWords.passages[index]])
        }

        Fila {
            id: calm

            text: App.tr("Less motion")
            note: App.tr("On every screen")
            kind: "elige"
            options: [App.tr("Like the system"), App.tr("Always"), App.tr("Never")]
            current: Math.max(0, ["system", "always", "never"].indexOf(String(tab.options["transition.less_motion"] ?? "system")))
            explain: App.tr("With less motion, every change of wallpaper is a plain fade. “Like the system” follows the setting of your desktop.")
            onChosen: index => App.act("SetOption", ["transition.less_motion", ["system", "always", "never"][index]])
        }

        Fila {
            id: pause

            text: App.tr("Pause")
            note: App.tr("On every screen")
            kind: "pliega"
            value: App.tr("Full screen, battery, lock")
            onOpened: tab.page.open("pause", pause)
        }

        Fila {
            id: more

            text: App.tr("More of %1", App.screen(tab.page.output))
            kind: "pliega"
            value: App.tr("Smoothness, fill color, HDR")
            onOpened: tab.page.open("screen", more)
        }

        Fila {
            id: layouts

            text: App.tr("Layouts")
            note: [App.tr("They keep which wallpaper goes on each screen"), layouts.off ? App.tr("useful with several screens or a laptop") : App.tr("they change by themselves when you connect or remove a screen")].join(" · ")
            off: !tab.several && !tab.battery && tab.page.profiles.length === 0
            kind: "pliega"
            value: tab.profile ? tab.profile.name : App.tr("Save this…")
            onOpened: tab.page.open("layouts", layouts)
        }
    }
}
