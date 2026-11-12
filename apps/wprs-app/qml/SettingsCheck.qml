import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    property var stage: null

    function same(found, wanted, what) {
        if (found !== wanted)
            throw new Error(what + ": " + JSON.stringify(found) + " instead of " + JSON.stringify(wanted));
    }

    function shown(info) {
        return Object.assign({
            "quality": "q2",
            "quality_reason": "user",
            "quality_cost": 0,
            "quality_budget": 0,
            "quality_auto": "",
            "quality_frame_ms": null,
            "quality_period_ms": null,
            "quality_step": "",
            "quality_pace": 1,
            "fps": 60,
            "left_out": []
        }, info);
    }

    function notes() {
        const reasons = Qt.createComponent("LeftOut.qml").createObject(check, {
            "daemon": App
        });
        const cost = App.tr("This scene costs %1 units a frame at this level, more than the %2 this device is estimated to hold: frames may drop and battery use rise.", "6.0", "4");
        const slow = App.tr("Its frames take %1 ms on the graphics card, more than the %2 ms a frame has at %3 frames a second: frames drop and battery use rises.", "25.0", "16.7", 60);
        const lower = App.tr("Automatic quality would lower it.");
        const still = App.tr("Automatic quality would keep it still at its poster.");
        const over = {
            "quality_cost": 6,
            "quality_budget": 4
        };
        const timed = {
            "quality_frame_ms": 25,
            "quality_period_ms": 16.7,
            "quality_step": "steady"
        };
        same(reasons.note(shown(Object.assign({
            "quality_auto": "lower"
        }, over))), cost + " " + lower, "an estimate over budget");
        same(reasons.note(shown(Object.assign({
            "quality_auto": "lower"
        }, over, timed))), slow + " " + lower, "frames measured too slow");
        same(reasons.note(shown(Object.assign({
            "quality_auto": ""
        }, over, timed))), "", "an estimate that measured frames disprove");
        same(reasons.note(shown(Object.assign({
            "quality_auto": "stop",
            "left_out": ["a", "b"]
        }, over))), [cost, still, App.tr("Effects left out to protect the graphics card stay out at every level: %1", 2)].join(" "), "a fixed level over a scene automatic quality keeps still");
        same(reasons.note(shown({
            "quality_auto": "stop"
        })), App.tr("This scene draws more model triangles than this device holds: frames may drop and battery use rise. Automatic quality would keep it still at its poster."), "model triangles alone");
        same(reasons.note(shown({
            "quality_auto": "still"
        })), App.tr("This scene is too heavy for this device, so automatic quality keeps it still at its poster. A fixed level plays it anyway, with fewer frames per second and more battery use."), "a still scene");
        same(reasons.note(shown({})), "", "a scene that fits");
        const steady = App.tr("The level holds: frames fit, without enough room for the next level.");
        same(reasons.timing(shown(timed)), App.tr("Measured GPU time: %1 ms a frame, for a target of %2 ms. %3", "25.0", "16.7", steady), "the measured time");
        same(reasons.timing(shown({
            "quality_step": "model"
        })), App.tr("GPU time not measured yet. %1", App.tr("The level comes from the estimate until frames are measured.")), "no measure yet");
        same(reasons.timing(shown({})), "", "other content");
        const paced = reasons.level(shown({
            "quality_pace": 2,
            "fps": 30
        }), {});
        same(paced.split(":")[0], App.tr("Quality %1 of 3 at %2 frames a second", "2", 30), "half the pace");
        reasons.destroy();
    }

    function lights(options, hints) {
        const panel = Qt.createComponent("SettingsRgb.qml").createObject(stage.contentItem, {
            "options": options
        });
        if (!panel)
            throw new Error("SettingsRgb.qml did not load");
        const texts = panel.children.filter(item => item.text !== undefined);
        const radios = texts.filter(item => item.checkable === true && item.autoExclusive === true);
        const hint = texts.find(item => hints.includes(item.text));
        const found = JSON.stringify([radios.map(r => r.enabled), radios.map(r => r.checked), hint?.text ?? ""]);
        panel.destroy();
        return found;
    }

    function covered() {
        stage = Qt.createQmlObject("import QtQuick; Window { width: 640; height: 480; visible: true }", check);
        const off = App.tr("Pausing while a window covers the screen is off, so the covered wallpaper keeps drawing and the lights keep following it whatever you choose here.");
        const follow = App.tr("While the covered wallpaper pauses, the lights keep changing with the scene or page, which keeps running without being drawn, as long as the server has devices connected.");
        const paused = App.tr("While the covered wallpaper pauses, nothing is sent and nothing runs for the lights until it shows again.");
        const hints = [off, follow, paused];
        same(lights({
            "pause.when_covered": false,
            "rgb.when_covered": "follow"
        }, hints), JSON.stringify([[false, false], [true, false], off]), "covered lights without the covered pause");
        same(lights({
            "rgb.when_covered": "follow"
        }, hints), JSON.stringify([[true, true], [true, false], follow]), "covered lights that follow");
        same(lights({
            "rgb.when_covered": "pause"
        }, hints), JSON.stringify([[true, true], [false, true], paused]), "covered lights that pause");
        stage.destroy();
    }

    function exercise() {
        notes();
        covered();
    }
}
