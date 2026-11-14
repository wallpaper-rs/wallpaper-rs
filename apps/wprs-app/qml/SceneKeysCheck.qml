import QtQuick
import io.github.wallpaper_rs.app

Timer {
    id: keys

    required property var host
    required property KeyWalk walk
    property bool finished: !App.backend.claim("keys.editor")
    property var page: null
    property int waited: 0
    readonly property SceneEditor helper: SceneEditor {}
    readonly property var regions: ["SceneSide", "SceneLayers", "SceneCanvasBar", "SceneOverlays", "SceneFields", "SceneProblems", "SceneFoot"]

    function begin() {
        if (finished)
            return;
        host.work(Qt.createComponent("ScenePage.qml"), {
            "folder": helper.folderFor("keys"),
            "starter": "parallax",
            "heading": "",
            "exercised": false
        });
        page = host.pageStack.currentItem;
        start();
    }

    function found(name) {
        return walk.gather(page, []).find(item => String(item).startsWith(name + "_") || String(item).startsWith(name + "("));
    }

    function tab(at) {
        const button = walk.gather(found("SceneSide"), []).find(item => item.text === App.tr(["Layers", "Assets", "Options", "Try"][at]) && item.checkable !== undefined);
        button.forceActiveFocus(Qt.TabFocusReason);
        walk.press(Qt.Key_Space, 0);
        if (page.tabs.currentIndex !== at)
            throw new Error("the Space key on the editor tab " + at + " does not open it");
    }

    function canvas(seen) {
        const overlays = found("SceneOverlays");
        const chosen = page.view.gizmos.find(gizmo => gizmo.pointer.startsWith("/scene/layers/") && !gizmo.locked);
        page.editor.select(chosen.pointer);
        walk.host.contentItem.forceActiveFocus();
        for (let n = 0; n < seen.length && !overlays.activeFocus; n++)
            walk.press(Qt.Key_Tab, 0);
        if (!overlays.activeFocus || !overlays.marked)
            throw new Error("the Tab key does not reach the canvas with its focus mark");
        const before = JSON.stringify(page.view.gizmos.find(gizmo => gizmo.pointer === chosen.pointer));
        const steps = page.view.history.done.length;
        walk.press(Qt.Key_Right, 0);
        walk.press(Qt.Key_Down, Qt.ShiftModifier);
        const after = JSON.stringify(page.view.gizmos.find(gizmo => gizmo.pointer === chosen.pointer));
        if (after === before || page.view.history.done.length !== steps + 2)
            throw new Error("the arrow keys on the canvas do not move the chosen layer");
        walk.press(Qt.Key_Escape, 0);
        if (overlays.activeFocus)
            throw new Error("the Escape key does not leave the canvas");
    }

    function next() {
        if (host.pageStack.currentItem !== page)
            throw new Error("the editor page was replaced before its keyboard walk");
        if (!page.editor.ready || (page.view.gizmos ?? []).length === 0)
            return ++waited < 40;
        page.change("/manifest/title", "");
        if ((page.view.problems ?? []).length === 0)
            throw new Error("the editor's keyboard walk found no problem to reach");
        for (let at = 3; at >= 0; at--) {
            tab(at);
            const seen = walk.check("editor tab " + at);
            const lost = regions.filter(name => found(name).visible && !walk.reached(seen, found(name)));
            if (lost.length > 0)
                throw new Error("the Tab key does not reach the editor's " + lost.join(", "));
        }
        page.run("undo");
        canvas(walk.tabs());
        return false;
    }

    function finish() {
        stop();
        finished = true;
    }

    interval: 250
    repeat: true
    onTriggered: {
        try {
            if (!next()) {
                if (waited >= 40)
                    throw new Error("the editor did not become ready for its keyboard walk");
                finish();
            }
        } catch (error) {
            finish();
            throw error;
        }
    }
}
