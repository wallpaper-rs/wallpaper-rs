import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

MouseArea {
    id: touch

    required property var page
    required property bool testing
    readonly property color tint: touch.pressedButtons ? Look.coral : Kirigami.Theme.highlightColor

    function buttons(held) {
        return (held & Qt.LeftButton ? 1 : 0) | (held & Qt.RightButton ? 2 : 0) | (held & Qt.MiddleButton ? 4 : 0);
    }

    function pulse(x, y) {
        ring.x = x - ring.width / 2;
        ring.y = y - ring.height / 2;
        burst.restart();
    }

    z: 10
    enabled: touch.testing
    hoverEnabled: true
    acceptedButtons: Qt.LeftButton | Qt.RightButton | Qt.MiddleButton
    onPositionChanged: mouse => touch.page.editor.touch(mouse.x / width, mouse.y / height, touch.buttons(mouse.buttons))
    onPressed: mouse => {
        touch.page.editor.touch(mouse.x / width, mouse.y / height, touch.buttons(mouse.buttons));
        touch.pulse(mouse.x, mouse.y);
    }
    onReleased: mouse => touch.page.editor.touch(mouse.x / width, mouse.y / height, touch.buttons(mouse.buttons))
    onExited: touch.page.editor.touch(-1, -1, 0)

    Rectangle {
        id: mark

        visible: touch.testing && touch.containsMouse
        x: touch.mouseX - width / 2
        y: touch.mouseY - height / 2
        width: Kirigami.Units.gridUnit
        height: width
        radius: width / 2
        color: "transparent"
        border.width: 2
        border.color: touch.tint

        Rectangle {
            anchors.centerIn: parent
            width: 4
            height: 4
            radius: 2
            color: touch.tint
        }
    }

    Rectangle {
        id: ring

        width: Kirigami.Units.gridUnit * 2
        height: width
        radius: width / 2
        color: "transparent"
        border.width: 3
        border.color: Look.coral
        opacity: 0

        ParallelAnimation {
            id: burst

            NumberAnimation {
                target: ring
                property: "scale"
                from: 0.3
                to: 1.6
                duration: 450
                easing.type: Easing.OutCubic
            }

            NumberAnimation {
                target: ring
                property: "opacity"
                from: 1
                to: 0
                duration: 450
            }
        }
    }
}
