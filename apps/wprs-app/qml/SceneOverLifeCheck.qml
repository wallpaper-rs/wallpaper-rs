import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function find(item, name) {
        if (!item)
            return null;
        if (item.objectName === name)
            return item;
        for (const child of item.children ?? []) {
            const found = find(child, name);
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
        const sizes = () => tools.info.render.size_over_life ?? [];
        const colors = () => tools.info.render.color_over_life ?? [];
        const steps = page.view.history.done.length;
        tools.edit("over", {
            "size": [],
            "color": []
        });
        tools.load();
        if (sizes().length > 0 || colors().length > 0)
            throw new Error("over life: the curves did not clear");
        find(tools, "sizeOverLife").clicked();
        find(tools, "colorOverLife").clicked();
        tools.load();
        if (sizes().length !== 2 || colors().length !== 2)
            throw new Error("over life: the buttons did not start the size and color curves: " + page.editor.error);
        if (!find(tools, "sizeCurve").visible || !find(tools, "colorBar").visible)
            throw new Error("over life: the curve and the gradient did not show");
        find(tools, "sizeCurve").send([[0, 0], [0.5, 1.5], [1, 0]], false);
        tools.load();
        if (sizes().length !== 3)
            throw new Error("over life: the size curve did not take its new points");
        find(tools, "sizeOverLife").clicked();
        find(tools, "colorOverLife").clicked();
        tools.load();
        if (sizes().length !== 0 || colors().length !== 0)
            throw new Error("over life: the remove buttons left curves");
        while (page.view.history.done.length > steps)
            page.run("undo");
        tools.destroy();
    }
}
