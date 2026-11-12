import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function read(pointer) {
        return JSON.parse(page.editor.value(pointer) || "null");
    }

    function ask(query) {
        return JSON.parse(page.editor.rig("", JSON.stringify(query)) || "null");
    }

    function mark(overlay, name) {
        return overlay.marks.find(m => m.name === name);
    }

    function layerAt(id) {
        return (page.view.gizmos ?? []).find(g => g.id === id);
    }

    function shapes() {
        page.run("add_layer", {
                "kind": "shape",
                "id": "fig_shape"
            });
        const gizmo = layerAt("fig_shape");
        page.run("layer", {
                "figure": {
                    "stop": {
                        "layer": "fig_shape",
                        "stop": {
                            "make": {}
                        }
                    }
                }
            });
        page.editor.select(gizmo.pointer);
        const overlay = Qt.createComponent("SceneShapeOverlay.qml").createObject(page, {
                "page": page,
                "ratio": 1
            });
        if (!overlay.visible || !mark(overlay, "size") || !mark(overlay, "radius") || !mark(overlay, "gradient_to"))
            throw new Error("figures: a rect shows no size, radius and gradient handles: " + JSON.stringify(overlay.marks));
        const size = read(gizmo.pointer + "/shape/rect/size");
        const to = mark(overlay, "size").at;
        overlay.put("size", [to[0] + 40, to[1] + 20], false);
        if (read(gizmo.pointer + "/shape/rect/size")[0] <= size[0])
            throw new Error("figures: dragging the size handle did not widen the rect");
        const end = mark(overlay, "gradient_to").at;
        overlay.put("gradient_to", [end[0], end[1] + 10], false);
        if (read(gizmo.pointer + "/gradient/to")[1] === 1)
            throw new Error("figures: dragging the end of the gradient changed nothing");
        const bar = Qt.createComponent("SceneShapeStops.qml").createObject(page, {
                "page": page,
                "layerId": "fig_shape",
                "stops": read(gizmo.pointer + "/gradient/stops")
            });
        bar.added(0.5);
        if (read(gizmo.pointer + "/gradient/stops").length !== 3)
            throw new Error("figures: the stops bar did not add a stop");
        bar.recolored(1, "#336699");
        if (read(gizmo.pointer + "/gradient/stops")[1][1] !== "#336699")
            throw new Error("figures: the stops bar did not recolour a stop");
        bar.stops = read(gizmo.pointer + "/gradient/stops");
        bar.removed(1);
        if (read(gizmo.pointer + "/gradient/stops").length !== 2)
            throw new Error("figures: the stops bar did not remove a stop");
        bar.destroy();
        overlay.destroy();
    }

    function texts() {
        page.run("add_layer", {
                "kind": "text",
                "id": "fig_text"
            });
        page.run("set", {
                "pointer": layerAt("fig_text").pointer + "/text",
                "value": "one two three four five\nsix seven"
            });
        const fonted = (ask({
                    "font_list": {
                        "layer": "fig_text"
                    }
                })?.system ?? []).length > 0;
        page.editor.select(layerAt("fig_text").pointer);
        const overlay = Qt.createComponent("SceneTextOverlay.qml").createObject(page, {
                "page": page,
                "ratio": 1
            });
        if (fonted) {
            if (!overlay.shown || overlay.shown.lines !== 2)
                throw new Error("figures: the text box does not count the typed line break: " + JSON.stringify(overlay.shown));
            overlay.wrap(overlay.shown.corners[1].map((v, k) => k === 0 ? v - 100 : v));
            if (!(read(layerAt("fig_text").pointer + "/wrap/width") > 0))
                throw new Error("figures: dragging the width handle did not wrap the text");
        }
        overlay.destroy();
        page.run("layer", {
                "figure": {
                    "fonts": {
                        "layer": "fig_text",
                        "font": "serif",
                        "fallback": ["monospace"]
                    }
                }
            });
        const pointer = layerAt("fig_text").pointer;
        if (read(pointer + "/font") !== "serif" || read(pointer + "/fallback")[0] !== "monospace")
            throw new Error("figures: the font and its fallback list were not set");
        const wall = 13 * 3600 + 5 * 60;
        const sample = ask({
                "clock_sample": {
                    "pattern": "HH:mm",
                    "wall": wall
                }
            });
        if (sample.text !== "13:05")
            throw new Error("figures: the format sample reads " + JSON.stringify(sample));
    }

    function exercise() {
        const steps = page.view.history.done.length;
        shapes();
        texts();
        while (page.view.history.done.length > steps)
            page.run("undo");
    }
}
