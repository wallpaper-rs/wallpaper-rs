import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var page

    function must(ok, text) {
        if (!ok)
            throw new Error(text);
    }

    function make(error, filtered) {
        return Qt.createComponent("LibraryState.qml").createObject(page, {
            "page": {
                "library": {
                    "total": 0,
                    "loading": false,
                    "error": error
                }
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
        run.saved.done = true;
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
        must(found.join() === "true," + App.tr("Undo") + ",function", "the first wallpaper does not announce itself with a way to undo it: " + found.join());
    }

    function exercise() {
        states();
        unnoticed();
        notice();
    }
}
