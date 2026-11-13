import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function read(pointer) {
        return JSON.parse(page.editor.value(pointer) || "null");
    }

    function events() {
        const line = Qt.createComponent("SceneClipLine.qml").createObject(page, {
                "page": page
            });
        line.load();
        const lane = Qt.createComponent("SceneClipEvents.qml").createObject(page, {
                "line": line
            });
        const pointer = page.mesh.pointer("puppet") + "/clips/0/events";
        lane.edit({
            "name": lane.fresh(),
            "time": 0.5
        });
        if (page.editor.error !== "" || read(pointer)?.length !== 1)
            throw new Error("clip events: the lane did not add an event: " + page.editor.error);
        line.load();
        const name = read(pointer)[0].name;
        lane.edit({
            "name": name,
            "at": 0.5,
            "time": 1
        });
        lane.edit({
            "name": name,
            "at": 1,
            "to": "blink"
        });
        if (read(pointer)[0].t !== 1 || read(pointer)[0].name !== "blink")
            throw new Error("clip events: the event was not moved and renamed");
        lane.edit({
            "name": "blink",
            "remove": true
        });
        if (read(pointer) !== null)
            throw new Error("clip events: the event was not removed");
        lane.destroy();
        line.destroy();
    }

    function shapes() {
        const stroke = {
            "layer": "puppet",
            "shape": "puff",
            "points": [[0.5, 0.5]],
            "mode": "push",
            "by": 10
        };
        page.run("shape_sculpt", stroke);
        if (page.editor.error !== "" || !(page.mesh.ask("puppet").shapes ?? []).includes("puff"))
            throw new Error("sculpt: the stroke made no blend shape: " + page.editor.error);
        const panel = Qt.createComponent("SceneShapeSculpt.qml").createObject(page, {
                "host": {
                    "page": page,
                    "layerId": "puppet",
                    "editable": true,
                    "solid": false,
                    "info": page.mesh.ask("puppet")
                }
            });
        panel.mesh.tool("puppet", "shape", panel.brush());
        panel.show();
        if (panel.shape !== "puff" || page.mesh.shapes.puff !== 1)
            throw new Error("sculpt: the preview does not show the shape being sculpted");
        page.mesh.tool("puppet", "shape", null);
        page.mesh.shapes = {};
        panel.destroy();
        ring();
    }

    function ring() {
        const host = Qt.createQmlObject("import QtQuick; Item { width: 400; height: 200 }", page);
        const ring = Qt.createComponent("SceneBrushRing.qml").createObject(host, {
                "host": host,
                "brush": {
                    "radius": 0.1
                }
            });
        if (Math.round(ring.diameter) !== 80)
            throw new Error("brush ring: the ring does not follow the radius: " + ring.diameter);
        ring.trail = [[0.25, 0.5], [0.5, 0.5]];
        if (!ring.visible || Math.round(ring.spotX) !== 200 || Math.round(ring.spotY) !== 100)
            throw new Error("brush ring: the ring does not follow the stroke: " + ring.spotX + " " + ring.visible);
        ring.brush = null;
        if (ring.visible)
            throw new Error("brush ring: the ring shows without a brush");
        host.destroy();
    }

    function script() {
        const pick = Qt.createComponent("SceneScriptRigPick.qml").createObject(page, {
                "page": page
            });
        const found = pick.ask({
            "kind": "bone",
            "name": "limb",
            "what": "turn"
        });
        if (found.code !== 'scene.layer("puppet").bone("limb").rotation = 0;')
            throw new Error("script calls: " + JSON.stringify(found));
        pick.destroy();
    }
}
