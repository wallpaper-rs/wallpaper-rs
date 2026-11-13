import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

RowLayout {
    id: picker

    required property var daemon
    property string output: ""
    property var info: ({})
    readonly property string screen: output || "default"
    readonly property bool silenced: info.sound_muted === true

    QQC2.Switch {
        text: picker.daemon.tr("Sound of this wallpaper")
        checked: !picker.silenced
        QQC2.ToolTip.text: picker.daemon.tr("Silences or plays the sound of the wallpaper of this screen only. The global mute stays the master: while it is on, nothing plays.")
        QQC2.ToolTip.visible: hovered
        onToggled: picker.daemon.act("SetOption", ["playback.muted." + picker.screen, checked ? "false" : "true"])
    }

    QQC2.Slider {
        id: level

        enabled: !picker.silenced
        from: 0
        to: 1
        value: picker.info.sound_volume ?? 1
        Accessible.name: picker.daemon.tr("Volume of this wallpaper")
        onMoved: debounce.restart()
    }

    QQC2.Label {
        text: Math.round(level.value * 100) + " %"
    }

    Timer {
        id: debounce

        interval: 200
        onTriggered: picker.daemon.act("SetOption", ["playback.volume." + picker.screen, level.value.toFixed(2)])
    }
}
