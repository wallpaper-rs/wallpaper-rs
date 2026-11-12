import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var page

    function exercise() {
        const meter = Qt.createComponent("SceneBandMeter.qml").createObject(page, {
                "page": page,
                "source": {
                    "audio": [0, 3],
                    "of": 16,
                    "reduce": "max"
                },
                "of": 16,
                "live": false
            });
        page.editor.levels = JSON.stringify({
                "granted": true,
                "listening": true,
                "raw": [[0.9, 0.1], [0.9, 0.1]],
                "loudness": 0.5,
                "beat": false,
                "silent": false
            });
        if (meter.bands.length !== 16 || JSON.stringify(meter.reply.marked) !== "[0,3]" || Math.abs(meter.reply.value - 0.9) > 1e-6)
            throw new Error("hearing: the meter gave " + JSON.stringify(meter.reply));
        page.editor.levels = JSON.stringify({
                "granted": false,
                "listening": false
            });
        if (meter.reply.granted !== false || meter.bands.length !== 0)
            throw new Error("hearing: the meter drew levels without the consent");
        page.editor.levels = "";
        meter.destroy();
        const pointer = "/scene/layers/0/opacity";
        const steps = page.view.history.done.length;
        page.run("set", {
            "pointer": pointer,
            "value": {
                "audio": "bass",
                "smooth": 0.3
            }
        });
        page.editor.select(pointer);
        if (!page.fields.some(field => field.key === "smooth" && field.widget === "smooth"))
            throw new Error("hearing: the smoothing of a binding has no control");
        const plot = JSON.parse(page.editor.rig("", JSON.stringify({
                    "response": {
                        "smooth": {
                            "up": 0.1,
                            "down": 1
                        }
                    }
                })));
        if (plot.points.length < 10 || plot.settles[1] <= plot.settles[0])
            throw new Error("hearing: the response plot is wrong " + JSON.stringify(plot));
        for (let k = page.view.history.done.length - steps; k > 0; k--)
            page.run("undo");
        const sounds = JSON.parse(page.editor.rig("", "{\"sounds\":{}}"));
        if (!Array.isArray(sounds.layers) || !Array.isArray(sounds.assets))
            throw new Error("hearing: the list of sounds is " + JSON.stringify(sounds));
        const restarts = SceneLook.restart;
        SceneLook.hear(page, {
            "asset": "none"
        });
        if (JSON.parse(page.editor.look).listen?.asset !== "none")
            throw new Error("hearing: listening did not reach the preview");
        SceneLook.hear(page, null);
        if (JSON.parse(page.editor.look).listen !== undefined)
            throw new Error("hearing: listening did not stop");
        SceneLook.restart = restarts;
        const path = JSON.parse(page.editor.rig("", "{\"wander\":{\"time\":2}}"));
        if (!(path.x >= 0 && path.x <= 1 && path.y >= 0 && path.y <= 1))
            throw new Error("hearing: the made-up pointer left the screen");
    }
}
