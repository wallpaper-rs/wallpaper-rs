import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: settings

    required property var host
    readonly property var pkg: host.pkg
    readonly property bool tunable: !!pkg && ["scene", "web"].includes(pkg.kind)
    property var screens: App.outputs.filter(name => ((App.state.outputs ?? {})[name] ?? {}).item === host.uid)

    property int bare: 0
    readonly property bool empty: screens.length > 0 && bare === screens.length

    visible: tunable

    Seccion {
        visible: settings.bare < settings.screens.length || settings.screens.length === 0
        text: App.tr("From its author")
    }

    Vacio {
        Layout.fillWidth: true
        visible: settings.empty
        small: true
        title: App.tr("This scene has no options of its own")
        text: App.tr("Its author left nothing to adjust: it looks the same for everybody. Below are your presets, which work for every scene, and the rest is under “More”.")
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
                text: App.tr("On %1", App.screen(modelData))
                font.bold: true
            }

            SceneControls {
                readonly property bool none: props.package !== undefined && fields.length === 0

                Layout.fillWidth: true
                onNoneChanged: settings.bare += none ? 1 : -1
                Component.onDestruction: settings && none && (settings.bare -= 1)
                daemon: App
                output: modelData
                tools: false
                rows: true
            }
        }
    }
}
