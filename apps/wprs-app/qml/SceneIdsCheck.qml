import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function read(pointer) {
        return JSON.parse(page.editor.value(pointer) || "null");
    }

    function layer(id) {
        return (page.view.layers ?? []).find(l => l.id === id) ?? null;
    }

    function exercise() {
        const steps = page.view.history.done.length;
        page.run("add_layer", {
            "kind": "solid",
            "id": "ids_lead"
        });
        page.run("add_layer", {
            "kind": "solid",
            "id": "ids_kid"
        });
        page.run("layer", {
            "parent": {
                "layer": "ids_kid",
                "parent": "ids_lead"
            }
        });
        const naming = Qt.createComponent("SceneRename.qml").createObject(page, {
            "page": page
        });
        naming.ask("layer", "ids_lead");
        naming.text = "ids_boss";
        naming.rename();
        naming.destroy();
        const kid = layer("ids_kid")?.pointer ?? "";
        if (read(kid + "/parent") !== "ids_boss")
            throw new Error("ids: renaming a layer did not rewrite the parent of its child: " + page.editor.error);
        const clip = Qt.createComponent("SceneLayerClip.qml").createObject(page, {
            "owner": {
                "page": page,
                "current": layer("ids_kid")
            }
        });
        const copied = clip.copy();
        if (!copied || (copied.layers ?? []).length !== 1)
            throw new Error("ids: copying a layer gave " + JSON.stringify(copied));
        if (SceneClip.recognised("some other text") !== null || SceneClip.recognised(SceneClip.marker + "{}") !== null)
            throw new Error("ids: text that is not a copied layer was taken for one");
        if (!SceneClip.board.text.startsWith(SceneClip.marker))
            throw new Error("ids: copying did not write the layers as text for the clipboard");
        const shared = SceneClip.recognised(SceneClip.marker + JSON.stringify(copied));
        if (!shared || typeof shared.from !== "string")
            throw new Error("ids: the copied layers were not recognised as a payload");
        clip.paste();
        clip.destroy();
        if (!layer("ids_kid_2"))
            throw new Error("ids: pasting did not add a copy with a free id: " + page.editor.error);
        page.run("remove_layer", {
            "id": "ids_boss"
        });
        if (read(layer("ids_kid").pointer + "/parent") !== null)
            throw new Error("ids: removing a parent left its child following it");
        const clock = Qt.createComponent("SceneCanvasClock.qml").createObject(page, {
            "page": page
        });
        const phone = (page.view.screen_sizes ?? []).find(s => s.size[1] > s.size[0]);
        clock.canvas({
            "size": {
                "size": phone.size
            }
        });
        clock.canvas({
            "clock": {
                "fresh": true
            }
        });
        if (JSON.stringify(page.view.canvas?.size) !== JSON.stringify(phone.size) || !clock.fresh)
            throw new Error("ids: the canvas did not take a phone size and a fresh seed: " + page.editor.error);
        clock.destroy();
        while (page.view.history.done.length > steps)
            page.run("undo");
    }
}
