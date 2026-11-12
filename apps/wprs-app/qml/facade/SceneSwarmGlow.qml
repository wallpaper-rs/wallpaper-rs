pragma Singleton
import QtQuick

QtObject {
    id: root

    property var info: null
    property bool shown: false
    property string layer: ""

    function load(page, system, layer) {
        root.layer = layer || "";
        info = system ? JSON.parse(page.editor.rig(system, JSON.stringify({
            "swarm_light": {
                "system": system,
                "layer": layer || null
            }
        })) || "null") : null;
    }

    function run(page, what, args, gesture) {
        page.run("particles", {
                [what]: Object.assign({
                        "system": info?.system ?? ""
                    }, layer ? {
                        "layer": layer
                    } : {}, args)
            }, gesture === true);
    }
}
