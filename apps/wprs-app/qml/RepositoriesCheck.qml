import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var page

    function expect(ok, what) {
        if (!ok)
            throw new Error("repositories: " + what);
    }

    function exercise() {
        page.repos = [{
                "repo_id": "aa11",
                "name": "",
                "url": "file:///examples",
                "bundled": true,
                "packages": 5,
                "hosts": [],
                "fingerprints": ["ab"],
                "warnings": []
            }, {
                "repo_id": "bb22",
                "name": "Home",
                "url": "https://casa.example/fondos",
                "packages": 2,
                "hosts": ["casa.example"],
                "fingerprints": ["cd"],
                "warnings": [],
                "last_error": "connection refused"
            }];
        expect(page.list.count === 2, "each repository must get its row");
        const bundled = page.list.itemAt(0);
        const home = page.list.itemAt(1);
        expect(bundled.row.text === App.tr("Examples") && !bundled.remove.visible, "the examples must keep their name and cannot be removed");
        expect(home.row.text === "Home" && home.row.note === "casa.example" && home.row.statusTone === "coral", "a failing repository must show its host and its trouble");
        expect(page.examples, "with the examples present there is nothing to add back");
        page.repos = [page.repos[1]];
        expect(!page.examples, "without the examples the page must offer them again");
        const shown = Qt.createComponent("RepoAddress.qml").createObject(page, {
            "text": "file:///repo/?fingerprint=ab&repo_id=cd"
        });
        shown.copy();
        const board = Qt.createQmlObject("import QtQuick; TextEdit { visible: false }", page);
        board.paste();
        expect(shown.copied && board.text === "file:///repo/?fingerprint=ab&repo_id=cd", "the repository address was not copied: " + board.text);
        board.destroy();
        shown.destroy();
        page.load();
    }
}
