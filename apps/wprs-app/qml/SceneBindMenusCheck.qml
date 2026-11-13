import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    readonly property var fake: ({
            "run": () => null,
            "editor": {
                "sweeping": false,
                "code": () => JSON.stringify({
                        "scripts": [
                            {
                                "path": "scripts/wave.js",
                                "functions": ["wave"]
                            }
                        ]
                    }),
                "rig": () => "{}",
                "select": () => null
            }
        })

    function entry(menu, text) {
        for (let i = 0; i < menu.count; i++) {
            if (menu.itemAt(i).text === text)
                return menu.itemAt(i);
        }
        const seen = [];
        for (let i = 0; i < menu.count; i++)
            seen.push(menu.itemAt(i).text);
        throw new Error("bind menus: no entry " + text + " among " + JSON.stringify(seen));
    }

    function made(file, field) {
        const given = field === undefined ? {
            "page": fake
        } : {
            "page": fake,
            "field": field
        };
        const menu = Qt.createComponent(file).createObject(page, given);
        menu.popup();
        return menu;
    }

    function exercise() {
        const script = made("SceneBindScript.qml");
        script.load();
        let chosen = null;
        script.chosen.connect(source => chosen = source);
        script.itemAt(2).triggered();
        if (!chosen || chosen.fn !== "wave")
            throw new Error("bind menus: choosing a script did nothing");
        script.destroy();
        const time = made("SceneBindTime.qml", {
                "widget": "number"
            });
        let picked = null;
        time.picked.connect(source => picked = source);
        entry(time, time.parts.hour).triggered();
        if (!picked || picked.clock !== "hour")
            throw new Error("bind menus: choosing a part of the time did nothing");
        time.destroy();
        const trial = made("SceneBindTry.qml", {
                "widget": "number",
                "source": "audio",
                "pointer": "/x"
            });
        let watched = false;
        trial.watch.connect(() => watched = true);
        entry(trial, App.tr("Watch the levels of the sound…")).triggered();
        if (!watched)
            throw new Error("bind menus: watching the levels did nothing");
        trial.destroy();
    }
}
