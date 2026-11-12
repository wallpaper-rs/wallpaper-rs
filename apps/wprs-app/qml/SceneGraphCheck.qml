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
        graph.move(0, 2, 3, 0, null);
        if (page.view.history.done.length !== before + 1 || JSON.stringify(times(pointer)) !== "[0,3,5]")
            fail("stretching two keys took more than one step or missed: " + JSON.stringify(times(pointer)));
        page.editor.hold(true);
        graph.bend(0, 3, "out", [0.5, 0.1]);
        graph.bend(0, 3, "out", [1, 0.2]);
        page.editor.hold(false);
        const key = read(pointer + "/keys/1");
        if (page.view.history.done.length !== before + 2 || JSON.stringify(key[2]) !== "{\"in\":[1,0.2],\"out\":[1,0.2]}")
            fail("one drag of an aligned handle was not one step: " + JSON.stringify(key));
        graph.copy();
        graph.paste();
        if (JSON.stringify(times(pointer)) !== "[0,3,5,6,8]")
            fail("pasting at the playhead missed: " + JSON.stringify(times(pointer)));
        graph.act({
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
