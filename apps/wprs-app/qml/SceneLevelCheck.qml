import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function ask(quality, battery) {
        return JSON.parse(page.editor.rig("", JSON.stringify({
                    "level": {
                        "quality": quality,
                        "battery": battery,
                        "calm": false
                    }
                })) || "null");
    }

    function exercise() {
        const lowest = ask("q0", false), drained = ask("q3", true);
        if (!lowest || lowest.quality !== "q0" || !Array.isArray(lowest.effects) || drained.quality !== "q2")
            throw new Error("levels: the editor did not answer for a level: " + JSON.stringify([lowest, drained]));
        SceneLook.quality = "q1";
        SceneLook.battery = true;
        SceneLook.calm = true;
        SceneLook.preview(page);
        const look = JSON.parse(page.editor.look);
        if (look.quality !== "q1" || look.battery !== true || look.calm !== true)
            throw new Error("levels: the preview look lost its level: " + page.editor.look);
        SceneLook.versus = "q0";
        SceneLook.versusGlow = true;
        SceneLook.rival = true;
        SceneLook.preview(page);
        const other = JSON.parse(page.editor.look);
        if (other.quality !== "q0" || other.bloom !== false)
            throw new Error("levels: the other side of a comparison was not previewed: " + page.editor.look);
        SceneLook.rival = false;
        SceneLook.versusGlow = false;
        SceneLook.battery = false;
        SceneLook.calm = false;
        SceneLook.quality = "q3";
        SceneLook.preview(page);
        const back = JSON.parse(page.editor.look);
        if (back.quality !== "q3" || back.battery || "bloom" in back)
            throw new Error("levels: the preview did not return to the full level: " + page.editor.look);
        const panel = Qt.createComponent("ScenePreviewPanel.qml").createObject(page, {
                "page": page
            });
        if (!panel)
            throw new Error("levels: the preview level panel did not load");
        panel.destroy();
    }
}
