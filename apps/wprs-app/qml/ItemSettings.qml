import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: settings

    required property var host
    readonly property var pkg: host.pkg
    readonly property bool tunable: !!pkg && ["scene", "web"].includes(pkg.kind)
    readonly property var screens: App.outputs.filter(name => ((App.state.outputs ?? {})[name] ?? {}).item === host.uid)

    visible: tunable

    Kirigami.Heading {
        level: 4
        text: App.tr("Settings")
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: settings.screens.length === 0
        wrapMode: Text.Wrap
        text: App.tr("The settings of this wallpaper appear here while it is on a screen, and in Settings under that screen. Apply it to a screen to change them.")
    }

    QQC2.Button {
        visible: settings.screens.length === 0
        text: App.tr("Apply to…")
        icon.name: "preferences-desktop-wallpaper"
        onClicked: settings.host.apply(settings.host.uid)
    }

    Repeater {
        model: settings.screens

        delegate: ColumnLayout {
            required property string modelData

            Layout.fillWidth: true

            QQC2.Label {
                visible: settings.screens.length > 1
                text: App.tr("On %1", modelData)
                font.bold: true
            }

            SceneControls {
                Layout.fillWidth: true
                daemon: App
                output: modelData
            }
        }
    }
}
