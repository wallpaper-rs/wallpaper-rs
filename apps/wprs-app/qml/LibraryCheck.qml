import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var page
    readonly property var library: page.library
    readonly property var covers: page.covers
    property Timer later: Timer {
        interval: 150
        onTriggered: check.close()
    }

    function must(ok, text) {
        if (!ok)
            throw new Error(text);
    }

    function keys() {
        must(library.total > 0, "the library shows no wallpaper to move to with the keyboard");
        covers.shelf.view.forceActiveFocus(Qt.TabFocusReason);
        App.backend.press(Qt.Key_A, Qt.ControlModifier);
        must(library.selected === library.total && page.choosing, "Ctrl+A does not choose every wallpaper of the library");
        App.backend.press(Qt.Key_Escape, 0);
        must(library.selected === 0 && !page.choosing, "Escape does not clear the choice of the library");
        App.backend.press(Qt.Key_Down, 0);
        must(page.all && page.full, "the Down key does not show every wallpaper");
        App.backend.press(Qt.Key_Escape, 0);
        must(!page.all, "Escape does not bring the shelf back");
        must(page.keyHints.length === 3 && page.keyHints[1].keys[0] === "down", "the shelf does not show its key hints");
    }

    function exercise() {
        screens();
        several();
        options();
        empty();
        damaged();
        page.tools.shown();
    }

    function screens() {
        covers.shelf.currentIndex = 0;
        const uid = library.uid(0);
        library.showing(JSON.stringify({
            "DP-1": uid,
            "HDMI-A-1": "other"
        }));
        const entry = page.entry;
        const found = [entry.inUse, entry.usedOn];
        page.shown();
        must(found.join() === "true,DP-1", "a wallpaper in use does not name its screen when there are two: " + found.join());
    }

    function several() {
        page.toggle(0);
        page.toggle(1);
        must(page.chosen.length === 2 && page.choosing, "choosing two wallpapers does not show them chosen");
        page.stop();
        must(page.chosen.length === 0 && !page.choosing, "Stop choosing does not clear the choice");
    }

    function options() {
        page.open(library.uid(0));
        must(page.skeleton === "options" && page.route.length === 2 && page.actions.length === 2, "the options of a wallpaper do not open in their own panel");
        page.level = "more";
        must(page.route.length === 3, "the More level of the options is not in the route");
        later.start();
    }

    function close() {
        page.routeBack();
        must(page.level === "" && page.opened !== "", "Escape does not go back from the More level");
        page.routeBack();
        must(page.opened === "" && page.skeleton === "", "Escape does not close the options of a wallpaper");
    }

    function empty() {
        const make = (error, filtered) => Qt.createComponent("LibraryState.qml").createObject(page, {
                "page": {
                    "library": {
                        "total": 0,
                        "loading": false,
                        "error": error
                    }
                },
                "bar": {
                    "filtered": filtered,
                    "text": "zz"
                }
            });
        const none = make("", true);
        const broken = make("no answer", false);
        const found = [none.visible, none.unmatched, none.failed, broken.failed];
        none.destroy();
        broken.destroy();
        must(found.join() === "true,true,false,true", "a search with no results or a failed library does not show its state: " + found.join());
    }

    function damaged() {
        const card = Qt.createComponent("LibraryCard.qml").createObject(page, {
            "page": {
                "target": "",
                "entry": {
                    "uid": "01J9Z0X9Z6Q7X4F3K5E1V0R1MS",
                    "title": "Lost",
                    "missing": false,
                    "broken": "the picture header is cut short"
                }
            }
        });
        const found = [card.hurt, card.main];
        card.destroy();
        must(found.join() === "true," + App.tr("Show in folder"), "a damaged file does not offer to show its folder: " + found.join());
    }
}
