import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function followed(widget) {
        const sent = [];
        const fake = {
            "run": (name, args) => sent.push(args),
            "editor": {
                "time": 0,
                "value": () => "\"blue\"",
                "rig": () => "null",
                "hear": () => null
            },
            "view": {
                "properties": []
            }
        };
        const bind = Qt.createComponent("SceneBind.qml").createObject(page, {
                "page": fake,
                "field": {
                    "source": "",
                    "widget": widget,
                    "pointer": "/scene/x",
                    "value": "#4060ff",
                    "shown": "#4060ff",
                    "default": "#4060ff"
                }
            });
        bind.follow({
            "id": "sun",
            "pointer": "/general/properties/sun"
        });
        bind.destroy();
        return sent[0].value;
    }

    function exercise() {
        const plain = followed("color");
        if (JSON.stringify(plain) !== "{\"prop\":\"sun\"}")
            throw new Error("bind: a color following a property stored " + JSON.stringify(plain));
        if (!followed("emitters").map)
            throw new Error("bind: an emitter list following a property lost its map");
    }
}
