import QtQuick
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: note

    property string playlist
    property var items: []
    readonly property var shown: items.filter(a => !playlist || (a.playlists ?? []).includes(playlist))

    function load() {
        App.query("ListAway", [], "Sync1", reply => note && (note.items = reply.items ?? []));
    }

    visible: shown.length > 0
    spacing: 0
    Component.onCompleted: load()

    Connections {
        target: App

        function onUpdated(topic) {
            if (["account", "library", "playlists"].includes(topic))
                note.load();
        }
    }

    Repeater {
        model: note.shown

        Kirigami.InlineMessage {
            required property var modelData

            Layout.fillWidth: true
            visible: true
            type: Kirigami.MessageType.Information
            text: App.tr("%1 exists only on another device. Publishing it to the server or to a repository makes it available here.", modelData.title || modelData.key.slice(0, 8))
        }
    }
}
