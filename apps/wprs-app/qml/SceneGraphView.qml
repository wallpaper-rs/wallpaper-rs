import QtQuick

QtObject {
    required property var plot
    property var span: null
    property var win: [0, 1]

    function fitAll() {
        span = null;
        win = [0, 1];
    }

    function fitPicked() {
        let t0 = Infinity, t1 = -Infinity, n0 = Infinity, n1 = -Infinity;
        for (const found of plot.graph.ops.framed()) {
            const b = found.bounds?.picked;
            if (!b)
                continue;
            t0 = Math.min(t0, b.range[0]);
            t1 = Math.max(t1, b.range[1]);
            for (const tr of plot.tracks.filter(tr => tr.s === found.s && b.parts[tr.p])) {
                n0 = Math.min(n0, (b.parts[tr.p][0] - tr.lo) / (tr.hi - tr.lo));
                n1 = Math.max(n1, (b.parts[tr.p][1] - tr.lo) / (tr.hi - tr.lo));
            }
        }
        if (!isFinite(t0))
            return;
        const dt = Math.max(t1 - t0, 0.2), mt = (t0 + t1) / 2;
        span = [mt - dt * 0.6, mt + dt * 0.6];
        if (isFinite(n0)) {
            const dn = Math.max(n1 - n0, 0.05), mn = (n0 + n1) / 2;
            win = [mn - dn * 0.6, mn + dn * 0.6];
        }
    }

    function zoom(px, py, factor, inValue) {
        if (inValue) {
            const n = share(py);
            win = [n - (n - win[0]) * factor, n + (win[1] - n) * factor];
        } else {
            const t = plot.time(px);
            span = [t - (t - plot.times[0]) * factor, t + (plot.times[1] - t) * factor];
        }
    }

    function pan(from, toward, dpx, dpy) {
        const dt = dpx / Math.max(1, plot.width - 2 * plot.pad) * (from[1] - from[0]);
        const dn = dpy / Math.max(1, plot.height - 2 * plot.pad) * (toward[1] - toward[0]);
        span = [from[0] - dt, from[1] - dt];
        win = [toward[0] + dn, toward[1] + dn];
    }

    function share(py) {
        return win[0] + (1 - (py - plot.pad) / Math.max(1, plot.height - 2 * plot.pad)) * (win[1] - win[0]);
    }
}
