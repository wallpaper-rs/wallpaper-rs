import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function fail(why) {
        throw new Error("transition: " + why + " " + page.editor.error);
    }

    function exercise() {
        const tool = page.settings.passage;
        const steps = page.view.history.done.length;
        tool.load();
        if (tool.info.in !== null)
            fail("the scene already had a transition");
        tool.start("in");
        if (!tool.open || tool.current === null || tool.current.descriptor !== "transitions/opening.json")
            fail("making one did not open the tool on the starter effect");
        if (!tool.current.plays.every(p => p))
            fail("the starter effect does not play at every level");
        tool.act({
            "duration_ms": 2500,
            "curve": [0.2, 0.8, 0.4, 1]
        });
        if (tool.current.duration_ms !== 2500 || tool.said !== "")
            fail("a duration and a curve did not change: " + tool.said);
        tool.act({
            "duration_ms": 20000
        });
        if (tool.said === "" || tool.current.duration_ms !== 2500)
            fail("a duration too long was not refused beside the value");
        page.editor.error = "";
        tool.act({
            "params": {
                "center": [0.3, 0.7]
            }
        });
        if (JSON.stringify(tool.current.params.center) !== "[0.3,0.7]")
            fail("moving the center did not write it");
        tool.progress = 0.8;
        tool.remove();
        if (tool.open || tool.info.in !== null)
            fail("removing it left it in the scene");
        page.run("undo");
        tool.load();
        if (tool.info.in === null)
            fail("undo did not bring it back");
        while (page.view.history.done.length > steps)
            page.run("undo");
        tool.load();
        tool.close();
        tool.progress = 0.45;
    }
}
