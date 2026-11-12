import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function expect(ok, message) {
        if (!ok)
            throw new Error("lights: " + message);
    }

    function lamp() {
        page.run("add_layer", {
            "kind": "light"
        });
        const layers = JSON.parse(page.editor.value("/scene/layers") || "[]");
        const at = layers.findIndex(l => l.type === "light");
        expect(at >= 0, "no light layer was added");
        const node = {
            "type": "light",
            "id": layers[at].id,
            "pointer": "/scene/layers/" + at
        };
        const tools = Qt.createComponent("SceneLightTools.qml").createObject(page, {
                "page": page,
                "node": node
            });
        expect(tools !== null && tools.light !== null, "the light tools did not load");
        tools.set({
            "light": "spot"
        });
        tools.load();
        expect(tools.light.type === "spot", "the type of the light did not change to spot");
        const air = Qt.createComponent("SceneLightAir.qml").createObject(page, {
                "tools": tools
            });
        air.load();
        expect(air.info !== null && air.info.on === false, "the air of a new light is not off");
        air.put({});
        air.load();
        expect(air.info.on === true, "the light did not light the air");
        air.put({
            "density": 0.7
        });
        air.load();
        expect(Math.abs(air.info.density - 0.7) < 0.01, "the density of the air was not kept: " + air.info.density);
        air.put({
            "off": true
        });
        air.load();
        expect(air.info.on === false, "the air stayed lit");
        const cast = Qt.createComponent("SceneLightCast.qml").createObject(page, {
                "tools": tools
            });
        cast.load();
        expect(cast.info !== null && cast.entries.length >= 1 && cast.entries[0].off === true, "a spot has no projection choices");
        tools.set({
            "light": "tube"
        });
        tools.load();
        expect(tools.light.type === "tube" && tools.light.length > 0, "the light did not become a tube");
        cast.load();
        expect(cast.info === null, "a tube still offers a projection");
        cast.destroy();
        air.destroy();
        tools.destroy();
    }

    function particles() {
        SceneSwarmGlow.load(page, "flakes", null);
        expect(SceneSwarmGlow.info !== null, "the particle light did not load");
        SceneSwarmGlow.run(page, "lit", {
            "lit": true
        });
        SceneSwarmGlow.load(page, "flakes", null);
        expect(SceneSwarmGlow.info.lit === true, "the particles did not take the lights");
        if ((SceneSwarmGlow.info.pictures ?? []).length > 0) {
            SceneSwarmGlow.run(page, "refract", {
                "normal": SceneSwarmGlow.info.pictures[0],
                "amount": 0.3
            });
            SceneSwarmGlow.load(page, "flakes", null);
            expect(SceneSwarmGlow.info.refract !== null && Math.abs(SceneSwarmGlow.info.refract.amount - 0.3) < 0.01, "the particles did not bend what is behind them");
            SceneSwarmGlow.run(page, "refract", {
                "off": true
            });
            SceneSwarmGlow.load(page, "flakes", null);
            expect(SceneSwarmGlow.info.refract === null, "the refraction stayed on");
        }
        SceneSwarmGlow.run(page, "lit", {
            "lit": false
        });
        SceneSwarmGlow.load(page, "flakes", null);
        expect(SceneSwarmGlow.info.lit === false, "the particles kept taking the lights");
    }

    function exercise() {
        lamp();
    }
}
