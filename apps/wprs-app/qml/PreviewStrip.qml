import QtQuick
import io.github.wallpaper_rs.app

Column {
    id: strip

    required property var preview
    readonly property var session: preview.session
    readonly property bool listed: session.source === "playlist"

    spacing: Look.e3

    Estado {
        anchors.right: parent.right
        icon: strip.listed ? "playlist" : "books"
        text: strip.listed ? App.tr("List «%1» · %2 of %3", strip.session.playlist_name ?? "", (strip.session.position ?? 0) + 1, strip.session.count ?? 0) : App.tr("Your library · %1 of %2", (strip.session.position ?? 0) + 1, strip.session.count ?? 0)
    }

    Row {
        anchors.right: parent.right
        spacing: Look.e4

        PreviewFigure {
            anchors.bottom: parent.bottom
            visible: strip.preview.current !== null
            item: strip.preview.current
            size: PreviewLook.tira
            opacity: PreviewLook.tiraAhora
            caption: App.tr("%1 · now", strip.preview.current ? strip.preview.current.title : "")
            using: true
        }

        PreviewFigure {
            anchors.bottom: parent.bottom
            visible: strip.preview.candidate !== null
            item: strip.preview.candidate
            size: PreviewLook.tiraFoco
            caption: strip.preview.candidate ? strip.preview.candidate.title : ""
            focused: true
        }

        Repeater {
            model: strip.session.ahead ?? []

            delegate: PreviewFigure {
                required property var modelData

                anchors.bottom: parent.bottom
                item: modelData
                size: PreviewLook.tira
                opacity: PreviewLook.tiraApagada
                caption: modelData.title
            }
        }
    }
}
