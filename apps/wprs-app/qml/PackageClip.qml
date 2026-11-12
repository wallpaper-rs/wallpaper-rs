import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: clip

    required property var page
    readonly property var settings: page.manifest.playback ?? ({})
    readonly property var frames: JSON.parse(strips.frames || "{}")[info?.file ?? ""] ?? []
    readonly property real length: info?.length ?? Math.max(settings.end ?? 0, (settings.start ?? 0) + 1, 10)
    readonly property real from: Math.min(settings.start ?? 0, length)
    readonly property real to: settings.end ?? length
    property var info: null

    function load() {
        info = JSON.parse(page.editor.rig("", JSON.stringify({
                        "media": {}
                    })) || "null");
        if (info?.file)
            strips.ask(info.file, 10);
    }

    function trim(key, seconds) {
        const value = Math.round(Math.max(0, Math.min(length, seconds)) * 100) / 100;
        if (key === "start")
            page.manifestEdit("playback", {
                    "values": {
                        "start": Math.min(value, Math.max(0, to - 0.1))
                    }
                });
        else if (value >= length - 0.05)
            page.manifestEdit("playback", {
                    "clear": ["end"]
                });
        else
            page.manifestEdit("playback", {
                    "values": {
                        "end": Math.max(value, from + 0.1)
                    }
                });
    }

    spacing: Kirigami.Units.smallSpacing
    Component.onCompleted: load()

    Strips {
        id: strips
    }

    Connections {
        function onViewChanged() {
            clip.load();
        }

        target: clip.page.editor
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        visible: !clip.info?.present
        color: Kirigami.Theme.neutralTextColor
        text: App.tr("Choose the video first: the part that plays is cut on its timeline.")
    }

    Item {
        id: bar

        Layout.fillWidth: true
        Layout.preferredHeight: Kirigami.Units.gridUnit * 3
        visible: !!clip.info?.present

        Rectangle {
            anchors.fill: parent
            radius: Kirigami.Units.smallSpacing
            color: Kirigami.Theme.alternateBackgroundColor
        }

        Row {
            anchors.fill: parent

            Repeater {
                model: clip.frames

                delegate: Image {
                    required property var modelData

                    width: bar.width / Math.max(1, clip.frames.length)
                    height: bar.height
                    source: App.url(modelData.file)
                    fillMode: Image.PreserveAspectCrop
                    asynchronous: true
                    opacity: 0.55
                }
            }
        }

        Rectangle {
            x: clip.from / clip.length * bar.width
            width: Math.max(2, (clip.to - clip.from) / clip.length * bar.width)
            height: bar.height
            color: "transparent"
            border.color: Kirigami.Theme.highlightColor
            border.width: 3
        }

        Repeater {
            model: ["start", "end"]

            delegate: Rectangle {
                id: grip

                required property string modelData
                readonly property real seconds: modelData === "start" ? clip.from : clip.to
                readonly property string held: ((x + width / 2) / bar.width * clip.length).toFixed(2)

                x: seconds / clip.length * bar.width - width / 2
                width: Kirigami.Units.gridUnit * 0.7
                height: bar.height
                radius: 2
                color: Kirigami.Theme.highlightColor
                border.color: Kirigami.Theme.backgroundColor

                HoverHandler {
                    id: over
                }

                QQC2.ToolTip.visible: over.hovered || move.active
                QQC2.ToolTip.text: modelData === "start" ? App.tr("Start %1 s", held) : App.tr("End %1 s", held)

                DragHandler {
                    id: move

                    target: grip
                    yAxis.enabled: false
                    xAxis.minimum: -grip.width / 2
                    xAxis.maximum: bar.width - grip.width / 2
                    onActiveChanged: {
                        if (!active)
                            clip.trim(grip.modelData, (grip.x + grip.width / 2) / bar.width * clip.length);
                    }
                }
            }
        }
    }

    RowLayout {
        Layout.fillWidth: true
        visible: !!clip.info?.present

        QQC2.Label {
            Layout.fillWidth: true
            text: App.tr("Plays from %1 s to %2 s of %3 s", clip.from.toFixed(2), clip.to.toFixed(2), clip.length.toFixed(2))
        }

        QQC2.Button {
            text: App.tr("Whole video")
            icon.name: "edit-undo"
            enabled: clip.settings.start !== undefined || clip.settings.end !== undefined
            onClicked: clip.page.manifestEdit("playback", {
                    "clear": ["start", "end"]
                })
        }
    }

    RowLayout {
        Layout.fillWidth: true

        QQC2.Label {
            text: App.tr("Speed:")
        }

        SceneSlider {
            id: speed

            Layout.preferredWidth: Kirigami.Units.gridUnit * 10
            editor: clip.page.editor
            from: 0
            to: 1
            value: Math.log((clip.settings.speed ?? 1) / 0.1) / Math.log(40)
            onMoved: clip.page.manifestEdit("playback", {
                    "values": {
                        "speed": Math.round(0.1 * Math.pow(40, value) * 100) / 100
                    }
                }, true)
        }

        QQC2.Label {
            text: Number(clip.settings.speed ?? 1).toLocaleString(Qt.locale(), "f", 2) + "×"
        }

        QQC2.Button {
            text: App.tr("Default")
            icon.name: "edit-undo"
            enabled: clip.settings.speed !== undefined
            onClicked: clip.page.manifestEdit("playback", {
                    "clear": ["speed"]
                })
        }

        QQC2.CheckBox {
            text: App.tr("Loop")
            checked: clip.settings.loop !== false
            onToggled: checked ? clip.page.manifestEdit("playback", {
                    "clear": ["loop"]
                }) : clip.page.manifestEdit("playback", {
                    "values": {
                        "loop": false
                    }
                })
        }
    }
}
