import QtQuick

MouseArea {
    id: touch

    required property var page
    required property bool testing

    function buttons(held) {
        return (held & Qt.LeftButton ? 1 : 0) | (held & Qt.RightButton ? 2 : 0) | (held & Qt.MiddleButton ? 4 : 0);
    }

    z: 10
    enabled: touch.testing
    hoverEnabled: true
    acceptedButtons: Qt.LeftButton | Qt.RightButton | Qt.MiddleButton
    onPositionChanged: mouse => touch.page.editor.touch(mouse.x / width, mouse.y / height, touch.buttons(mouse.buttons))
    onPressed: mouse => touch.page.editor.touch(mouse.x / width, mouse.y / height, touch.buttons(mouse.buttons))
    onReleased: mouse => touch.page.editor.touch(mouse.x / width, mouse.y / height, touch.buttons(mouse.buttons))
    onExited: touch.page.editor.touch(-1, -1, 0)
}
