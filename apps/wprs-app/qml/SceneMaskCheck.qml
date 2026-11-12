import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function read(pointer) {
        return JSON.parse(page.editor.value(pointer) || "null");
    }

    function exercise() {
        const steps = page.view.history.done.length;
        const id = read("/scene/layers/0/id");
        page.run("effects", {
                "add": {
                    "layer": id,
                    "effect": "vignette"
                }
            });
        const index = (read("/scene/layers/0/effects") ?? []).length - 1;
        const use = "/scene/layers/0/effects/" + index;
        page.editor.select(use);
        const panel = Qt.createComponent("SceneEffectMask.qml").createObject(page, {
                "page": page,
                "pointer": use
            });
        const overlay = Qt.createComponent("SceneMaskOverlay.qml").createObject(page, {
                "page": page,
                "ratio": 1
            });
        panel.put({
                "shape": panel.shaped("rect")
            });
        if (panel.source !== "shape" || panel.kind !== "rect" || overlay.kind !== "rect")
            throw new Error("mask: the panel did not give the effect a shape: " + page.editor.error);
        const owner = overlay.owner;
        overlay.hold("size", overlay.at([0.5 + 60 / owner.size[0], 0.5 - 25 / owner.size[1]]));
        overlay.put({
                "feather": 12
            });
        const mask = read(use + "/mask");
        if (JSON.stringify(mask.shape.rect.size) !== "[120,50]" || mask.feather !== 12)
            throw new Error("mask: the handles did not size or soften the shape: " + JSON.stringify(mask));
        const before = page.view.history.done.length;
        page.editor.hold(true);
        overlay.put({
                "feather": 20
            });
        const until = Date.now() + 700;
        while (Date.now() < until) {
        }
        overlay.put({
                "feather": 30
            });
        page.editor.hold(false);
        if (page.view.history.done.length !== before + 1 || read(use + "/mask/feather") !== 30)
            throw new Error("mask: a held drag of the soft edge that paused took " + (page.view.history.done.length - before) + " steps");
        panel.put({
                "shape": panel.shaped("star")
            });
        overlay.hold("size", overlay.at([0.5 + 40 / owner.size[0], 0.5 + 10 / owner.size[1]]));
        if (read(use + "/mask/shape/star/size") !== 80)
            throw new Error("mask: the star was not sized from its handle");
        const thumb = Qt.createComponent("SceneMaskThumb.qml").createObject(page, {
                "page": page,
                "layerId": id,
                "index": index
            });
        if (!thumb.shown || thumb.shown.area_u.length === 0 || thumb.loops.length === 0)
            throw new Error("mask: the thumbnail does not draw the shape: " + JSON.stringify(thumb.shown));
        thumb.destroy();
        layered(use);
        panel.put({
                "off": true
            });
        if (read(use + "/mask") !== null || overlay.masking)
            throw new Error("mask: the mask was not taken off");
        panel.destroy();
        overlay.destroy();
        while (page.view.history.done.length > steps)
            page.run("undo");
    }

    function layered(use) {
        const pointer = "/scene/layers/0";
        page.editor.select(pointer);
        const panel = Qt.createComponent("SceneEffectMask.qml").createObject(page, {
                "page": page,
                "pointer": pointer
            });
        panel.put({
                "shape": panel.shaped("ellipse")
            });
        if (panel.index !== -1 || panel.source !== "shape" || read(pointer + "/mask/shape/ellipse") === null)
            throw new Error("mask: the panel did not give the layer a mask of its own: " + page.editor.error);
        const overlay = Qt.createComponent("SceneMaskOverlay.qml").createObject(page, {
                "page": page,
                "ratio": 1
            });
        if (!overlay.masking || overlay.kind !== "ellipse" || overlay.shown.holds.length < 2)
            throw new Error("mask: the mask of the layer is not on the canvas: " + JSON.stringify(overlay.shown));
        overlay.hold("feather", overlay.shown.holds.find(h => h.grip === "size").at);
        panel.put({
                "off": true
            });
        if (overlay.masking || read(pointer + "/mask") !== null)
            throw new Error("mask: the mask of the layer was not taken off");
        overlay.destroy();
        panel.destroy();
        page.editor.select(use);
    }
}
