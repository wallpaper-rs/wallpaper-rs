import QtQuick
import org.kde.kirigami as Kirigami

Canvas {
    id: plot

    required property var graph
    readonly property real pad: Kirigami.Units.gridUnit
    readonly property var tracks: build(graph.shown, graph.hidden, graph.chosen)
    readonly property var chosenTimes: graph.chosen.map(c => c.t)
    readonly property var box: chosenTimes.length > 1 ? [Math.min.apply(null, chosenTimes), Math.max.apply(null, chosenTimes)] : null
    readonly property alias mouse: mouse
    property real dx: 0
    property real dy: 0
    property real stretch: 1
    property var band: null
    readonly property var watched: [tracks, graph.chosen, graph.now, dx, dy, stretch, band, width, height]

    function build(shown, hidden, chosen) {
        const out = [];
        shown.forEach((c, s) => c.tracks.forEach(tr => {
                    if (!hidden[s + ":" + tr.part])
                        out.push(fit({
                            "s": s,
                            "p": tr.part,
                            "points": tr.points,
                            "keys": c.keys
                        }));
                }));
        return out;
    }

    function fit(tr) {
        const all = tr.points.map(pt => pt[1]).concat(tr.keys.map(k => k.parts?.[tr.p]).filter(v => v !== undefined));
        tr.keys.filter(k => graph.picked(tr.s, k.t)).forEach(k => ["in", "out"].forEach(side => k[side]?.at?.[tr.p] && all.push(k[side].at[tr.p][1])));
        const lo = all.length ? Math.min.apply(null, all) : 0, hi = all.length ? Math.max.apply(null, all) : 1;
        const margin = Math.max(1e-3, (hi - lo) * 0.08);
        tr.lo = lo - margin;
        tr.hi = hi + margin;
        return tr;
    }

    function tx(t) {
        return pad + (t - graph.span[0]) / Math.max(1e-6, graph.span[1] - graph.span[0]) * (width - 2 * pad);
    }

    function time(px) {
        return graph.span[0] + (px - pad) / Math.max(1, width - 2 * pad) * (graph.span[1] - graph.span[0]);
    }

    function vy(tr, v) {
        return pad + (1 - (v - tr.lo) / (tr.hi - tr.lo)) * (height - 2 * pad);
    }

    function value(tr, py) {
        return tr.lo + (1 - (py - pad) / Math.max(1, height - 2 * pad)) * (tr.hi - tr.lo);
    }

    function shifted(tr, k) {
        const picked = graph.picked(tr.s, k.t) && (dx !== 0 || dy !== 0 || stretch !== 1);
        const pivot = box ? box[0] : k.t;
        const px = tx(pivot + (k.t - pivot) * (picked ? stretch : 1)) + (picked ? dx : 0);
        const lifted = picked && mouse.held?.lift && mouse.held.s === tr.s && mouse.held.p === tr.p;
        return [px, vy(tr, k.parts[tr.p]) + (lifted ? dy : 0)];
    }

    function handles(tr) {
        const out = [];
        tr.keys.filter(k => graph.picked(tr.s, k.t) && k.parts).forEach(k => ["in", "out"].forEach(side => {
                    const h = k[side];
                    if (h?.at?.[tr.p])
                        out.push({
                            "k": k,
                            "side": side,
                            "h": h,
                            "from": [tx(k.t), vy(tr, k.parts[tr.p])],
                            "to": [tx(h.at[tr.p][0]), vy(tr, h.at[tr.p][1])]
                        });
                }));
        return out;
    }

    onWatchedChanged: requestPaint()
    onPaint: {
        const ctx = getContext("2d");
        ctx.reset();
        ctx.fillStyle = Kirigami.Theme.alternateBackgroundColor;
        ctx.fillRect(0, 0, width, height);
        const now = tx(graph.now + (graph.shown[0]?.offset ?? 0));
        ctx.strokeStyle = Kirigami.Theme.negativeTextColor;
        ctx.beginPath();
        ctx.moveTo(now, 0);
        ctx.lineTo(now, height);
        ctx.stroke();
        for (const tr of tracks) {
            const tint = graph.tint(tr.s, tr.p);
            ctx.strokeStyle = tint;
            ctx.lineWidth = 2;
            ctx.beginPath();
            tr.points.forEach((pt, i) => i ? ctx.lineTo(tx(pt[0]), vy(tr, pt[1])) : ctx.moveTo(tx(pt[0]), vy(tr, pt[1])));
            ctx.stroke();
            ctx.lineWidth = 1;
            for (const hd of handles(tr)) {
                ctx.strokeStyle = Kirigami.Theme.textColor;
                ctx.setLineDash(hd.h.set ? [] : [3, 3]);
                ctx.beginPath();
                ctx.moveTo(hd.from[0], hd.from[1]);
                ctx.lineTo(hd.to[0], hd.to[1]);
                ctx.stroke();
                ctx.setLineDash([]);
                ctx.fillStyle = hd.h.set ? Kirigami.Theme.textColor : Kirigami.Theme.backgroundColor;
                ctx.beginPath();
                ctx.arc(hd.to[0], hd.to[1], 4, 0, 2 * Math.PI);
                ctx.fill();
                ctx.stroke();
            }
            for (const k of tr.keys.filter(k => k.parts)) {
                const at = shifted(tr, k), r = 5;
                ctx.fillStyle = graph.picked(tr.s, k.t) ? Kirigami.Theme.highlightColor : tint;
                ctx.beginPath();
                ctx.moveTo(at[0], at[1] - r);
                ctx.lineTo(at[0] + r, at[1]);
                ctx.lineTo(at[0], at[1] + r);
                ctx.lineTo(at[0] - r, at[1]);
                ctx.closePath();
                ctx.fill();
            }
        }
        if (box) {
            const right = tx(box[0] + (box[1] - box[0]) * stretch) + dx;
            ctx.strokeStyle = Kirigami.Theme.highlightColor;
            ctx.strokeRect(tx(box[0]) + dx, 2, right - tx(box[0]) - dx, height - 4);
            ctx.fillStyle = Kirigami.Theme.highlightColor;
            ctx.fillRect(right - 3, 2, 6, pad);
        }
        if (band) {
            ctx.strokeStyle = Kirigami.Theme.focusColor;
            ctx.strokeRect(band[0], band[1], band[2] - band[0], band[3] - band[1]);
        }
    }

    SceneGraphMouse {
        id: mouse

        anchors.fill: parent
        plot: plot
    }
}
