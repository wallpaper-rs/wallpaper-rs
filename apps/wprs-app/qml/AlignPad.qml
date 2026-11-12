import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: pad

    property real px: 0
    property real py: 0
    readonly property real snap: 0.1
    readonly property real dot: Kirigami.Units.gridUnit * 0.9

    signal moved(real x, real y)
    signal settled(real x, real y)

    function clamp(v) {
        const c = Math.max(-1, Math.min(1, v));
        return [-1, 0, 1].find(s => Math.abs(c - s) < snap) ?? Math.round(c * 100) / 100;
    }

    function at(point) {
        return [clamp(point.x / width * 2 - 1), clamp(point.y / height * 2 - 1)];
    }

    implicitWidth: Kirigami.Units.gridUnit * 12
    implicitHeight: implicitWidth * 9 / 16
    Accessible.role: Accessible.Slider
    Accessible.name: App.tr("Alignment")

    Rectangle {
        anchors.fill: parent
        radius: Kirigami.Units.smallSpacing
        color: Kirigami.Theme.alternateBackgroundColor
        border.color: Kirigami.Theme.disabledTextColor
    }

    Repeater {
        model: 9

        delegate: Rectangle {
            required property int index

            x: (index % 3) * (pad.width - width) / 2
            y: Math.floor(index / 3) * (pad.height - height) / 2
            width: Kirigami.Units.smallSpacing * 1.5
            height: width
            radius: width / 2
            color: Kirigami.Theme.disabledTextColor
        }
    }

    Rectangle {
        x: (pad.px + 1) / 2 * pad.width - width / 2
        y: (pad.py + 1) / 2 * pad.height - height / 2
        width: pad.dot
        height: width
        radius: width / 2
        color: Kirigami.Theme.highlightColor
        border.color: Kirigami.Theme.backgroundColor
        border.width: 2
    }

    MouseArea {
        anchors.fill: parent
        cursorShape: Qt.CrossCursor
        onPressed: mouse => {
            const p = pad.at(mouse);
            pad.moved(p[0], p[1]);
        }
        onPositionChanged: mouse => {
            if (pressed) {
                const p = pad.at(mouse);
                pad.moved(p[0], p[1]);
            }
        }
        onReleased: mouse => {
            const p = pad.at(mouse);
            pad.settled(p[0], p[1]);
        }
    }
}
