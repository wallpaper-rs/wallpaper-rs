pragma Singleton
import QtQuick

QtObject {
    property string quality: "q3"
    property int restart: 0
    property bool battery: false
    property bool calm: false
    property bool rival: false
    property string versus: "q1"
    property bool versusGlow: false
    readonly property string effective: battery && levels.indexOf(quality) > 2 ? "q2" : quality
    property var media: null
    property var values: null
    property string preset: ""
    readonly property var sample: ({
            "title": "Night Drive",
            "artist": "Example Artist",
            "album": "Example Album",
            "playing": true,
            "position": 65,
            "duration": 200
        })
    readonly property var levels: ["q0", "q1", "q2", "q3"]

    function preview(page) {
        const look = JSON.parse(page.editor.look || "{}");
        look.quality = rival ? versus : quality;
        look.restart = restart;
        look.battery = battery;
        look.calm = calm;
        if (rival && versusGlow)
            look.bloom = false;
        else
            delete look.bloom;
        if (media)
            look.media = media;
        else
            delete look.media;
        if (values !== null || preset !== "") {
            look.values = values ?? {};
            look.preset = preset;
        } else {
            delete look.values;
            delete look.preset;
        }
        page.editor.look = JSON.stringify(look);
        page.editor.seek(page.clock, page.editor.playing);
    }

    function again(page) {
        restart += 1;
        preview(page);
    }

    function tryValue(page, key, value) {
        const next = Object.assign({}, values ?? {});
        next[key] = value;
        values = next;
        preview(page);
    }

    function tryPreset(page, id) {
        values = {};
        preset = id;
        preview(page);
    }

    function untry(page) {
        values = null;
        preset = "";
        preview(page);
    }
}
