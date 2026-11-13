import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function find(item, test) {
        if (!item)
            return null;
        if (test(item))
            return item;
        for (const child of item.children ?? []) {
            const found = find(child, test);
            if (found)
                return found;
        }
        return null;
    }

    function exercise() {
        const tools = Qt.createComponent("SceneSwarmTools.qml").createObject(page, {
                "page": page,
                "system": "flakes"
            });
        const count = () => tools.info.emitters.length;
        const before = count();
        const add = find(tools, item => item.objectName === "addEmitter");
        if (!add || !add.enabled)
            throw new Error("emitters: the panel has no button to add an emitter");
        add.clicked();
        tools.load();
        if (count() !== before + 1)
            throw new Error("emitters: adding an emitter left " + count() + " of " + (before + 1));
        const drop = find(tools, item => item.objectName === "removeEmitter" && item.enabled);
        if (!drop)
            throw new Error("emitters: an emitter has no button to remove it");
        drop.clicked();
        tools.load();
        if (count() !== before)
            throw new Error("emitters: removing an emitter left " + count() + " of " + before);
        page.run("undo");
        page.run("undo");
        tools.destroy();
    }
}
