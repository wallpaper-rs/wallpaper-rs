import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function dragged() {
        const panel = Qt.createComponent("SceneLayers.qml").createObject(page, {
            "page": page
        });
        const top = () => panel.layers.filter(layer => /^\/scene\/layers\/\d+$/.test(layer.pointer));
        const before = top();
        const moving = before.find(layer => !layer.locked);
        const onto = before.find(layer => layer.pointer !== moving.pointer);
        panel.drop(moving, onto);
        const after = top().find(layer => layer.id === moving.id);
        if (!after || after.pointer !== onto.pointer)
            throw new Error("dragging a layer did not move it: " + JSON.stringify(after));
        panel.drop(after, top().find(layer => layer.pointer === moving.pointer));
        const back = top().map(layer => layer.id).join();
        panel.destroy();
        if (back !== before.map(layer => layer.id).join())
            throw new Error("dragging a layer back did not restore the order: " + back);
    }

    function ran(check, method) {
        Qt.createComponent(check).createObject(page, {
            "page": page
        })[method ?? "exercise"]();
        App.backend.reap();
    }

    function exercise() {
        App.backend.reap();
        for (const kind of page.view.starters) {
            if (!App.backend.claim("scene." + kind))
                continue;
            page.editor.create(page.folder + "-" + kind, kind, "");
            page.run("add_layer", {
                "kind": "text"
            });
            page.change("/scene/layers/0/opacity", 0.5);
            page.run("keyframe", {
                "pointer": "/scene/layers/0/opacity",
                "time": 1,
                "value": 0.25,
                "ease": "in_out_sine"
            });
            page.run("duplicate_layer", {
                "id": "text"
            });
            page.run("lock", {
                "id": "text",
                "locked": true
            });
            page.run("hide", {
                "id": "text_copy",
                "hidden": true
            });
            page.run("solo", {
                "id": "text"
            });
            page.run("group", {
                "ids": ["text_copy"]
            });
            page.run("undo");
            page.run("redo");
            dragged();
            page.run("set", {
                "pointer": "/scene/timelines",
                "value": [
                    {
                        "id": "intro",
                        "duration": 2
                    }
                ]
            });
            page.run("set", {
                "pointer": "/scene/layers/0/opacity/timeline",
                "value": "intro"
            });
            page.run("set", {
                "pointer": "/scene/layers/0/effects",
                "value": [
                    {
                        "effect": "vignette"
                    },
                    {
                        "effect": "film_grain"
                    }
                ]
            });
            page.run("move", {
                "from": "/scene/layers/0/effects/1",
                "to": "/scene/layers/0/effects/0"
            });
            page.editor.select("/scene/layers/0/effects/0");
            if (page.editor.count("/scene/layers/0/effects") !== 2 || !page.fields.some(field => field.key === "effect" && field.value === "film_grain"))
                throw new Error(kind + ": the effects did not move");
            page.editor.select("/scene/layers/0/opacity");
            if (!page.fields.some(field => field.key === "timeline" && field.value === "intro"))
                throw new Error(kind + ": the keyframes do not show their timeline");
            const steps = page.view.history.done.length;
            page.change("/scene/canvas/background", "#101010", true);
            page.change("/scene/canvas/background", "#202020", true);
            if (page.view.history.done.length !== steps + 1)
                throw new Error(kind + ": a drag took more than one undo step");
            page.editor.select("/scene/layers/0");
            if (kind !== "empty" && !page.fields.some(field => field.widget === "stops"))
                throw new Error(kind + ": gradient stops have no editor");
            if (kind === "particles") {
                page.editor.select("/scene/particles/flakes/emitters/0");
                if (!page.fields.some(field => field.widget === "points"))
                    throw new Error(kind + ": the emitter box has no editor");
                ran("SceneSwarmCheck.qml");
                ran("SceneShapingCheck.qml");
                ran("SceneLightsCheck.qml", "particles");
            }
            if (kind === "empty")
                for (const check of ["SceneMeshCheck.qml", "SceneHandleCheck.qml", "SceneTimingCheck.qml", "SceneScriptCheck.qml", "SceneMediaCheck.qml", "SceneTouchCheck.qml", "SceneBandCheck.qml", "SceneBindCheck.qml", "SceneBindMenusCheck.qml", "SceneProblemsCheck.qml", "SceneLevelCheck.qml", "SceneLightsCheck.qml", "SceneHearingCheck.qml", "SceneShaderCheck.qml", "SceneTransitionCheck.qml"])
                    ran(check);
            if (kind === "parallax") {
                ran("SceneManifestCheck.qml");
                for (const check of ["SceneSettingsCheck.qml", "SceneTryCheck.qml", "SceneAuthorCheck.qml", "SceneImportCheck.qml", "SceneStatesCheck.qml"])
                    ran(check);
            }
            page.jump("/scene/layers/0/opacity");
            page.editor.seek(0.5, false);
            if (page.editor.error !== "" || page.view.problems.some(problem => !problem.warning))
                throw new Error(kind + ": " + (page.editor.error || JSON.stringify(page.view.problems)));
            App.backend.reap();
        }
        ran("SceneFrameCheck.qml");
        page.editor.save();
        page.tabs.currentIndex = 2;
        page.editor.select("/manifest");
        page.tabs.currentIndex = 1;
        page.builder.open();
        page.builder.close();
        page.use.apply("");
        if (!page.use.placing)
            throw new Error("setting the scene as the wallpaper did not start");
    }
}
