import QtQuick
import QtQuick.Layouts
import "../look"

ColumnLayout {
    id: head

    required property var daemon
    property bool titled: false
    readonly property var current: daemon.current
    readonly property bool paused: current.paused === true
    readonly property bool muted: current.muted === true
    property alias pause: pausing

    enabled: daemon.pollError === ""
    spacing: Look.e3

    RowLayout {
        Layout.fillWidth: true
        visible: head.titled

        Words {
            text: "wallpaper-rs"
            title: true
            font.pixelSize: Look.tFila
        }

        Item {
            Layout.fillWidth: true
        }

        Status {
            paused: head.paused
        }
    }

    Rectangle {
        Layout.preferredHeight: Look.altoControl
        Layout.preferredWidth: group.implicitWidth + 2 * Look.linea
        radius: Look.radio
        color: "transparent"
        border.width: Look.linea
        border.color: Look.nocheLinea

        RowLayout {
            id: group

            anchors.fill: parent
            anchors.margins: Look.linea
            spacing: 0

            Command {
                id: pausing

                Layout.fillHeight: true
                quiet: true
                glyph: head.paused ? "play-fill" : "pause-fill"
                text: head.paused ? i18n("Resume") : i18n("Pause")
                onClicked: head.daemon.act("SetPaused", [!head.paused])
            }

            Rectangle {
                Layout.fillHeight: true
                implicitWidth: Look.linea
                color: Look.nocheLinea
            }

            Command {
                Layout.fillHeight: true
                quiet: true
                glyph: head.muted ? "speaker-slash" : "speaker-high"
                text: ""
                Accessible.name: head.muted ? i18n("Unmute") : i18n("Mute all")
                onClicked: head.daemon.act("SetMuted", [!head.muted])
            }

            Level {
                id: volume

                Layout.preferredWidth: 84
                Layout.rightMargin: Look.e3
                from: 0
                to: 1
                value: head.current.volume ?? 0
                Accessible.name: i18n("Master volume")
                onMoved: debounce.restart()
            }
        }
    }

    Timer {
        id: debounce

        interval: 200
        onTriggered: {
            head.daemon.act("SetVolume", [volume.value]);
            if (head.muted)
                head.daemon.act("SetMuted", [false]);
        }
    }
}
