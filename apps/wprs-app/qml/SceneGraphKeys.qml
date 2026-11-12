import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var graph

    function picks() {
        const out = [];
        graph.chosen.forEach(c => {
            const pointer = graph.shown[c.s].pointer;
            const pick = out.find(p => p.pointer === pointer);
            if (pick)
                pick.times.push(c.t);
            else
                out.push({
                    "pointer": pointer,
                    "times": [c.t]
                });
        });
        return out;
    }

    function act(args, gesture) {
        graph.page.run("edit_keys", Object.assign({
            "picks": picks()
        }, args), gesture === true);
    }

    function move(shift, scale, pivot, lift, part) {
        const args = {
            "shift": Math.round(shift * 1000) / 1000,
            "scale": scale,
            "pivot": pivot
        };
        if (lift !== 0) {
            args.lift = lift;
            if (part !== null)
                args.part = part;
        }
        const before = graph.page.view.history?.done?.length ?? 0;
        act(args);
        if ((graph.page.view.history?.done?.length ?? 0) !== before)
            graph.chosen = graph.chosen.map(c => ({
                        "s": c.s,
                        "t": Math.round((pivot + (c.t - pivot) * scale + shift) * 1000) / 1000
                    }));
    }

    function alike(s) {
        const c = graph.shown[s];
        return graph.chosen.every(o => {
            const d = graph.shown[o.s];
            return d.kind === c.kind && d.unit === c.unit && d.tracks.length === c.tracks.length;
        });
    }

    function middle(p) {
        const all = graph.chosen.map(c => graph.shown[c.s].keys.find(k => Math.abs(k.t - c.t) < 1e-3)?.parts?.[p]).filter(v => v !== undefined);
        return all.length ? (Math.min.apply(null, all) + Math.max.apply(null, all)) / 2 : 0;
    }

    function rescale(factor, pivot, part) {
        const args = {
            "value_scale": Math.round(factor * 1e6) / 1e6
        };
        if (pivot !== null)
            args.value_pivot = Math.round(pivot * 1e6) / 1e6;
        if (part !== null)
            args.part = part;
        act(args);
    }

    function bend(s, t, side, pair, part) {
        const args = {
            "picks": [{
                    "pointer": graph.shown[s].pointer,
                    "times": [t]
                }],
            "aligned": graph.aligned
        };
        if (part !== undefined && part !== null)
            args.part = part;
        args[side + "_handle"] = pair.map(n => Math.round(n * 1e6) / 1e6);
        graph.page.run("edit_keys", args, true);
    }

    function split(s, t) {
        const k = graph.shown[s]?.keys.find(k => Math.abs(k.t - t) < 1e-3);
        return (k?.in?.split ?? false) || (k?.out?.split ?? false);
    }

    function link() {
        act({
            "link": true
        });
    }

    function raw(s) {
        return JSON.parse(graph.page.editor.value(graph.shown[s].list) || "[]");
    }

    function copy() {
        SceneGraphSources.copied = picks().map(p => {
            const s = graph.shown.findIndex(c => c.pointer === p.pointer);
            return {
                "pointer": p.pointer,
                "keys": raw(s).filter(k => p.times.some(t => Math.abs(k[0] - t) < 1e-3))
            };
        });
    }

    function paste() {
        const lone = graph.copied.length === 1 && graph.shown.length > 0 && !graph.shown.some(c => c.pointer === graph.copied[0].pointer);
        const target = lone ? (graph.chosen.length ? graph.shown[graph.chosen[0].s].pointer : graph.shown[0].pointer) : null;
        graph.page.run("edit_keys", {
            "picks": graph.copied.map(c => ({
                        "pointer": target ?? c.pointer,
                        "keys": c.keys
                    })),
            "at": Math.round((graph.now + (graph.shown[0]?.offset ?? 0)) * 1000) / 1000
        });
    }
}
