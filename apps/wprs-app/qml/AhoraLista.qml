import QtQuick
import io.github.wallpaper_rs.app

Row {
    id: line

    required property string screen
    required property var playlist
    property real now: Date.now() / 1000
    readonly property real remaining: playlist && playlist.next_at ? playlist.next_at - now : -1

    function when() {
        if (remaining < 0)
            return App.tr("List “%1”", playlist ? playlist.name : "");
        const minutes = Math.max(1, Math.ceil(remaining / 60));
        const span = minutes < 90 ? App.tr("%1 min", minutes) : App.tr("%1 h", Math.round(minutes / 60));
        return App.tr("List “%1” · changes in %2", playlist.name, span);
    }

    function stop() {
        const was = playlist;
        const output = screen;
        App.act("Stop", [output], "Playlists1", () => applicationWindow().showPassiveNotification(App.tr("The rotation of “%1” stopped.", was.name), "long", App.tr("Resume"), () => App.act("Start", [was.uid, output], "Playlists1")));
    }

    spacing: Look.e3
    visible: !!playlist
    layoutDirection: Qt.RightToLeft

    Timer {
        interval: 60000
        repeat: true
        running: line.visible && line.Window.window !== null && line.Window.window.visible
        triggeredOnStart: true
        onTriggered: line.now = Date.now() / 1000
    }

    Mandos {
        label: App.tr("The list that rotates")

        Mando {
            first: true
            icon: "skip-back"
            text: App.tr("Previous")
            onClicked: App.act("Previous", [line.screen], "Playlists1")
        }

        Mando {
            icon: "skip-forward"
            text: App.tr("Next")
            onClicked: App.act("Next", [line.screen], "Playlists1")
        }

        Mando {
            icon: "pause-fill"
            text: App.tr("Pause the list")
            onClicked: line.stop()
        }
    }

    Estado {
        anchors.verticalCenter: parent.verticalCenter
        icon: "playlist"
        text: line.when()
    }
}
