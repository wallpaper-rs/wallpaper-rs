import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: marks

    required property var lane
    readonly property var list: (lane.e.bursts ?? []).map((b, k) => ({
                "index": k,
                "burst": b
            })).filter(row => !row.burst.on)

    function write(k, key, value) {
        const next = (lane.e.bursts ?? []).map(b => Object.assign({}, b));
        if (value === null || (key === "at" && value <= 0))
            delete next[k][key];
        else
            next[k][key] = value;
        lane.emission.put({
                "bursts": next
            }, true);
    }

    function ticks(row, end) {
        const b = row.burst, first = lane.delay + (b.at ?? 0), last = Math.min(end, lane.span), every = b.every ?? 0;
        const out = [];
        if (first > last)
            return out;
        const step = every > 0 ? Math.max(every, 2 * lane.span / Math.max(1, width)) : lane.span + 1;
        for (let s = first; s <= last && out.length < 2000; s += step)
            out.push(s);
        return out;
    }

    function paint(ctx, h, end) {
        ctx.strokeStyle = Kirigami.Theme.positiveTextColor;
        ctx.lineWidth = 2;
        ctx.beginPath();
        list.forEach(row => ticks(row, end).forEach(s => {
                    ctx.moveTo(lane.place(s), 0);
                    ctx.lineTo(lane.place(s), h);
                }));
        ctx.stroke();
    }

    onListChanged: lane.repaint()

    SceneRepeater {
        id: knobs

        rows: marks.list.flatMap(row => [[row.index, "at", App.tr("Burst %1 starts after %2 s", row.index + 1, (row.burst.at ?? 0))]].concat((row.burst.every ?? 0) > 0 ? [[row.index, "every", App.tr("Burst %1 repeats every %2 s", row.index + 1, row.burst.every)]] : []))

        delegate: Rectangle {
            id: knob

            required property int index
            readonly property var modelData: knobs.rows[index]
            readonly property var burst: lane.e.bursts?.[modelData[0]] ?? ({})
            readonly property bool repeat: modelData[1] === "every"
            property real moved: -1

            x: (moved >= 0 ? moved : lane.place(lane.delay + (burst.at ?? 0) + (repeat ? burst.every ?? 0 : 0))) - width / 2
            y: repeat ? 0 : marks.height / 2 - height / 2
            width: Kirigami.Units.gridUnit * (repeat ? 0.6 : 0.8)
            height: width
            radius: repeat ? 2 : width / 2
            color: drag.active ? Kirigami.Theme.positiveTextColor : Kirigami.Theme.highlightColor
            border.color: Kirigami.Theme.backgroundColor

            QQC2.ToolTip.text: modelData[2]
            QQC2.ToolTip.visible: hover.hovered || drag.active

            HoverHandler {
                id: hover
            }

            DragHandler {
                id: drag

                target: null
                onActiveChanged: {
                    lane.holder.hold(active);
                    lane.held = active ? lane.fitted : 0;
                    if (!active)
                        knob.moved = -1;
                }
                onCentroidChanged: {
                    if (!active)
                        return;
                    knob.moved = Math.max(0, knob.mapToItem(marks, centroid.position.x, 0).x);
                    const at = lane.when(knob.moved), start = knob.burst.at ?? 0;
                    if (knob.repeat)
                        marks.write(knob.modelData[0], "every", Math.min(86400, Math.max(0.02, at - lane.delay - start)));
                    else
                        marks.write(knob.modelData[0], "at", Math.min(86400, Math.max(0, at - lane.delay)));
                }
            }
        }
    }
}
