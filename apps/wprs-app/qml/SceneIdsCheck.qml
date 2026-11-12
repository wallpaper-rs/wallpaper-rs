import QtQuick

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
        clip.paste();
        clip.destroy();
        if (!layer("ids_kid_2"))
            throw new Error("ids: pasting did not add a copy with a free id: " + page.editor.error);
        page.run("remove_layer", {
            "id": "ids_boss"
        });
        if (read(layer("ids_kid").pointer + "/parent") !== null)
            throw new Error("ids: removing a parent left its child following it");
        while (page.view.history.done.length > steps)
            page.run("undo");
    }
}
