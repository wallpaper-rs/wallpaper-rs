import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function read(pointer) {
        return JSON.parse(page.editor.value(pointer) || "null");
    }

    function make(name, props) {
        const made = Qt.createComponent(name).createObject(page, props);
        if (!made)
            throw new Error("textures: " + name + " was not made: " + page.editor.error);
        return made;
    }

    function exercise() {
        const id = read("/scene/layers/0/id");
        page.run("effects", {
                "add": {
                    "layer": id,
                    "effect": "fire"
                }
            });
        const use = "/scene/layers/0/effects/" + ((read("/scene/layers/0/effects") ?? []).length - 1);
        const at = use + "/params/gradient";
        const input = value => ({
                    "page": page,
                    "field": {
                        "pointer": at
                    },
                    "value": value
                });
        const ramp = make("SceneRampTool.qml", {
                "input": input(""),
                "format": "png"
            });
        ramp.make([[0, "#000000"], [1, "#ff0000"]]);
        const made = read(at);
        if (typeof made !== "string" || read("/scene/assets/" + made + "/path") === null)
            throw new Error("textures: the stops made no ramp picture: " + page.editor.error);
        ramp.input = input(made);
        if (ramp.kept?.length !== 2 || ramp.stops[1][1] !== "#ff0000")
            throw new Error("textures: the ramp did not read its stops back");
        const stops = make("SceneStops.qml", {
                "stops": ramp.stops
            });
        let sent = null;
        stops.committed.connect(list => sent = list);
        stops.live = [[0, "#000000"], [0.5, "#ffff00"], [1, "#ff0000"]];
        stops.finish();
        ramp.make(sent);
        if (read(at) !== made || read("/scene/ext/editor/ramps/" + made).length !== 3)
            throw new Error("textures: editing the stops did not make the same ramp again");
        const brush = make("SceneTextureBrush.qml", {
                "input": input(""),
                "mode": "flow"
            });
        brush.pick();
        if (page.brush?.direction !== true || page.brush.pointer !== at || !brush.painting)
            throw new Error("textures: the direction brush was not offered");
        const grey = make("SceneTextureBrush.qml", {
                "input": input(""),
                "mode": "gray"
            });
        grey.pick();
        if (page.brush?.direction === true || !grey.painting || brush.painting)
            throw new Error("textures: the grey brush was not offered");
        page.brush = null;
        const depth = make("SceneDepthTool.qml", {
                "input": input("")
            });
        if (!depth.model || !String(depth.model.install).includes("models download"))
            throw new Error("textures: the depth tool did not ask whether the model is installed");
    }
}
