import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function exercise() {
        page.run("add_layer", {
                "kind": "solid",
                "id": "pad"
            });
        const pointer = page.mesh.pointer("pad");
        const panel = Qt.createComponent("SceneTouch.qml").createObject(page, {
                "page": page,
                "node": {
                    "id": "pad",
                    "type": "solid",
                    "pointer": pointer
                }
            });
        if (panel.takes !== "off")
            throw new Error("touch: a new layer takes the pointer");
        panel.put("all", null);
        if (JSON.parse(page.editor.value(pointer + "/solid")) !== true || panel.takes !== "all")
            throw new Error("touch: the layer does not take the pointer: " + page.editor.error);
        panel.put("alpha", 0.3);
        if (panel.takes !== "alpha" || panel.alpha !== 0.3)
            throw new Error("touch: the opacity threshold was not set");
        const overlay = Qt.createComponent("SceneTouchOverlay.qml").createObject(page, {
                "page": page,
                "ratio": 1
            });
        const listed = overlay.areas.find(a => a.id === "pad");
        if (!listed || listed.takes !== "alpha" || listed.alpha !== 0.3)
            throw new Error("touch: the overlay does not list the area: " + JSON.stringify(overlay.areas));
        overlay.destroy();
        panel.put("off", null);
        if (page.editor.value(pointer + "/solid") !== "" && page.editor.value(pointer + "/solid") !== "null")
            throw new Error("touch: letting the pointer through left a solid");
        panel.destroy();
        page.run("remove_layer", {
                "id": "pad"
            });
    }
}
