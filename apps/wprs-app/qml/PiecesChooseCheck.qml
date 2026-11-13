import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var host

    function fail(what) {
        throw new Error("pieces: " + what);
    }

    function press(key) {
        App.backend.press(key, 0);
    }

    function named(item) {
        if (!item.Accessible.name)
            fail("a piece has no name for assistive tools: " + item);
    }

    function exercise() {
        const page = Qt.createComponent("PiecesChoose.qml").createObject(host, {
                "width": 1600,
                "height": 900
            });
        if (!page)
            fail("the page of the pieces to choose does not load");
        [page.filter, page.search, page.shelf, page.grid].forEach(named);
        page.filter.forceActiveFocus();
        press(Qt.Key_Right);
        press(Qt.Key_Return);
        if (page.filter.current !== 1)
            fail("the arrows and Enter do not pick a filter word");
        let activated = -1;
        page.shelf.activated.connect(index => activated = index);
        page.shelf.forceActiveFocus();
        press(Qt.Key_Right);
        press(Qt.Key_Return);
        if (page.shelf.currentIndex !== 2 || activated !== 2)
            fail("the arrows and Enter do not move along the shelf and open the lit cover");
        if (page.shelf.currentName !== page.shelf.model[2].name)
            fail("the shelf does not name its lit cover");
        const chosen = page.grid.chosen.length;
        page.grid.forceActiveFocus();
        press(Qt.Key_Space);
        if (page.grid.chosen.length !== chosen - 1)
            fail("Space does not toggle a cover while choosing");
        press(Qt.Key_Right);
        if (page.grid.currentIndex !== 3)
            fail("the arrows do not move in the grid");
        let removed = false;
        page.chip.removed.connect(() => removed = true);
        page.chip.forceActiveFocus();
        press(Qt.Key_Return);
        if (!removed)
            fail("the remove button of a filter word does not answer the keyboard");
        page.destroy();
    }
}
