import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: graph

    required property var page
    property var sources: []
    property real now: 0
    property var shown: []
    property var hidden: ({})
    property var chosen: []
    readonly property var copied: SceneGraphSources.copied
    property bool aligned: true
    readonly property var watch: page.view
    readonly property var span: {
        const all = shown.map(c => c.range ?? [0, 1]);
        return all.length ? [Math.min.apply(null, all.map(r => r[0])), Math.max.apply(null, all.map(r => r[1]))] : [0, 1];
    }

    function named(part) {
        const names = {
            "value": App.tr("Value"),
            "x": "X",
            "y": "Y",
            "z": "Z",
            "red": App.tr("Red"),
            "green": App.tr("Green"),
            "blue": App.tr("Blue"),
            "alpha": App.tr("Alpha")
        };
        return names[part] ?? part;
    }

    function tint(s, part) {
        const colors = [Kirigami.Theme.negativeTextColor, Kirigami.Theme.positiveTextColor, Kirigami.Theme.linkColor, Kirigami.Theme.neutralTextColor];
        return (shown[s]?.tracks.length ?? 0) > 1 ? colors[part % 4] : Kirigami.Theme.highlightColor;
    }

    function load() {
        shown = sources.map(s => JSON.parse(page.editor.rig("", JSON.stringify({
                "curves": Object.assign({
                    "samples": 160
                }, s)
            })) || "{}")).map((c, i) => Object.assign(c, {
                    "label": sources[i].label
                })).filter(c => c.list !== undefined);
        chosen = chosen.filter(c => shown[c.s]?.keys.some(k => Math.abs(k.t - c.t) < 1e-3));
    }

    function picked(s, t) {
        return chosen.some(c => c.s === s && Math.abs(c.t - t) < 1e-3);
    }

    function choose(s, t, adding) {
        const rest = adding ? chosen.filter(c => !(c.s === s && Math.abs(c.t - t) < 1e-3)) : [];
        chosen = adding && rest.length < chosen.length ? rest : rest.concat([{
                "s": s,
                "t": t
            }]);
    }

    function picks() {
        const out = [];
        chosen.forEach(c => {
            const pointer = shown[c.s].pointer;
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
        page.run("edit_keys", Object.assign({
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
        const before = page.view.history?.done?.length ?? 0;
        act(args);
        if ((page.view.history?.done?.length ?? 0) !== before)
            chosen = chosen.map(c => ({
                        "s": c.s,
                        "t": Math.round((pivot + (c.t - pivot) * scale + shift) * 1000) / 1000
                    }));
    }

    function bend(s, t, side, pair) {
        const args = {
            "picks": [{
                    "pointer": shown[s].pointer,
                    "times": [t]
                }],
            "aligned": aligned
        };
        args[side + "_handle"] = pair.map(n => Math.round(n * 1e6) / 1e6);
        page.run("edit_keys", args, true);
    }

    function raw(s) {
        return JSON.parse(page.editor.value(shown[s].list) || "[]");
    }

    function copy() {
        SceneGraphSources.copied = picks().map(p => {
            const s = shown.findIndex(c => c.pointer === p.pointer);
            return {
                "pointer": p.pointer,
                "keys": raw(s).filter(k => p.times.some(t => Math.abs(k[0] - t) < 1e-3))
            };
        });
    }

    function paste() {
        const lone = copied.length === 1 && shown.length > 0 && !shown.some(c => c.pointer === copied[0].pointer);
        const target = lone ? (chosen.length ? shown[chosen[0].s].pointer : shown[0].pointer) : null;
        page.run("edit_keys", {
            "picks": copied.map(c => ({
                        "pointer": target ?? c.pointer,
                        "keys": c.keys
                    })),
            "at": Math.round((now + (shown[0]?.offset ?? 0)) * 1000) / 1000
        });
    }

    title: App.tr("Curve editor")
    standardButtons: Kirigami.Dialog.Close
    preferredWidth: Kirigami.Units.gridUnit * 40
    onOpened: load()
    onWatchChanged: visible && load()
    onSourcesChanged: {
        chosen = [];
        if (visible)
            load();
    }

    ColumnLayout {
        spacing: Kirigami.Units.smallSpacing

        SceneGraphTools {
            Layout.fillWidth: true
            graph: graph
        }

        SceneGraphPlot {
            id: plot

            Layout.fillWidth: true
            Layout.preferredHeight: Kirigami.Units.gridUnit * 18
            graph: graph
        }

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            color: Kirigami.Theme.disabledTextColor
            text: graph.shown.length === 0 ? App.tr("No keys to show: add keyframes to a value first.") : App.tr("Click a key to pick it, Ctrl+click to add it, drag around keys to pick several. Drag picked keys to move them in time and value, the grip at the right of the selection to stretch their times, and the round handles to bend the curve. Each track is drawn on its own scale.")
        }
    }
}
