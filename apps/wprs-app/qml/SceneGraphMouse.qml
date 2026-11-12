import QtQuick

MouseArea {
    id: mouse

    required property var plot
    readonly property var graph: plot.graph
    property string mode: ""
    property var held: null
    property real x0: 0
    property real y0: 0
    property real delta: 0
    property real factor: 1
    property real mid: 0

    function touches(tr) {
        return held?.lift === true && (graph.shown[held.s].tracks.length > 1 ? tr.p === held.p : true);
    }

    function changed(v) {
        return held.values ? mid + (v - mid) * factor : v + delta;
    }

    function near(a, b) {
        return Math.hypot(a[0] - b[0], a[1] - b[1]) <= 7;
    }

    function hit(px, py) {
        for (const tr of plot.tracks) {
            const handle = plot.handles(tr).find(hd => near(hd.to, [px, py]));
            if (handle)
                return {
                    "mode": "handle",
                    "s": tr.s,
                    "p": tr.p,
                    "t": handle.k.t,
                    "side": handle.side
                };
        }
        const box = plot.box;
        if (box && Math.abs(px - plot.tx(box[1])) <= 6 && py <= plot.pad + 2)
            return {
                "mode": "scale"
            };
        for (const tr of plot.tracks) {
            const k = tr.keys.find(k => k.parts && near(plot.shifted(tr, k), [px, py]));
            if (k)
                return {
                    "mode": "move",
                    "s": tr.s,
                    "p": tr.p,
                    "t": k.t
                };
        }
        return {
            "mode": "band"
        };
    }

    function bend(px, py) {
        const tr = plot.tracks.find(tr => tr.s === held.s && tr.p === held.p);
        const k = tr?.keys.find(k => Math.abs(k.t - held.t) < 1e-3);
        const h = k?.[held.side];
        if (!h)
            return;
        const sign = held.side === "out" ? 1 : -1;
        const dt = Math.max(0, Math.min(h.room, sign * (plot.time(px) - k.t)));
        const rise = sign * (plot.value(tr, py) - k.parts[tr.p]);
        if (!graph.linked && graph.shown[held.s].tracks.length > 1) {
            graph.ops.bend(held.s, held.t, held.side, [dt, rise], tr.p);
            return;
        }
        const gain = h.gains[tr.p] ?? 1;
        const dv = Math.abs(gain) < 1e-9 ? h.dv : rise / gain;
        graph.ops.bend(held.s, held.t, held.side, [dt, dv]);
    }

    function finish() {
        const tr = held ? plot.tracks.find(tr => tr.s === held.s && tr.p === held.p) : null;
        if (mode === "handle")
            graph.page.editor.hold(false);
        else if (mode === "move" && (plot.dx !== 0 || plot.dy !== 0)) {
            const parts = graph.shown[held.s].tracks.length;
            if (held.values && factor !== 1)
                graph.ops.rescale(factor, mid, parts > 1 ? held.p : null);
            else if (!held.values)
                graph.ops.move(plot.time(x0 + plot.dx) - plot.time(x0), 1, 0, held.lift ? delta : 0, parts > 1 ? held.p : null);
        } else if (mode === "move" && !held.adding)
            graph.chosen = [{
                    "s": held.s,
                    "t": held.t
                }];
        else if (mode === "scale" && plot.stretch !== 1)
            graph.ops.move(0, Math.round(plot.stretch * 1000) / 1000, plot.box[0], 0, null);
        else if (mode === "band" && plot.band) {
            const band = plot.band;
            const l = Math.min(band[0], band[2]), t = Math.min(band[1], band[3]), r = Math.max(band[0], band[2]), b = Math.max(band[1], band[3]);
            const inside = plot.tracks.flatMap(tr => tr.keys.filter(k => {
                    const at = k.parts ? plot.shifted(tr, k) : [-1, -1];
                    return at[0] >= l && at[0] <= r && at[1] >= t && at[1] <= b;
                }).map(k => ({
                        "s": tr.s,
                        "t": k.t
                    })));
            const base = held.adding ? graph.chosen : [];
            graph.chosen = base.concat(inside.filter(c => !base.some(b => b.s === c.s && Math.abs(b.t - c.t) < 1e-3)));
        } else if (mode === "band" && !held.adding)
            graph.chosen = [];
        plot.dx = 0;
        plot.dy = 0;
        delta = 0;
        factor = 1;
        plot.stretch = 1;
        plot.band = null;
        mode = "";
        held = null;
    }

    acceptedButtons: Qt.LeftButton | Qt.MiddleButton
    onPressed: event => {
        x0 = event.x;
        y0 = event.y;
        if (event.button === Qt.MiddleButton) {
            mode = "pan";
            held = {
                "mode": "pan",
                "span": plot.times,
                "win": plot.win
            };
            return;
        }
        held = hit(event.x, event.y);
        held.adding = (event.modifiers & (Qt.ControlModifier | Qt.ShiftModifier)) !== 0;
        mode = held.mode;
        if (mode === "handle")
            graph.page.editor.hold(true);
        if (mode === "move" && (held.adding || !graph.picked(held.s, held.t)))
            graph.choose(held.s, held.t, held.adding);
        held.lift = mode === "move" && graph.ops.alike(held.s);
        held.values = held.lift && (event.modifiers & Qt.AltModifier) !== 0;
        if (held.values)
            mid = graph.ops.middle(held.p);
    }
    onPositionChanged: event => {
        if (mode === "handle")
            bend(event.x, event.y);
        else if (mode === "move") {
            const tr = plot.tracks.find(tr => tr.s === held.s && tr.p === held.p);
            const v0 = graph.shown[held.s].keys.find(k => Math.abs(k.t - held.t) < 1e-3)?.parts?.[held.p] ?? mid;
            plot.dx = held.values ? 0 : event.x - x0;
            plot.dy = held.lift && Math.abs(event.y - y0) > 4 ? event.y - y0 : 0;
            delta = plot.dy !== 0 && tr ? plot.value(tr, y0 + plot.dy) - plot.value(tr, y0) : 0;
            factor = held.values && plot.dy !== 0 && tr && Math.abs(v0 - mid) > 1e-9 ? (plot.value(tr, plot.vy(tr, v0) + plot.dy) - mid) / (v0 - mid) : 1;
        } else if (mode === "scale" && plot.box)
            plot.stretch = Math.max(0.05, (plot.time(event.x) - plot.box[0]) / Math.max(1e-6, plot.box[1] - plot.box[0]));
        else if (mode === "band")
            plot.band = [x0, y0, event.x, event.y];
        else if (mode === "pan")
            plot.view.pan(held.span, held.win, event.x - x0, event.y - y0);
    }
    onWheel: wheel => {
        const step = wheel.angleDelta.y !== 0 ? wheel.angleDelta.y : wheel.angleDelta.x;
        if (step === 0)
            return;
        const across = plot.width - 2 * plot.pad;
        if ((wheel.modifiers & Qt.ShiftModifier) !== 0 || wheel.angleDelta.y === 0)
            plot.view.pan(plot.times, plot.win, step > 0 ? across / 10 : -across / 10, 0);
        else
            plot.view.zoom(wheel.x, wheel.y, step > 0 ? 0.8 : 1.25, (wheel.modifiers & Qt.ControlModifier) !== 0);
    }
    onReleased: finish()
    onCanceled: finish()
}
