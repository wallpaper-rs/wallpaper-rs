import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.FormLayout {
    id: playback

    required property var page
    readonly property var settings: page.manifest.playback ?? ({})
    readonly property var rates: [0, "display", 10, 15, 24, 30, 48, 60, 90, 120, 144, 240]

    function put(values, clear, gesture) {
        page.manifestEdit("playback", {
                "values": values,
                "clear": clear ?? []
            }, gesture === true);
    }

    QQC2.ComboBox {
        Kirigami.FormData.label: App.tr("Highest frame rate:")
        model: [App.tr("Not set"), App.tr("Follow the screen")].concat(playback.rates.slice(2).map(r => App.tr("%1 frames a second", r)))
        currentIndex: Math.max(0, playback.rates.indexOf(playback.settings.max_fps ?? 0))
        onActivated: index => index === 0 ? playback.put({}, ["max_fps"]) : playback.put({
                    "max_fps": playback.rates[index]
                })
    }

    RowLayout {
        Kirigami.FormData.label: App.tr("Volume:")

        SceneSlider {
            id: volume

            Layout.preferredWidth: Kirigami.Units.gridUnit * 12
            editor: playback.page.editor
            from: 0
            to: 1
            stepSize: 0.05
            value: playback.settings.volume ?? 1
            onMoved: playback.put({
                    "volume": Math.round(value * 100) / 100
                }, [], true)
        }

        QQC2.Label {
            text: Math.round(volume.value * 100) + " %"
        }

        QQC2.Button {
            text: App.tr("Default")
            icon.name: "edit-undo"
            enabled: playback.settings.volume !== undefined
            onClicked: playback.put({}, ["volume"])
        }
    }

    QQC2.CheckBox {
        Kirigami.FormData.label: App.tr("Sound:")
        text: App.tr("Start without sound")
        checked: playback.settings.muted === true
        onToggled: checked ? playback.put({
                "muted": true
            }) : playback.put({}, ["muted"])
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        color: Kirigami.Theme.disabledTextColor
        text: App.tr("What the user sets for the screen always wins over these defaults.")
    }
}
