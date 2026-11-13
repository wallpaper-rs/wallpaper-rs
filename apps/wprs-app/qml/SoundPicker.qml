import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

RowLayout {
    id: picker

    required property var daemon
    property string item: ""
    property var own: ({})
    readonly property bool silenced: own.sound_muted === true

    function load() {
        if (!item) {
            own = {};
            return;
        }
        const wanted = item;
        picker.daemon.query("GetItem", [wanted], "Library1", reply => picker && picker.item === wanted && (picker.own = reply.item));
    }

    function save(muted, volume) {
        picker.daemon.act("SetSound", [[item], muted, volume], "Library1");
    }

    visible: item !== ""
    onItemChanged: load()
    Component.onCompleted: load()

    Connections {
        target: picker.daemon.backend

        function onChanged(topic) {
            if (topic === "library")
                picker.load();
        }
    }

    QQC2.Switch {
        text: picker.daemon.tr("Sound of this wallpaper")
        checked: !picker.silenced
        QQC2.ToolTip.text: picker.daemon.tr("Silences or plays this wallpaper wherever it is shown, with its own volume. The global mute stays the master: while it is on, nothing plays.")
        QQC2.ToolTip.visible: hovered
        onToggled: picker.save(checked ? "false" : "true", "")
    }

    QQC2.Slider {
        id: level

        enabled: !picker.silenced
        from: 0
        to: 100
        stepSize: 5
        value: picker.own.sound_volume ?? 100
        Accessible.name: picker.daemon.tr("Volume of this wallpaper")
        onMoved: debounce.restart()
    }

    QQC2.Label {
        text: Math.round(level.value) + " %"
    }

    Timer {
        id: debounce

        interval: 200
        onTriggered: picker.save("", String(Math.round(level.value)))
    }
}
