import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: part

    required property var page
    readonly property var status: page.inputs.media ?? {}
    readonly property bool remote: (App.state.options ?? {})["media.remote_covers"] === true
    readonly property var using: status.using ?? []

    implicitHeight: filas.height

    Filas {
        id: filas

        Fila {
            text: App.tr("What plays now")
            value: !part.status.listening ? App.tr("No scene reads it") : part.status.track ? (part.status.playing ? App.tr("Something is playing.") : App.tr("Playback is paused or stopped.")) : App.tr("No media player is open.")
            explain: App.tr("A scene or page shows the title, artist, album, cover and progress of what a media player on this computer plays only after you allow it for that package, and only while it is shown. Nothing of it is kept, logged or sent anywhere.")
            status: !part.status.listening ? "" : [App.tr("Read now for: %1", part.using.join(", ")), part.status.players ? App.tr("Media players found: %1", part.status.players) : "", part.status.cover ? App.tr("A cover is shown.") : ""].filter(t => t).join(" ")
        }

        Fila {
            text: App.tr("Fetch covers from the internet")
            kind: "llave"
            checked: part.remote
            value: checked ? App.tr("Yes") : App.tr("No")
            explain: App.tr("Some players give the address of a cover on the internet instead of a file. With this on, such a cover is downloaded (at most 4 MiB, within 5 seconds) while an allowed wallpaper shows it, so that site learns that the track plays here; covers stay in memory only.")
            onToggled: App.act("SetOption", ["media.remote_covers", String(checked)], "Daemon1")
        }
    }
}
