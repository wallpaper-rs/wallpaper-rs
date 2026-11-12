import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var page

    function read(pointer) {
        return JSON.parse(page.editor.value(pointer) || "null");
    }

    function make(name, props) {
        const made = Qt.createComponent(name).createObject(page, props);
        if (!made)
            throw new Error("seeding: " + name + " was not made");
        return made;
    }

    function fail(what) {
        throw new Error("seeding: " + what + (page.editor.error ? " (" + page.editor.error + ")" : ""));
    }

    function exercise() {
        const steps = page.view.history.done.length;
        const tools = make("SceneSwarmTools.qml", {
                "page": page,
                "system": "flakes"
            });
        const life = make("SceneSwarmSpread.qml", {
                "tools": tools,
                "row": ["life", "Life", 0, 30, 0.1],
                "value": [1, 2]
            });
        life.write([1.5, 4], 2, false);
        if (JSON.stringify(read("/scene/particles/flakes/init/life")) !== "[1.5,4,2]")
            fail("the two-ended slider and the curve did not set the range of the life");
        const tint = make("SceneSwarmTint.qml", {
                "tools": tools,
                "init": read("/scene/particles/flakes/init")
            });
        tint.choose(1);
        tint.init = read("/scene/particles/flakes/init");
        tint.hue("hue", [100, 200], false);
        if (read("/scene/particles/flakes/init/hsv/hue")[1] !== 200)
            fail("the hue range was not set");
        tools.edit("start", {
                "ranges": {
                    "hsv": {
                        "hue": [180, 240]
                    }
                }
            });
        tint.init = read("/scene/particles/flakes/init");
        if (JSON.stringify([tint.hues.sat, tint.hues.val]) !== "[[1,1],[1,1]]")
            fail("hues written without saturation or value do not show their full defaults");
        tint.choose(2);
        if ((read("/scene/particles/flakes/init/palette/colors") ?? []).length !== 2 || read("/scene/particles/flakes/init/hsv") !== null)
            fail("the palette did not replace the hues");
        tint.choose(3);
        let stops = null;
        const bar = make("SceneGradientBar.qml", {
                "stops": read("/scene/particles/flakes/init/gradient")
            });
        bar.edited.connect((s, gesture) => stops = s);
        bar.added(0.5);
        if (stops?.length !== 3 || stops[1][0] !== 0.5)
            fail("a click on the gradient bar did not add a stop in its place");
        bar.stops = stops;
        const moved = bar.moved(1, 0.95, true);
        bar.stops = stops;
        if (moved !== 1 || stops[1][0] !== 0.95 || bar.near(0.94, 0.02) !== 1)
            fail("a stop of the gradient did not move");
        let window = null;
        const fade = make("SceneLifeWindow.qml", {
                "window": [0, 0.1, 0.9, 1]
            });
        fade.edited.connect((w, gesture) => window = w);
        fade.put(1, 0.95, true);
        if (window[1] !== 0.9)
            fail("a handle of the life window passed the next one");
        const windows = make("SceneSwarmWindows.qml", {
                "tools": tools
            });
        windows.put(0, [0, 0.2, 0.8, 1], false);
        if (read("/scene/particles/flakes/forces/0/window")[2] !== 0.8)
            fail("the force took no window of the life");
        page.run("particles", {
                "create": {
                    "system": "lonely"
                }
            });
        const systems = make("SceneSwarmSystems.qml", {
                "page": page,
                "system": "flakes"
            });
        systems.rename("lonely", "alone");
        if (read("/scene/particles/alone") === null || read("/scene/particles/lonely") !== null)
            fail("the systems panel did not rename a system");
        systems.create({
                "system": "twin",
                "from": "alone"
            }, true);
        if (read("/scene/particles/twin") === null || page.editor.selected !== "/scene/particles/twin")
            fail("the systems panel did not copy a system and open the copy");
        systems.create({
                "system": "kid",
                "parent": "alone"
            }, false);
        if (read("/scene/particles/alone/children/0/system") !== "kid")
            fail("the systems panel did not add a system that another one starts");
        const named = make("SceneSwarmNamed.qml", {
                "page": page,
                "renamed": {
                    "from": "old",
                    "to": "alone",
                    "uses": [{
                            "path": "scripts/a.js",
                            "line": 3
                        }]
                }
            });
        if (named.where[0] !== "scripts/a.js:3" || !named.visible)
            fail("the rename warning does not name the line of the script");
        let closed = false;
        named.done.connect(() => closed = true);
        named.actions[0].trigger();
        if (page.editor.error !== "" || !closed)
            fail("the rename warning did not rewrite the names and close");
        if (page.view.history.done.length !== steps + 11)
            fail("a tool of the particles did not take exactly one undo step");
        for (let k = 0; k < 11; k++)
            page.run("undo");
        [tools, life, tint, bar, fade, windows, systems, named].forEach(o => o.destroy());
    }
}
