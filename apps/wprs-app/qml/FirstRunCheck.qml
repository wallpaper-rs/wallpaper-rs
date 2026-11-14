import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var page
    property Item aside: Qt.createQmlObject("import QtQuick; FocusScope {}", page)

    function must(ok, text) {
        if (!ok)
            throw new Error(text);
    }

    function make(error, filtered) {
        return Qt.createComponent("LibraryState.qml").createObject(aside, {
            "page": {
                "library": {
                    "total": 0,
                    "loading": false,
                    "error": error
                },
                "search": () => {}
            },
            "bar": {
                "filtered": filtered,
                "text": ""
            }
        });
    }

    function states() {
        const fresh = make("", false);
        const broken = make("no answer", false);
        const found = [fresh.starting, fresh.failed, broken.starting, broken.failed];
        fresh.destroy();
        broken.destroy();
        must(found.join() === "true,false,false,true", "an empty library with no error does not show the start, or a failed one still does: " + found.join());
    }

    function restart() {
        const broken = make("no answer", false);
        const before = broken.fault.action;
        broken.fault.acted();
        const found = [before === App.tr("Restart the service"), broken.restarting, broken.fault.action === App.tr("Restarting the service…"), broken.fault.details];
        broken.destroy();
        must(found.join() === "true,true,true,no answer", "the service error does not offer to restart the service: " + found.join());
    }

    function shelf() {
        const placed = [];
        const start = Qt.createComponent("FirstStart.qml").createObject(aside, {
            "page": {},
            "adder": null,
            "run": {
                "shelf": [{
                        "id": "org.example.snow",
                        "title": "Snowfall",
                        "kind": "scene"
                    }],
                "previewed": {},
                "spec": pkg => pkg.id,
                "place": (pkg, auto) => placed.push(pkg.id + ":" + auto)
            }
        });
        const found = [start.examples[0].name, start.examples[0].caption === App.tr("Scene · Bundled examples"), start.shelf.currentIndex];
        start.put();
        start.destroy();
        found.push(placed.join());
        must(found.join() === "Snowfall,true,0,org.example.snow:false", "the examples' shelf does not light its first cover or set it: " + found.join());
    }

    function screens() {
        const run = Qt.createComponent("FirstRun.qml").createObject(page, {
            "bare": false
        });
        const one = run.told("Snowfall", ["DP-1"]);
        const two = run.told("Snowfall", ["DP-1", "HDMI-1"]);
        run.destroy();
        const found = [one.action === App.tr("Undo"), one.text.includes("DP-1"), two.action === App.tr("Apply to…"), two.text.includes("DP-1"), two.text.includes("HDMI-1")];
        must(found.join() === "true,false,true,true,true", "the first wallpaper's notice does not speak of the screens only when there are several: " + found.join());
    }

    function unnoticed() {
        const run = Qt.createComponent("FirstRun.qml").createObject(page, {
            "bare": true,
            "shelf": [{
                    "id": "org.example.first",
                    "title": "First",
                    "repo_id": "r",
                    "version": "1.0.0"
                }]
        });
        run.begin();
        must(run.phase === "", "a check began the first wallpaper by itself: " + run.phase);
        run.allowed = true;
        run.tried = true;
        run.begin();
        must(run.phase === "", "the first wallpaper is applied twice");
        run.destroy();
    }

    function notice() {
        const run = Qt.createComponent("FirstRun.qml").createObject(page, {
            "bare": false
        });
        const notices = applicationWindow().shell.notices;
        run.announce({
            "title": "Snowfall"
        });
        const found = [notices.text.includes("Snowfall"), notices.action, typeof notices.callback];
        notices.hide();
        run.destroy();
        must(found.join() === "true," + (App.outputs.length > 1 ? App.tr("Apply to…") : App.tr("Undo")) + ",function", "the first wallpaper does not announce itself with a way to undo it: " + found.join());
    }

    function exercise() {
        states();
        restart();
        shelf();
        screens();
        unnoticed();
        notice();
    }
}
