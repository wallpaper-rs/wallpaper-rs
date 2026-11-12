import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function exercise() {
        const steps = page.view.history.done.length;
        const tool = Qt.createComponent("SceneSplitTool.qml").createObject(page, {
            "page": page
        });
        const glow = Qt.createComponent("SceneGlowSplit.qml").createObject(page, {
            "page": page,
            "width": 400,
            "height": 200
        });
        page.run("canvas", {
            "bloom": {
                "strength": 1
            }
        });
        if (!tool.glowing || tool.halved)
            throw new Error("glow split: the button does not wait for a canvas with a glow");
        page.editor.halve(true);
        if (!tool.halved || glow.shown)
            throw new Error("glow split: turning it on shows the other half before its stream exists");
        glow.drag(100);
        if (glow.at !== 0.25)
            throw new Error("glow split: dragging the line did not move the split: " + glow.at);
        glow.drag(-20);
        if (glow.at !== 0.05)
            throw new Error("glow split: the line left the canvas: " + glow.at);
        page.run("canvas", {
            "bloom": {
                "off": true
            }
        });
        if (tool.glowing || tool.halved)
            throw new Error("glow split: it stayed on after the glow was removed");
        glow.destroy();
        tool.destroy();
        while (page.view.history.done.length > steps)
            page.run("undo");
    }
}
