import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: part

    required property var page
    readonly property var options: App.state.options ?? {}
    readonly property var sizes: [128, 256, 512, 1024, 2048, 4096, 8192]
    readonly property int cache: Number(options["library.thumb_cache_mb"] ?? 512)
    readonly property var cached: sizes.includes(cache) ? sizes : sizes.concat([cache]).sort((a, b) => a - b)

    function flag(key, on) {
        App.act("SetOption", [key, String(on)]);
    }

    implicitHeight: filas.height

    Filas {
        id: filas

        Fila {
            text: App.tr("Change wallpaper with the keyboard")
            kind: "captura"
            keys: SettingsFolder.keys(part.options["shortcuts.next"] ?? "LOGO+ALT+N")
            explain: App.tr("Goes to the next wallpaper of the list. To change the combination, press Enter and then the new keys.")
            onCaptured: keys => App.act("SetOption", ["shortcuts.next", SettingsFolder.joined(keys)])
        }

        Fila {
            text: App.tr("Previous wallpaper with the keyboard")
            kind: "captura"
            keys: SettingsFolder.keys(part.options["shortcuts.previous"] ?? "LOGO+ALT+P")
            explain: App.tr("Goes back to the wallpaper before. To change the combination, press Enter and then the new keys.")
            onCaptured: keys => App.act("SetOption", ["shortcuts.previous", SettingsFolder.joined(keys)])
        }

        Fila {
            text: App.tr("Keyboard shortcuts")
            note: App.tr("The desktop is asked for these two combinations")
            kind: "llave"
            checked: part.options["shortcuts.enabled"] === true
            value: checked ? App.tr("Yes") : App.tr("No")
            explain: App.tr("Where the desktop cannot give them, bind the commands of the terminal tool to your own keys.")
            onToggled: part.flag("shortcuts.enabled", checked)
        }

        Fila {
            text: App.tr("News from the repositories")
            kind: "elige"
            options: [App.tr("Only when you ask"), App.tr("Look for them"), App.tr("Look for them and install them")]
            current: part.options["repos.auto_update"] === true ? 2 : part.options["repos.auto_refresh"] === false ? 0 : 1
            explain: App.tr("New wallpapers and new versions of the ones you have, in the repositories you added.")
            onChosen: index => {
                part.flag("repos.auto_refresh", index > 0);
                part.flag("repos.auto_update", index === 2);
            }
        }

        Fila {
            text: App.tr("Thumbnail cache")
            note: App.tr("They are made again when needed")
            kind: "elige"
            options: part.cached.map(size => size >= 1024 && size % 1024 === 0 ? App.tr("%1 GB", size / 1024) : App.tr("%1 MB", size))
            current: Math.max(0, part.cached.indexOf(part.cache))
            explain: App.tr("The most room the small pictures of your library may take on this computer.")
            onChosen: index => App.act("SetOption", ["library.thumb_cache_mb", String(part.cached[index])])
        }

        Fila {
            text: App.tr("Show the number of votes")
            kind: "llave"
            checked: part.options["repos.vote_counts"] === true
            value: checked ? App.tr("Yes") : App.tr("No")
            onToggled: part.flag("repos.vote_counts", checked)
        }

        Fila {
            text: App.tr("Watch the library")
            note: App.tr("New wallpapers in your folders appear by themselves")
            kind: "llave"
            checked: part.options["library.watch"] !== false
            value: checked ? App.tr("Yes") : App.tr("No")
            onToggled: part.flag("library.watch", checked)
        }
    }
}
