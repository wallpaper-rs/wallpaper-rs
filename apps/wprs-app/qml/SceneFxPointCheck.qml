import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function read(pointer) {
        return JSON.parse(page.editor.value(pointer) || "null");
    }

    function mark(overlay, param) {
        return overlay.marks.find(m => m.param === param);
    }

    function exercise() {
        const steps = page.view.history.done.length;
        const id = read("/scene/layers/0/id");
        page.run("effects", {
                "add": {
                    "layer": id,
                    "effect": "spin"
                }
            });
        const use = "/scene/layers/0/effects/" + ((read("/scene/layers/0/effects") ?? []).length - 1);
        page.editor.select(use);
        const overlay = Qt.createComponent("SceneEffectPointOverlay.qml").createObject(page, {
                "page": page,
                "ratio": 1
            });
        const center = mark(overlay, "center"), angle = mark(overlay, "angle");
        if (!center || center.kind !== "point" || !angle || angle.kind !== "angle" || !overlay.visible)
            throw new Error("effect points: a spin shows no point and no dial on the canvas: " + JSON.stringify(overlay.marks));
        const gizmo = (page.view.gizmos ?? []).find(g => g.id === id);
        const to = overlay.at([0.25, 0.75]);
        overlay.put("center", to, false);
        const now = read(use + "/params/center");
        if (Math.abs(now[0] - 0.25) > 0.01 || Math.abs(now[1] - 0.75) > 0.01)
            throw new Error("effect points: dragging the point wrote " + JSON.stringify(now));
        const down = mark(overlay, "center").at;
        overlay.put("angle", [down[0], down[1] + 30], false);
        if (Math.abs(read(use + "/params/angle") - 90) > 0.5)
            throw new Error("effect points: the dial did not turn the angle to 90: " + read(use + "/params/angle"));
        if (!gizmo)
            throw new Error("effect points: the layer has no handles");
        overlay.destroy();
        generic(id);
        captured();
        while (page.view.history.done.length > steps)
            page.run("undo");
    }

    function generic(id) {
        page.run("effects", {
            "add": {
                "layer": id,
                "effect": "waves"
            }
        });
        const use = "/scene/layers/0/effects/" + ((read("/scene/layers/0/effects") ?? []).length - 1);
        page.editor.select(use);
        const phase = page.fields.find(field => field.key === "phase");
        for (const name of ["white", "black", "noise", "normal_flat"])
            if (!phase || phase.widget !== "ref:image" || !phase.options.includes(name))
                throw new Error("effect pictures: the picture of a texture does not offer " + name + ": " + JSON.stringify(phase?.options));
        page.run("set", {
            "pointer": use + "/params/phase",
            "value": "noise"
        });
        if (read(use + "/params/phase") !== "noise")
            throw new Error("effect pictures: a generic picture was not taken: " + page.editor.error);
    }

    function captured() {
        page.run("add_layer", {
                "kind": "adjustment",
                "id": "capture_lens"
            });
        const lens = (page.view.gizmos ?? []).find(g => g.id === "capture_lens");
        page.run("layer", {
                "transform": {
                    "layer": "capture_lens",
                    "position": [500, 400],
                    "size": [300, 200]
                }
            });
        page.editor.select(lens.pointer);
        const zone = Qt.createComponent("SceneCaptureZone.qml").createObject(page, {
                "page": page,
                "ratio": 1
            });
        if (!zone.shown || zone.shown.size[0] !== 300)
            throw new Error("capture: the zone is not shown for a layer with a size: " + JSON.stringify(zone.shown));
        zone.size([1, 1], [700, 560]);
        if (read(lens.pointer + "/size")[0] === 300 || zone.shown.size[0] === 300)
            throw new Error("capture: dragging a handle did not change the size");
        zone.destroy();
    }
}
