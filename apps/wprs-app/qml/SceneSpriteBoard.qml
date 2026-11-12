import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: board

    required property var sheet
    property int selected: -1
    property int target: -1
    property var draft: null
    property string mode: ""
    property var origin: [0, 0, 0, 0]
    property var start: [0, 0]
    readonly property var size: sheet?.size ?? [1, 1]
    readonly property real fit: Math.max(0.01, Math.min(width / size[0], height / size[1]))
    readonly property var rects: sheet?.rects ?? []
    readonly property var live: draft ?? [0, 0, 0, 0]

    signal pick(int index)
    signal moved(int index, var rect)
    signal drawn(var rect)

    function clamp(rect) {
        const w = Math.max(1, Math.min(size[0], Math.round(rect[2])));
        const h = Math.max(1, Math.min(size[1], Math.round(rect[3])));
        return [Math.max(0, Math.min(size[0] - w, Math.round(rect[0]))), Math.max(0, Math.min(size[1] - h, Math.round(rect[1]))), w, h];
    }

    function hit(x, y) {
        for (let i = rects.length - 1; i >= 0; i--) {
            const r = rects[i];
            if (x >= r[0] && y >= r[1] && x < r[0] + r[2] && y < r[1] + r[3])
                return i;
        }
        return -1;
    }

    function corner(x, y) {
        const r = rects[selected];
        const reach = Kirigami.Units.gridUnit / fit;
        return r !== undefined && Math.abs(x - (r[0] + r[2])) < reach && Math.abs(y - (r[1] + r[3])) < reach;
    }

    function drag(x, y) {
        const dx = x - start[0];
        const dy = y - start[1];
        const o = origin;
        if (mode === "move")
            draft = clamp([o[0] + dx, o[1] + dy, o[2], o[3]]);
        else if (mode === "size")
            draft = clamp([o[0], o[1], Math.min(size[0] - o[0], o[2] + dx), Math.min(size[1] - o[1], o[3] + dy)]);
        else if (mode === "draw")
            draft = clamp([Math.min(start[0], x), Math.min(start[1], y), Math.abs(dx), Math.abs(dy)]);
    }

    Item {
        anchors.centerIn: parent
        width: board.size[0] * board.fit
        height: board.size[1] * board.fit

        Rectangle {
            anchors.fill: parent
            color: Kirigami.Theme.alternateBackgroundColor
        }

        Image {
            anchors.fill: parent
            source: board.sheet ? App.url(board.sheet.file) : ""
            fillMode: Image.Stretch
            smooth: false
        }

        Repeater {
            model: board.rects

            delegate: Rectangle {
                required property var modelData
                required property int index

                visible: !(board.draft && board.selected === index && board.mode !== "draw")
                x: modelData[0] * board.fit
                y: modelData[1] * board.fit
                width: modelData[2] * board.fit
                height: modelData[3] * board.fit
                color: board.selected === index ? Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.25) : "transparent"
                border.color: board.selected === index ? Kirigami.Theme.highlightColor : Kirigami.Theme.positiveTextColor
                border.width: 1

                QQC2.Label {
                    anchors.left: parent.left
                    anchors.top: parent.top
                    padding: 1
                    text: parent.index + 1
                    font.pixelSize: Math.min(parent.height, Kirigami.Units.gridUnit)
                    color: parent.border.color
                }
            }
        }

        Rectangle {
            visible: board.draft !== null
            x: board.live[0] * board.fit
            y: board.live[1] * board.fit
            width: board.live[2] * board.fit
            height: board.live[3] * board.fit
            color: "transparent"
            border.color: Kirigami.Theme.neutralTextColor
            border.width: 2
        }

        Rectangle {
            visible: board.selected >= 0 && board.selected < board.rects.length
            x: ((board.draft ?? board.rects[board.selected] ?? [0, 0, 0, 0])[0] + (board.draft ?? board.rects[board.selected] ?? [0, 0, 0, 0])[2]) * board.fit - width / 2
            y: ((board.draft ?? board.rects[board.selected] ?? [0, 0, 0, 0])[1] + (board.draft ?? board.rects[board.selected] ?? [0, 0, 0, 0])[3]) * board.fit - height / 2
            width: Kirigami.Units.gridUnit * 0.7
            height: width
            color: Kirigami.Theme.highlightColor
        }

        MouseArea {
            anchors.fill: parent
            onPressed: mouse => {
                const x = mouse.x / board.fit;
                const y = mouse.y / board.fit;
                board.start = [x, y];
                if (board.corner(x, y)) {
                    board.mode = "size";
                    board.target = board.selected;
                    board.origin = board.rects[board.selected];
                    return;
                }
                const found = board.hit(x, y);
                board.target = found;
                board.pick(found);
                board.mode = found >= 0 ? "move" : "draw";
                board.origin = found >= 0 ? board.rects[found] : [x, y, 0, 0];
            }
            onPositionChanged: mouse => board.drag(mouse.x / board.fit, mouse.y / board.fit)
            onReleased: {
                const done = board.draft;
                const mode = board.mode;
                board.draft = null;
                board.mode = "";
                if (!done || done[2] < 2 || done[3] < 2)
                    return;
                if (mode === "draw")
                    board.drawn(done);
                else
                    board.moved(board.target, done);
            }
        }
    }
}
