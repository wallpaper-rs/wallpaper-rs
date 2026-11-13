import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

QQC2.ToolBar {
    id: bar

    required property ItemModel items
    required property var tools

    position: QQC2.ToolBar.Footer

    contentItem: RowLayout {
        QQC2.Label {
            Layout.fillWidth: true
            elide: Text.ElideRight
            text: bar.items.selected > 0 ? App.tr("%1 chosen", bar.items.selected) : App.tr("Click a picture to choose it. Ctrl+click adds one, Shift+click chooses a range, Ctrl+A chooses all.")
        }

        QQC2.Button {
            visible: bar.items.selected > 0
            text: App.tr("Add to playlist…")
            icon.name: "view-media-playlist"
            onClicked: bar.tools.addToPlaylist()
        }

        QQC2.Button {
            visible: bar.items.selected > 0
            text: App.tr("Rotate every…")
            icon.name: "media-playlist-repeat"
            onClicked: bar.tools.rotate()
        }

        QQC2.Button {
            text: bar.items.selected > 0 ? App.tr("Clear") : App.tr("Choose all")
            onClicked: bar.items.everything(bar.items.selected === 0)
        }
    }
}
