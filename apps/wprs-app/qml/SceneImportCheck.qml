import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function fail(text) {
        throw new Error("import: " + text);
    }

    function exercise() {
        const shared = Qt.createComponent("SceneShared.qml").createObject(page, {
                "page": page
            });
        shared.installed = [
            {
                "uid": "a",
                "title": "Stone"
            },
            {
                "uid": "b",
                "title": "Wood"
            }
        ];
        if (shared.listed.count !== 2)
            fail("the installed assets packages are not listed");
        shared.paths.text = "/nowhere/pack.wpak";
        shared.pick(shared.paths.text);
        if (shared.file !== "/nowhere/pack.wpak" || shared.uid !== "" || !shared.info?.error)
            fail("a typed path was not read");
        shared.destroy();
        const meter = Qt.createComponent("SceneBindMeter.qml").createObject(page, {
                "page": page
            });
        meter.uses = {
            "audio": true,
            "granted": []
        };
        if (!meter.canAllow)
            fail("the sound cannot be allowed in one click");
        meter.uses = {
            "audio": true,
            "granted": ["audio"]
        };
        if (meter.canAllow)
            fail("the sound can be allowed twice");
        meter.destroy();
        const field = Qt.createComponent("SceneField.qml").createObject(page, {
                "daemon": App,
                "output": "",
                "modelData": {
                    "kind": "bool",
                    "key": "k",
                    "label": "K",
                    "value": false
                }
            });
        field.flag.toggle();
        if (!field.flag.checked)
            fail("the switch did not change");
        field.modelData = {
            "kind": "bool",
            "key": "k",
            "label": "K",
            "value": false
        };
        if (field.flag.checked)
            fail("a reset left the switch as the person set it");
        field.destroy();
    }
}
