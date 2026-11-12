import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function read(pointer) {
        return JSON.parse(page.editor.value(pointer) || "null");
    }

    function exercise() {
        const grips = Qt.createComponent("SceneParallaxGrips.qml").createObject(page, {
            "page": page,
            "ratio": 1
        });
        page.editor.select("/scene/canvas");
        if (!grips.onCanvas || grips.offset === null)
            throw new Error("parallax grips: the canvas shows no handle for the offset of its parallax");
        grips.shift([grips.size[0] * 0.75, grips.size[1] * 0.5]);
        const offset = read("/scene/canvas/parallax/offset");
        if (!offset || Math.abs(offset[0] - 0.5) > 0.001 || Math.abs(offset[1]) > 0.001)
            throw new Error("parallax grips: dragging the offset handle wrote " + JSON.stringify(offset));
        page.editor.select("/scene/layers/0");
        if (grips.chosen === null || !grips.trying || grips.bound)
            throw new Error("parallax grips: a layer with a parallax depth shows no handle while the parallax is tried");
        grips.slide([grips.middle[0] + grips.unit * 2, grips.middle[1]], false);
        const apart = read("/scene/layers/0/parallax");
        if (!Array.isArray(apart) || Math.abs(apart[0] - 2) > 0.01 || Math.abs(apart[1]) > 0.01)
            throw new Error("parallax grips: dragging the depth handle of a layer wrote " + JSON.stringify(apart));
        page.run("layer", {
            "depth": {
                "layer": grips.chosen.id,
                "depth": [0.5, 0.5]
            }
        });
        grips.slide([grips.middle[0] + grips.unit, grips.middle[1] + grips.unit], false);
        if (read("/scene/layers/0/parallax") !== 1)
            throw new Error("parallax grips: the handle did not keep the same depth across and up and down: " + JSON.stringify(read("/scene/layers/0/parallax")));
        grips.slide([grips.middle[0] + grips.unit, grips.middle[1]], true);
        const free = read("/scene/layers/0/parallax");
        if (!Array.isArray(free) || Math.abs(free[0] - 1) > 0.01 || Math.abs(free[1]) > 0.01)
            throw new Error("parallax grips: holding Shift did not set the depths apart: " + JSON.stringify(free));
        grips.destroy();
    }
}
