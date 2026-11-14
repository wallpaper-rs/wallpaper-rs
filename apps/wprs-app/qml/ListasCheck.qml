import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var page

    function must(ok, text) {
        if (!ok)
            throw new Error(text);
    }

    function keys() {
        must(page.shown.length > 1, "the playlists page shows fewer than two playlists to move between");
        page.shelf.covers.view.forceActiveFocus(Qt.TabFocusReason);
        App.backend.press(Qt.Key_Right, 0);
        must(page.shelf.covers.currentIndex === 1, "the Right key does not move to the next playlist");
        App.backend.press(Qt.Key_Left, 0);
        must(page.shelf.covers.currentIndex === 0, "the Left key does not move back to the first playlist");
    }

    function rotating() {
        const line = Qt.createComponent("AhoraLista.qml").createObject(page, {
            "screen": "DP-1",
            "playlist": {
                "uid": "x",
                "name": "Sea",
                "next_at": Date.now() / 1000 + 400
            }
        });
        const text = line.when();
        line.destroy();
        must(text === App.tr("List “%1” · changes in %2", "Sea", App.tr("%1 min", 7)), "the rotating line does not say when the list changes: " + text);
    }

    function ordering() {
        const list = page.editor.list;
        page.editor.items = [{
                "uid": "a",
                "title": "A"
            }, {
                "uid": "b",
                "title": "B"
            }];
        must(list.movable && list.otherText !== "" && list.addText !== "", "a manual playlist cannot be ordered or filled at level one");
        let moved = [];
        const seen = (from, to) => moved = [from, to];
        list.reordered.connect(seen);
        list.move(0, 1);
        list.move(0, -1);
        list.reordered.disconnect(seen);
        must(moved.length === 2 && moved[0] === 0 && moved[1] === 1, "moving a wallpaper of a playlist does not ask for its new place");
    }

    function exercise() {
        must(page.loaded && page.playlists.length === 2, "the playlists page did not load the playlists");
        must(page.shelf.filter.words[1].count === page.running.length && page.shelf.filter.words[0].count === 2, "the filter of the playlists does not count every word");
        must(page.bandLow > 0, "the playlists shelf does not use the low band");
        must(page.shelf.entries[0].caption.indexOf(page.shelf.every("30m")) >= 0, "the card of a playlist does not say how often it changes");
        page.kind = 1;
        must(page.shown.length === page.running.length, "the filter Running still shows stopped playlists");
        page.kind = 0;
        rotating();
        const first = page.playlists[0];
        page.opened = first.uid;
        must(page.skeleton === "options" && page.route.length === 2, "editing a playlist does not open its options");
        must(page.shelf.covers.visible === false && page.bandLow === 0, "the shelf stays under the playlist editor");
        ordering();
        page.level = "more";
        must(page.route.length === 3, "the More level of a playlist has no route");
        const rules = (App.schema.rules ?? []).length - 1;
        must(page.editor.more.rows.children.filter(row => row.kind !== undefined).length >= 4, "the More level does not show search, folder and removal as rows");
        must(rules > 0 && page.editor.more.rules.count === rules, "the More level does not show every rule of the playlist as a row");
        page.routeBack();
        must(page.level === "" && page.opened === first.uid, "going back from More closes the playlist");
        page.routeBack();
        must(page.opened === "", "going back does not return to the shelf");
        const all = page.playlists;
        page.playlists = [];
        must(page.shelf.empty && page.bandLow > 0, "the playlists page shows no empty state in the band without playlists");
        page.playlists = all;
    }
}
