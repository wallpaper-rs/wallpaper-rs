import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: more

    required property var page
    required property var playlist

    Loader {
        Layout.fillWidth: true
        Layout.fillHeight: true
        active: more.visible && !!more.playlist.uid
        sourceComponent: PlaylistEditor {
            playlist: more.playlist
        }
    }

    QQC2.Button {
        flat: true
        icon.name: "trash"
        text: App.tr("Remove the list")
        onClicked: more.page.remove(more.playlist)
    }
}
