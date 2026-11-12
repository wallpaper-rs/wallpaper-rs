import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: panel

    required property var page
    readonly property bool placed: panel.page.editor.placed !== ""

    PackageTrack {
        Layout.fillWidth: true
        page: panel.page
    }

    Kirigami.Heading {
        Layout.fillWidth: true
        level: 4
        text: App.tr("On a screen")
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        text: App.tr("Show a made-up track on the screen where this was last applied, instead of what a media player plays, to try values, triggers and scripts; this changes what is really shown there, not only a preview.")
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: !panel.placed
        wrapMode: Text.Wrap
        text: App.tr("Build and apply this package to a screen first.")
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: panel.placed
        text: App.tr("Screen: %1", panel.page.editor.placed)
    }

    Flow {
        Layout.fillWidth: true
        enabled: panel.placed

        QQC2.Button {
            text: App.tr("Simulate from a file…")
            icon.name: "view-media-playlist"
            onClicked: files.open()
        }

        QQC2.Button {
            text: App.tr("Stop")
            icon.name: "media-playback-stop"
            onClicked: panel.page.editor.simulateMedia("")
        }
    }

    FileDialog {
        id: files

        fileMode: FileDialog.OpenFile
        nameFilters: [App.tr("A JSON file of a made-up track") + " (*.json)"]
        onAccepted: panel.page.editor.simulateMedia(App.path(selectedFile))
    }
}
