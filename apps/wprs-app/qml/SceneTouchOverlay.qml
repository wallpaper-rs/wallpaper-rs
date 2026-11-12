import QtQuick
import org.kde.kirigami as Kirigami

Item {
    id: overlay

    required property var page
    required property real ratio
    property int revision: 0
    readonly property var areas: (revision, JSON.parse(page.editor.rig("", JSON.stringify({
        "touch": true
    })) || "null")?.layers ?? [])
    readonly property var chosen: areas.find(a => a.pointer === page.editor.selected) ?? null

    visible: chosen !== null

    Connections {
        function onViewChanged() {
            overlay.revision++;
        }

        target: overlay.page.editor
    }

    Repeater {
        model: overlay.chosen ? overlay.areas : []

        delegate: Item {
            id: area

            required property var modelData
            readonly property var g: (overlay.page.view.gizmos ?? []).find(g => g.pointer === modelData.pointer) ?? null
            readonly property bool current: modelData.pointer === overlay.chosen?.pointer
            readonly property var center: g ? [g.offset[0] + g.position[0], g.offset[1] + g.position[1]] : [0, 0]

            visible: g !== null && modelData.takes !== "off"
            x: center[0] * overlay.ratio - width / 2
            y: center[1] * overlay.ratio - height / 2
            width: g ? Math.max(1, Math.abs(g.size[0] * g.scale[0]) * overlay.ratio) : 0
            height: g ? Math.max(1, Math.abs(g.size[1] * g.scale[1]) * overlay.ratio) : 0
            rotation: g ? g.rotation : 0

            Rectangle {
                anchors.fill: parent
                color: Kirigami.Theme.highlightColor
                opacity: area.current ? 0.18 : 0.06
            }

            Canvas {
                readonly property string mode: area.modelData.takes
                readonly property bool lit: area.current

                anchors.fill: parent
                onModeChanged: requestPaint()
                onLitChanged: requestPaint()
                onWidthChanged: requestPaint()
                onHeightChanged: requestPaint()
                onPaint: {
                    const ctx = getContext("2d");
                    ctx.reset();
                    ctx.strokeStyle = Kirigami.Theme.highlightColor;
                    ctx.lineWidth = area.current ? 2 : 1;
                    ctx.setLineDash(area.modelData.takes === "alpha" ? [6, 4] : []);
                    ctx.strokeRect(1, 1, width - 2, height - 2);
                }
            }

            Rectangle {
                visible: area.current && area.modelData.takes === "alpha"
                x: 4
                y: 4
                width: tag.implicitWidth + 8
                height: tag.implicitHeight + 4
                radius: 3
                color: Kirigami.Theme.backgroundColor
                opacity: 0.85

                Text {
                    id: tag

                    anchors.centerIn: parent
                    color: Kirigami.Theme.textColor
                    font.pixelSize: Kirigami.Theme.smallFont.pixelSize
                    text: Math.round(area.modelData.alpha * 100) + "%"
                }
            }
        }
    }
}
