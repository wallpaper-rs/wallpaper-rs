import QtQuick
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Rectangle {
    id: picture

    required property var board
    readonly property var strip: board.strip
    readonly property int columns: Math.max(1, strip?.[0] ?? 1)

    function frame() {
        const r = board.rect;
        handle.x = r.x * width;
        handle.y = r.y * height;
        handle.width = r.w * width;
        handle.height = r.h * height;
    }

    Layout.fillWidth: true
    Layout.maximumWidth: Kirigami.Units.gridUnit * 30
    Layout.preferredHeight: width * (strip ? strip[1] / strip[0] : 0.25)
    color: Kirigami.Theme.alternateBackgroundColor
    border.color: Kirigami.Theme.disabledTextColor
    clip: true
    Accessible.name: App.tr("The picture the lights take their colors from")
    onWidthChanged: frame()
    onHeightChanged: frame()

    Connections {
        target: picture.board

        function onRectChanged() {
            picture.frame();
        }
    }

    Grid {
        anchors.fill: parent
        columns: picture.columns

        Repeater {
            model: picture.strip ? picture.strip[0] * picture.strip[1] : 0

            delegate: Rectangle {
                required property int index

                width: picture.width / picture.columns
                height: picture.height / Math.max(1, picture.strip[1])
                color: Qt.rgba(picture.strip[2][index * 3] / 255, picture.strip[2][index * 3 + 1] / 255, picture.strip[2][index * 3 + 2] / 255, 1)
            }
        }
    }

    Repeater {
        model: picture.board.targets.filter(t => t !== picture.board.target && picture.board.parse(String(picture.board.places[t.key] ?? "")))

        delegate: Rectangle {
            required property var modelData
            readonly property var r: picture.board.parse(String(picture.board.places[modelData.key]))

            x: r.x * picture.width
            y: r.y * picture.height
            width: r.w * picture.width
            height: r.h * picture.height
            color: "transparent"
            border.color: Kirigami.Theme.textColor
            opacity: 0.5

            Text {
                anchors.fill: parent
                anchors.margins: 2
                text: parent.modelData.text
                textFormat: Text.PlainText
                elide: Text.ElideRight
                font.pixelSize: Kirigami.Units.gridUnit * 0.6
                color: Kirigami.Theme.textColor
            }
        }
    }

    Rectangle {
        id: handle

        readonly property bool keyTarget: true

        visible: picture.board.target !== null && picture.board.value !== "off"
        activeFocusOnTab: true
        Accessible.role: Accessible.Grouping
        Accessible.name: App.tr("Frame of the device on the picture; arrow keys move it, Shift with arrows sizes it")
        Keys.onPressed: event => {
            const step = {
                [Qt.Key_Left]: [-1, 0],
                [Qt.Key_Right]: [1, 0],
                [Qt.Key_Up]: [0, -1],
                [Qt.Key_Down]: [0, 1]
            }[event.key];
            if (!step)
                return;
            const r = picture.board.rect, d = 0.02;
            if (event.modifiers & Qt.ShiftModifier)
                picture.board.put(r.x, r.y, r.w + step[0] * d, r.h + step[1] * d, r.flow);
            else
                picture.board.put(r.x + step[0] * d, r.y + step[1] * d, r.w, r.h, r.flow);
            event.accepted = true;
        }
        color: Qt.alpha(Kirigami.Theme.highlightColor, 0.25)
        border.color: activeFocus ? Kirigami.Theme.focusColor : Kirigami.Theme.highlightColor
        border.width: activeFocus ? 4 : 2
        Component.onCompleted: picture.frame()

        function done() {
            picture.board.put(x / picture.width, y / picture.height, width / picture.width, height / picture.height, picture.board.rect.flow);
        }

        MouseArea {
            anchors.fill: parent
            cursorShape: Qt.SizeAllCursor
            drag.target: handle
            drag.minimumX: 0
            drag.minimumY: 0
            drag.maximumX: picture.width - handle.width
            drag.maximumY: picture.height - handle.height
            onReleased: handle.done()
        }

        Rectangle {
            anchors.right: parent.right
            anchors.bottom: parent.bottom
            width: Kirigami.Units.gridUnit * 0.8
            height: width
            color: Kirigami.Theme.highlightColor
            Accessible.name: App.tr("Size of the frame")

            MouseArea {
                anchors.fill: parent
                cursorShape: Qt.SizeFDiagCursor
                onPositionChanged: mouse => {
                    const at = mapToItem(picture, mouse.x, mouse.y);
                    handle.width = Math.max(8, Math.min(picture.width - handle.x, at.x - handle.x));
                    handle.height = Math.max(8, Math.min(picture.height - handle.y, at.y - handle.y));
                }
                onReleased: handle.done()
            }
        }
    }
}
