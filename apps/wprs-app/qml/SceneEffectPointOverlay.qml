import QtQuick
import org.kde.kirigami as Kirigami

Item {
    id: overlay

    required property var page
    required property real ratio
    readonly property var found: page.editor.selected.match(/^(.*\/layers\/\d+)\/effects\/(\d+)(\/params(\/.*)?)?$/)
    readonly property var gizmo: found ? (page.view.gizmos ?? []).find(g => g.pointer === found[1]) ?? null : null
    readonly property string base: found ? found[1] + "/effects/" + found[2] : ""
    readonly property string effect: found ? String(JSON.parse(page.editor.value(overlay.base + "/effect") || "null") ?? "") : ""
    readonly property var described: (page.view.effect_catalog ?? []).find(e => e.id === overlay.effect) ?? null
    readonly property var points: overlay.described?.points ?? []
    readonly property bool quad: overlay.described?.quad ?? false

    function at(point) {
        const raw = JSON.parse(overlay.page.editor.value(overlay.base + "/params/" + point.id) || "null");
        if (Array.isArray(raw) && raw.length >= 2)
            return [Number(raw[0]), Number(raw[1])];
        if (typeof raw === "number")
            return [raw, raw];
        return point.default;
    }

    function put(id, value) {
        overlay.page.run("effects", {
                "set": {
                    "layer": overlay.gizmo.id,
                    "index": Number(overlay.found[2]),
                    "params": {
                        [id]: value.map(v => Math.round(v * 1000) / 1000)
                    }
                }
            }, true);
    }

    visible: gizmo !== null && points.length > 0

    Item {
        id: frame

        readonly property var g: overlay.gizmo ?? ({
                "offset": [0, 0],
                "position": [0, 0],
                "size": [0, 0],
                "scale": [1, 1],
                "rotation": 0
            })
        readonly property var center: [g.offset[0] + g.position[0], g.offset[1] + g.position[1]]

        x: center[0] * overlay.ratio - width / 2
        y: center[1] * overlay.ratio - height / 2
        width: Math.max(1, Math.abs(g.size[0] * g.scale[0]) * overlay.ratio)
        height: Math.max(1, Math.abs(g.size[1] * g.scale[1]) * overlay.ratio)
        rotation: g.rotation

        Canvas {
            id: outline

            anchors.fill: parent
            visible: overlay.quad
            onPaint: {
                const ctx = getContext("2d");
                ctx.reset();
                ctx.strokeStyle = Kirigami.Theme.highlightColor;
                ctx.lineWidth = 2;
                ctx.beginPath();
                overlay.points.forEach((point, i) => {
                        const xy = overlay.at(point);
                        i === 0 ? ctx.moveTo(xy[0] * width, xy[1] * height) : ctx.lineTo(xy[0] * width, xy[1] * height);
                    });
                ctx.closePath();
                ctx.stroke();
            }
        }

        Connections {
            target: overlay
            function onPointsChanged() {
                outline.requestPaint();
            }
        }

        Repeater {
            model: overlay.points

            delegate: Rectangle {
                id: knob

                required property var modelData
                readonly property var frac: overlay.at(knob.modelData)

                x: frac[0] * frame.width - width / 2
                y: frac[1] * frame.height - height / 2
                width: Kirigami.Units.gridUnit * 0.8
                height: width
                radius: width / 2
                color: Kirigami.Theme.highlightColor
                border.color: Kirigami.Theme.backgroundColor
                border.width: 2

                MouseArea {
                    anchors.fill: parent
                    anchors.margins: -Kirigami.Units.smallSpacing
                    cursorShape: Qt.SizeAllCursor
                    onPositionChanged: mouse => {
                        const p = mapToItem(frame, mouse.x, mouse.y);
                        overlay.put(knob.modelData.id, [p.x / frame.width, p.y / frame.height]);
                        outline.requestPaint();
                    }
                }
            }
        }
    }
}
