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

    function rate() {
        return JSON.parse(page.editor.value("/scene/particles/flakes/emitters/0/rate") || "null");
    }

    function pause() {
        page.editor.passTime(600);
    }

    function exercise() {
        const tools = Qt.createComponent("SceneSwarmTools.qml").createObject(page, {
                "page": page,
                "system": "flakes"
            });
        const emitter = find(tools, item => item.emitter !== undefined && item.tools === tools);
        const slider = find(emitter, item => item.holding !== undefined && item.from !== undefined);
        const knob = find(emitter, item => item.moved !== undefined && item.modelData?.[0] === "delay");
        if (!emitter || !slider || !knob)
            throw new Error("kept: the emitter tools are missing their rate slider or emission lane");
        const emission = find(emitter, item => typeof item.rate === "function" && typeof item.put === "function");
        const steps = page.view.history.done.length;
        slider.hold(true);
        emission.rate(11, false, true);
        emission.rate(12, false, true);
        pause();
        emission.rate(13, false, true);
        if (find(tools, item => item === emitter) !== emitter || find(emitter, item => item === slider) !== slider || find(emitter, item => item === knob) !== knob)
            throw new Error("kept: an edit during a drag rebuilt the emitter that was being dragged");
        slider.parent = null;
        slider.hold(false);
        pause();
        emission.rate(14, false, true);
        if (page.view.history.done.length !== steps + 2 || rate() !== 14)
            throw new Error("kept: a drag whose slider left mid-gesture took " + (page.view.history.done.length - steps) + " steps, not 2");
        emission.put({
                "delay": 2
            });
        if (knob.modelData[1] !== 2 || find(emitter, item => item === knob) !== knob)
            throw new Error("kept: the lane did not follow a new start without rebuilding its handles");
        const layer = (page.view.layers ?? []).find(l => l.type === "particles");
        const scope = find(tools, item => item.names !== undefined && item.layers !== undefined);
        const render = () => JSON.parse(page.editor.value(layer.pointer + "/render") || "null");
        tools.renderLayer = layer.id;
        tools.edit("render", {
                "fps": 9
            });
        tools.load();
        if (render()?.fps !== 9 || tools.info.scope.differs.indexOf("fps") < 0 || !scope.info.own)
            throw new Error("scope: a change with a layer chosen did not start the render of that layer: " + JSON.stringify(tools.info.scope) + page.editor.error);
        tools.edit("scope", {
                "layer": layer.id,
                "own": false
            });
        tools.load();
        if (render() !== null || tools.info.scope.own)
            throw new Error("scope: the layer did not go back to the render of the system: " + page.editor.error);
        tools.renderLayer = "";
        tools.destroy();
    }
}
