import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: part

    required property var page

    implicitHeight: filas.height

    Filas {
        id: filas

        Repeater {
            model: part.page.outputs

            Fila {
                required property string modelData
                readonly property var playing: ((App.state.outputs ?? {})[modelData] ?? {}).playlist ?? null

                objectName: "dynamic"
                width: parent.width
                text: part.page.outputs.length > 1 ? modelData : App.tr("List that plays")
                kind: "elige"
                options: [App.tr("No list")].concat(part.page.playlists.map(list => list.name))
                current: playing ? 1 + part.page.playlists.findIndex(list => list.uid === playing.uid) : 0
                explain: App.tr("The wallpapers of this list take turns on this screen, as the list's rules say.")
                onChosen: index => index > 0 ? App.act("Start", [part.page.playlists[index - 1].uid, modelData], "Playlists1") : App.act("Stop", [modelData], "Playlists1")
            }
        }
    }
}
