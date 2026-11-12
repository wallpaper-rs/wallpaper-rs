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
}
