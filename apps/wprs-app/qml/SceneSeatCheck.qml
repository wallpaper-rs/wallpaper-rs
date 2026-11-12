import QtQuick

QtObject {
    required property var page

    function read(pointer) {
        return JSON.parse(page.editor.value(pointer) || "null");
    }

    function seat(id) {
        return (page.view.gizmos ?? []).find(g => g.id === id);
    }

    function search(item, test) {
        for (const child of item?.children ?? []) {
            if (test(child))
                return child;
            const found = search(child, test);
            if (found)
                return found;
        }
        return null;
    }

    function near(a, b) {
        return Math.abs(a[0] - b[0]) < 0.1 && Math.abs(a[1] - b[1]) < 0.1;
    }

    function handle(id) {
        return Qt.createComponent("SceneGizmo.qml").createObject(page, {
                "page": page,
                "index": (page.view.gizmos ?? []).findIndex(g => g.id === id),
                "ratio": 1
            });
    }

    function exercise() {
        const steps = page.view.history.done.length;
        page.run("add_layer", {
                "kind": "solid",
                "id": "seat_lead"
            });
        page.run("add_layer", {
                "kind": "solid",
                "id": "seat_kid"
            });
        page.run("layer", {
                "transform": {
                    "layer": "seat_lead",
                    "position": [400, 300],
                    "rotation": 90,
                    "size": [200, 100]
                }
            });
        page.run("layer", {
                "transform": {
                    "layer": "seat_kid",
                    "position": [700, 500],
                    "size": [50, 50]
                }
            });
        const kid = seat("seat_kid").pointer;
        const before = seat("seat_kid").pivot;
        page.editor.select(kid);
        page.picking = "parent";
        const lead = handle("seat_lead");
        lead.tapped();
        lead.destroy();
        if (read(kid + "/parent") !== "seat_lead" || page.picking !== "")
            throw new Error("seats: tapping a layer while picking did not make it the parent: " + page.editor.error);
        if (!near(seat("seat_kid").pivot, before))
            throw new Error("seats: the new child jumped from " + JSON.stringify(before) + " to " + JSON.stringify(seat("seat_kid").pivot));
        const gizmo = handle("seat_kid");
        gizmo.grip("move", [520, 330], false);
        if (!near(seat("seat_kid").pivot, [520, 330]))
            throw new Error("seats: the handle of a child does not land under the pointer: " + JSON.stringify(seat("seat_kid").pivot));
        const overlays = search(page, item => typeof item.nudge === "function");
        if (!overlays?.nudge([0, 10], false) || !near(seat("seat_kid").pivot, [520, 340]))
            throw new Error("seats: the arrow keys did not nudge the layer");
        const corner = seat("seat_kid").outline[2];
        gizmo.grip({
                "scale": {
                    "corner": [1, 1]
                }
            }, [corner[0] + 10, corner[1] + 10], false);
        if (!near(seat("seat_kid").outline[2], [corner[0] + 10, corner[1] + 10]))
            throw new Error("seats: the corner did not follow the pointer");
        gizmo.destroy();
        const tools = search(page, item => typeof item.fit === "function" && item.chosen !== undefined);
        tools.fit("stretch");
        const size = page.view.canvas?.size ?? [1920, 1080];
        if (!near(seat("seat_kid").outline[0], [0, 0]) || !near(seat("seat_kid").outline[2], size))
            throw new Error("seats: stretching to the canvas left " + JSON.stringify(seat("seat_kid").outline));
        while (page.view.history.done.length > steps)
            page.run("undo");
        Qt.createComponent("SceneIdsCheck.qml").createObject(page, {
            "page": page
        }).exercise();
    }
}
