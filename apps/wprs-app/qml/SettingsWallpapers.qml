import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: tab

    required property var page
    readonly property var options: App.state.options ?? {}
    readonly property int mature: options["library.hide_mature"] === true ? 0 : options["library.show_mature"] === true ? 2 : 1
    readonly property var lost: page.sources.filter(source => !!source.error)
    readonly property var plays: page.outputs.map(name => [name, ((App.state.outputs ?? {})[name] ?? {}).playlist ?? null])
    readonly property string playing: plays.length === 1 ? (plays[0][1] ? plays[0][1].name : App.trc("list", "None")) : plays.map(pair => App.tr("%1: %2", pair[0], pair[1] ? pair[1].name : App.tr("none"))).join(" · ")
    readonly property var rows: [
        {
            "text": adult.text,
            "value": adult.options[adult.current]
        },
        {
            "text": flashes.text,
            "value": flashes.options[flashes.current],
            "explain": flashes.explain
        },
        {
            "text": folders.text,
            "value": folders.value
        },
        {
            "text": lists.text,
            "value": lists.value
        },
        {
            "text": App.tr("List that plays on each screen"),
            "value": lists.value,
            "place": "lists"
        },
        {
            "text": more.text,
            "value": more.note
        },
        {
            "text": App.tr("Add a folder…"),
            "place": "folders"
        },
        {
            "text": App.tr("Include subfolders"),
            "place": "folders"
        },
        {
            "text": App.tr("Change wallpaper with the keyboard"),
            "place": "more"
        },
        {
            "text": App.tr("Previous wallpaper with the keyboard"),
            "place": "more"
        },
        {
            "text": App.tr("Keyboard shortcuts"),
            "place": "more"
        },
        {
            "text": App.tr("News from the repositories"),
            "place": "more"
        },
        {
            "text": App.tr("Thumbnail cache"),
            "place": "more"
        },
        {
            "text": App.tr("Show the number of votes"),
            "place": "more"
        },
        {
            "text": App.tr("Watch the library"),
            "place": "more"
        }
    ]

    implicitHeight: filas.height

    Filas {
        id: filas

        Seccion {
            text: App.tr("What is shown")
        }

        Fila {
            id: adult

            text: App.tr("Mature content")
            kind: "elige"
            options: [App.tr("Hidden"), App.tr("Blurred"), App.tr("Visible")]
            current: tab.mature
            explain: App.tr("Hidden leaves it out of every list. Blurred shows its cover blurred until you open it. Visible shows it as it is.")
            onChosen: index => {
                App.act("SetOption", ["library.hide_mature", String(index === 0)]);
                App.act("SetOption", ["library.show_mature", String(index === 2)]);
            }
        }

        Fila {
            id: flashes

            text: App.tr("Flashes")
            kind: "elige"
            options: [App.tr("Disabled"), App.tr("Allowed")]
            current: tab.options["playback.allow_flashing"] === true ? 1 : 0
            explain: current === 0 ? App.tr("Wallpapers with flashes or sudden changes of light do not play, and the library marks them with “Has flashes, disabled”.") : App.tr("Wallpapers with flashes or sudden changes of light play like any other.")
            onChosen: index => App.act("SetOption", ["playback.allow_flashing", String(index === 1)])
        }

        Seccion {
            text: App.tr("Your library")
        }

        Fila {
            id: folders

            text: App.tr("Library folders")
            kind: "pliega"
            value: tab.page.sources.length === 1 ? App.tr("1 folder") : App.tr("%1 folders", tab.page.sources.length)
            note: tab.lost.length > 0 ? App.tr("%1 cannot be found", tab.lost.map(source => SettingsFolder.name(source)).join(", ")) : ""
            noteTone: "coral"
            onOpened: tab.page.open("folders", folders)
        }

        Fila {
            id: lists

            text: tab.page.outputs.length > 1 ? App.tr("Lists per screen") : App.tr("List that plays")
            kind: "pliega"
            value: tab.playing
            onOpened: tab.page.open("lists", lists)
        }

        Fila {
            id: more

            text: App.tr("More")
            note: App.tr("Keyboard shortcut, news, thumbnails, votes, watching folders")
            kind: "pliega"
            onOpened: tab.page.open("more", more)
        }
    }
}
