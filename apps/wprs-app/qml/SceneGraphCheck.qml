import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function read(pointer) {
        return JSON.parse(page.editor.value(pointer) || "null");
    }

    function times(pointer) {
        return (read(pointer + "/keys") ?? []).map(k => k[0]);
    }

    function fail(what) {
        throw new Error("curve editor: " + what + " " + page.editor.error);
    }

    function drag(graph) {
        const plot = Qt.createComponent("SceneGraphPlot.qml").createObject(page, {
            "graph": graph,
            "width": 400,
            "height": 200
        });
        const tr = plot.build(graph.shown, graph.hidden, graph.chosen)[0];
        const at = plot.shifted(tr, tr.keys[1]);
        const hit = plot.mouse.hit(at[0], at[1]);
        if (hit.mode !== "move" || hit.t !== 2)
            fail("a press on a key did not pick it: " + JSON.stringify(hit));
        graph.chosen = [{
                "s": 0,
                "t": 2
            }];
        plot.mouse.held = hit;
        plot.mouse.mode = "move";
        plot.mouse.x0 = at[0];
        plot.dx = plot.tx(3) - plot.tx(2);
        plot.mouse.finish();
        const wide = plot.times[1] - plot.times[0];
        plot.view.zoom(200, 100, 0.5, false);
        if (Math.abs(plot.times[1] - plot.times[0] - wide / 2) > 1e-6)
            fail("zooming did not halve the time span: " + JSON.stringify(plot.times));
        graph.load();
        plot.view.fitPicked();
        if (!(plot.times[0] < 3 && plot.times[1] > 3 && plot.times[1] - plot.times[0] < wide))
            fail("fitting the picked key missed: " + JSON.stringify(plot.times) + JSON.stringify(graph.chosen));
        plot.view.fitAll();
        if (plot.view.span !== null || plot.win[1] !== 1)
            fail("fitting all keys kept a zoom");
        plot.destroy();
    }

    function exercise() {
        const steps = page.view.history.done.length;
        const pointer = "/scene/layers/0/transform/position";
        for (const key of [[0, [0, 0]], [2, [200, 100]], [4, [50, 50]]])
            page.run("keyframe", {
                    "pointer": pointer,
                    "time": key[0],
                    "value": key[1]
                });
        const sources = SceneGraphSources.tracks(page.view.tracks, {
            "pointer": pointer
        });
        const graph = Qt.createComponent("SceneGraph.qml").createObject(page, {
            "page": page,
            "sources": sources,
            "now": 6
        });
        graph.parent = QQC2.Overlay.overlay;
        graph.open();
        graph.load();
        const curve = graph.shown.find(c => c.pointer === pointer);
        if (!curve || curve.tracks.length !== 2 || curve.tracks[1].name !== "y")
            fail("the position did not show an x and a y track: " + JSON.stringify(graph.shown.map(c => c.pointer)));
        graph.sources = [sources.find(s => s.pointer === pointer)];
        graph.load();
        drag(graph);
        if (JSON.stringify(times(pointer)) !== "[0,3,4]")
            fail("dragging a key did not move it: " + JSON.stringify(times(pointer)));
        graph.chosen = [{
                "s": 0,
                "t": 3
            }, {
                "s": 0,
                "t": 4
            }];
        const before = page.view.history.done.length;
        graph.ops.move(0, 2, 3, 0, null);
        if (page.view.history.done.length !== before + 1 || JSON.stringify(times(pointer)) !== "[0,3,5]")
            fail("stretching two keys took more than one step or missed: " + JSON.stringify(times(pointer)));
        graph.ops.rescale(2, 0, 1);
        if (JSON.stringify(read(pointer + "/keys").map(k => k[1][1])) !== "[0,200,100]")
            fail("scaling the y of two keys missed: " + JSON.stringify(read(pointer + "/keys")));
        page.editor.hold(true);
        graph.ops.bend(0, 3, "out", [0.5, 0.1]);
        graph.ops.bend(0, 3, "out", [1, 0.2]);
        page.editor.hold(false);
        const key = read(pointer + "/keys/1");
        if (page.view.history.done.length !== before + 3 || JSON.stringify(key[2]) !== "{\"in\":[1,0.2],\"out\":[1,0.2]}")
            fail("one drag of an aligned handle was not one step: " + JSON.stringify(key));
        const both = graph.chosen;
        graph.ops.bend(0, 3, "out", [0.5, 30], 1);
        const split = read(pointer + "/keys/1")[2];
        if (!Array.isArray(split.out[0]) || split.out[1][1] !== 30 || split.out[0][1] === 30)
            fail("a handle of one part bent more than its track: " + JSON.stringify(split));
        graph.chosen = [both[0]];
        graph.load();
        if (!graph.ops.split(0, 3))
            fail("the curves did not tell the handles apart: " + JSON.stringify(graph.shown[0].keys[1]));
        graph.ops.link();
        if (Array.isArray(read(pointer + "/keys/1")[2].out[0]))
            fail("linking the parts kept one handle per part");
        graph.chosen = both;
        graph.ops.copy();
        if (!SceneGraphSources.board.text.startsWith("wprs-keys:") || SceneGraphSources.copied.length !== 1)
            fail("copying keys did not write them as text for the clipboard: " + SceneGraphSources.board.text);
        graph.ops.paste();
        if (JSON.stringify(times(pointer)) !== "[0,3,5,6,8]")
            fail("pasting at the playhead missed: " + JSON.stringify(times(pointer)));
        graph.ops.act({
            "remove": true
        });
        if (times(pointer).length !== 3)
            fail("removing the picked keys left " + times(pointer).length);
        const rows = SceneGraphSources.rows([{
                "kind": "bone",
                "name": "arm",
                "channel": "angles",
                "keys": [{}]
            }], {
            "layer": "body",
            "clip": "wave"
        }, null);
        if (JSON.stringify(rows[0].row) !== "[\"bones\",\"arm\",\"angles\"]")
            fail("a clip row did not name its channel: " + JSON.stringify(rows));
        graph.close();
        graph.destroy();
        while (page.view.history.done.length > steps)
            page.run("undo");
    }
}
