import QtQuick
import io.github.wallpaper_rs.app

Timer {
    id: check

    required property var host
    property var shown: null
    property int step: 0
    property int tries: 0
    readonly property var calm: ({
            "model": 1,
            "serial": 2,
            "output": "DP-1",
            "active": true,
            "ended": null,
            "source": "playlist",
            "playlist_name": "Sea",
            "position": 0,
            "count": 2,
            "candidate": {
                "uid": "01J9Z0X9Z6Q7X4F3K5E1V0R1VD",
                "kind": "video",
                "title": "Waves",
                "imported": true
            },
            "now": null,
            "ahead": [],
            "plays": {
                "kind": "fade",
                "why": "calm",
                "duration_ms": 600
            }
        })
    readonly property var steps: [[() => {}, () => shown.began && shown.active && shown.mode !== "error" && shown.candidate.title === "Aurora" && shown.session.ahead.length === 1 && shown.kind === "scene" && shown.words.kindWord === App.tr("Scene") && shown.words.still === App.tr("Still picture of the scene: it moves once applied")], [() => shown.press(Qt.Key_Right), () => shown.mode === "transicion" && shown.candidate.title === "Waves" && shown.sample < 1], [() => shown.heard("DP-1", calm), () => shown.mode === "menos-movimiento" && !shown.sampling && shown.keyHints.length === 5], [() => shown.heard("HDMI-A-1", Object.assign({}, calm, {
                        "plays": {
                            "kind": "none",
                            "why": "chosen"
                        }
                    })), () => shown.calm], [() => shown.take(Object.assign({}, calm, {
                        "active": false,
                        "candidate": null
                    })), () => shown.mode === "vacio" && shown.press(Qt.Key_Return) && shown.left === 3], [() => shown.fail({
                    "message": "no answer"
                }), () => shown.mode === "error" && shown.failure === "no answer" && shown.keyHints.length === 2], [() => shown.press(Qt.Key_Return), () => shown.failure === "" && shown.active && shown.candidate.title === "Aurora"], [() => {
                shown.left = -1;
                shown.press(Qt.Key_Escape);
            }, () => shown.left === 0], [() => {
                shown.left = -1;
                shown.press(Qt.Key_Return);
            }, () => shown.left === 0], [() => {
                shown.left = -1;
                shown.heard("DP-1", {
                    "output": "DP-1",
                    "active": false,
                    "ended": "expired"
                });
            }, () => shown.left === 0]]

    function start() {
        host.waiting++;
        shown = Qt.createComponent("Preview.qml").createObject(null, {
            "output": "DP-1"
        });
        host.held = host.held.concat([check]);
        running = true;
    }

    function finish(problem) {
        running = false;
        host.waiting--;
        shown.destroy();
        host.held = host.held.filter(c => c !== check);
        if (problem)
            throw new Error("preview window: " + problem);
    }

    interval: 20
    repeat: true
    onTriggered: {
        if (!shown)
            return finish("the window did not load");
        if (!steps[step][1]())
            return ++check.tries > 200 ? finish("step " + step + " (" + shown.mode + ") did not settle") : undefined;
        check.tries = 0;
        if (++step < steps.length)
            return steps[step][0]();
        finish("");
    }
}
