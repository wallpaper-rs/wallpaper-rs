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
        while (page.view.history.done.length > steps)
            page.run("undo");
    }
}
