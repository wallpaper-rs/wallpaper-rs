import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: preview

    required property var page
    required property bool ready
    property bool orbit
    property real turn

    function aim() {
        if (page.regionPreview && ready)
            page.relief.peek(page.editor.json, page.pointer[0], page.pointer[1]);
    }

    function place(x, y) {
        page.pointer = [Math.max(-1, Math.min(1, x / pad.width * 2 - 1)), Math.max(-1, Math.min(1, y / pad.height * 2 - 1))];
    }

    Connections {
        target: preview.page

        function onPointerChanged() {
            preview.aim();
        }

        function onRegionPreviewChanged() {
            preview.aim();
        }
    }

    onReadyChanged: aim()

    Timer {
        interval: 100
        repeat: true
        running: preview.orbit && preview.page.regionPreview && preview.visible
        onTriggered: {
            preview.turn += 0.12;
            preview.page.pointer = [0.8 * Math.cos(preview.turn), 0.8 * Math.sin(preview.turn)];
        }
    }

    QQC2.Switch {
        text: App.tr("Preview the movement while you edit")
        checked: preview.page.regionPreview
        onToggled: preview.page.regionPreview = checked
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: preview.page.regionPreview && !preview.ready
        text: App.tr("The preview needs the layers of this picture; it starts when they are built.")
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
    }

    RowLayout {
        visible: preview.page.regionPreview

        Rectangle {
            id: pad

            Layout.preferredWidth: Kirigami.Units.gridUnit * 6
            Layout.preferredHeight: Kirigami.Units.gridUnit * 4
            color: Kirigami.Theme.alternateBackgroundColor
            border.color: Kirigami.Theme.textColor
            activeFocusOnTab: true
            Keys.onPressed: event => {
                const step = 0.2, p = preview.page.pointer;
                const moves = {
                    [Qt.Key_Left]: [-step, 0],
                    [Qt.Key_Right]: [step, 0],
                    [Qt.Key_Up]: [0, -step],
                    [Qt.Key_Down]: [0, step]
                };
                const by = moves[event.key];
                if (by) {
                    preview.page.pointer = [Math.max(-1, Math.min(1, p[0] + by[0])), Math.max(-1, Math.min(1, p[1] + by[1]))];
                    event.accepted = true;
                }
            }

            Rectangle {
                x: (preview.page.pointer[0] + 1) / 2 * pad.width - width / 2
                y: (preview.page.pointer[1] + 1) / 2 * pad.height - height / 2
                width: Kirigami.Units.gridUnit
                height: width
                radius: width / 2
                color: Kirigami.Theme.highlightColor
                border.color: pad.activeFocus ? Kirigami.Theme.textColor : "transparent"
            }

            MouseArea {
                anchors.fill: parent
                onPressed: mouse => {
                    preview.orbit = false;
                    preview.place(mouse.x, mouse.y);
                }
                onPositionChanged: mouse => {
                    if (pressed)
                        preview.place(mouse.x, mouse.y);
                }
            }
        }

        QQC2.Switch {
            text: App.tr("Orbit")
            checked: preview.orbit
            onToggled: preview.orbit = checked
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: preview.page.regionPreview
        text: App.tr("Move the dot to play the pointer, or let it orbit. Over the picture, the pointer moves it too.")
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
    }
}
