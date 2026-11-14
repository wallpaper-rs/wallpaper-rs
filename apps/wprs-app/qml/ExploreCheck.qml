import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var page
    readonly property var catalog: page.catalog
    readonly property var installer: page.installer
    readonly property var examples: ({
            "repo_id": "aa11",
            "name": "",
            "url": "file:///examples",
            "bundled": true,
            "hosts": [],
            "fingerprints": ["ab"],
            "warnings": []
        })
    readonly property var house: ({
            "repo_id": "bb22",
            "name": "Home",
            "url": "https://casa.example/fondos",
            "hosts": ["casa.example"],
            "fingerprints": ["cd"],
            "warnings": []
        })

    function pkg(repo, id, extra) {
        return Object.assign({
            "repo_id": repo,
            "id": id,
            "title": id,
            "author": "Ana",
            "version": "1.0.0",
            "versions": ["1.0.0"],
            "installed": [],
            "asks": [],
            "size": 2048
        }, extra ?? {});
    }

    function expect(state, ok, what) {
        if (!ok)
            throw new Error("explore, " + state + ": " + what);
    }

    function fill(repos, packages, installed) {
        catalog.repos = repos;
        catalog.packages = packages;
        catalog.total = packages.length;
        catalog.installed = installed;
        catalog.word = 0;
    }

    function motion() {
        fill([examples, house], [pkg("bb22", "dunes", {
                "animated": true
            }), pkg("bb22", "flash", {
                "animated": true,
                "photosensitive": true
            })], []);
        catalog.asked = {};
        catalog.moving = {};
        page.shelf.wanted(0);
        page.shelf.wanted(1);
        expect("vista", catalog.asked["bb22/dunes@1.0.0"] === true && !catalog.asked["bb22/flash@1.0.0"], "a card under the pointer must ask for its animated preview, unless it flashes");
        catalog.moving = {
            "bb22/dunes@1.0.0": "/cache/dunes.gif",
            "bb22/flash@1.0.0": "/cache/flash.gif"
        };
        expect("vista", page.shelf.movingOf(0) !== "" && page.shelf.movingOf(1) === "", "only the card that may move gets its animated preview");
        catalog.hidden = 2;
        expect("oculto", page.upperBlock.hidden.visible && page.upperBlock.hidden.text === App.tr("%1 hidden by your content filter", 2), "the wallpapers the content filter hid must be counted");
        catalog.hidden = 0;
    }

    function copying() {
        const copy = Qt.createComponent("BrowseCopy.qml").createObject(check.page, {
            "catalog": catalog,
            "wallpaper": pkg("bb22", "dunes", {
                "kind": "scene"
            })
        });
        const button = Qt.createComponent("ItemCopyButton.qml").createObject(null, {
            "drawer": copy
        });
        expect("copia", copy.record === null && !button.visible, "a scene that is not installed must not offer a copy");
        catalog.installed = [{
                "repo_id": "bb22",
                "id": "dunes",
                "version": "1.0.0",
                "item": "AB12CD"
            }];
        copy.item = {
            "path": "/library/dunes.wpak"
        };
        expect("copia", copy.record !== null && button.visible, "an installed scene must offer Edit a copy");
        copy.wallpaper = pkg("bb22", "dunes", {
            "kind": "image"
        });
        expect("copia", !button.visible, "a package that is not a scene must not offer a copy");
        button.destroy();
        copy.destroy();
    }

    function exercise() {
        const notes = Qt.createComponent("BrowseNotes.qml").createObject(check.page);
        fill([examples], [pkg("aa11", "pond")], []);
        expect("vacio", catalog.lone && catalog.fresh && !page.upperBlock.visible && page.bar.link.visible, "a first look must hide the upper block and offer the link");
        expect("vacio", catalog.words.length === 1 && catalog.words[0].text === App.tr("Examples"), "the bundled repository must be called Examples");
        expect("primero", catalog.covers.length === 1 && catalog.covers[0].name === "pond", "the example must be on the shelf");
        installer.tell(App.tr("“%1” is now your wallpaper.", "pond"), {}, true);
        expect("primero", installer.said[installer.said.length - 1].includes("pond"), "setting the first wallpaper must be said");
        fill([examples, house], [pkg("aa11", "pond"), pkg("bb22", "dunes", {
                "featured": true,
                "asks": ["network"],
                "photosensitive": true,
                "flash_disabled": true
            })], [{
                "repo_id": "aa11",
                "id": "pond",
                "version": "1.0.0"
            }]);
        expect("normal", page.upperBlock.visible && catalog.words.map(w => w.kind).join() === "featured,all,repo,repo", "the filter words are " + catalog.words.map(w => w.kind));
        expect("normal", catalog.covers.length === 1 && catalog.covers[0].name === "dunes", "Featured must come first");
        expect("detalles", notes.of(catalog.chosen[0])[0] === App.tr("Has flashes, disabled"), "a wallpaper with its flashes off must say so: " + notes.of(catalog.chosen[0])[0]);
        page.open = true;
        page.at = 0;
        catalog.word = 1;
        expect("detalles", catalog.covers.length === 2, "All must list both");
        page.at = 1;
        expect("detalles", !page.open, "moving to another wallpaper must fold its details");
        page.flip();
        expect("todos", page.full && page.grid.visible && !page.shelf.visible, "See all must show the grid");
        page.flip();
        expect("todos", !page.full && page.shelf.visible, "See the shelf must come back");
        expect("instalado", notes.here(catalog.packages[0]) === false && notes.here(pkg("aa11", "pond", {
                "installed": ["1.0.0"]
            })), "an installed wallpaper must be recognised");
        expect("anterior", notes.older(pkg("aa11", "pond", {
                "installed": ["2.0.0"]
            })), "a newer installed version must be noticed");
        installer.watching = [{
                "spec": "bb22/dunes@1.0.0",
                "job": 7,
                "title": "dunes",
                "apply": false
            }];
        expect("instalando", installer.running(catalog.packages[1]) !== null, "a watched install must show its progress");
        installer.announce([{
                "id": 7,
                "state": "done",
                "target": "bb22/dunes@1.0.0"
            }]);
        expect("instalando", installer.watching.length === 0 && installer.said[installer.said.length - 1] === App.tr("“%1” is installed.", "dunes"), "a finished install must be confirmed");
        catalog.installed = catalog.installed.concat([{
                "repo_id": "bb22",
                "id": "dunes",
                "version": "1.0.0",
                "update": "1.1.0",
                "asks": []
            }]);
        expect("instalados", catalog.words[catalog.words.length - 1].kind === "updates" && catalog.plain.length === 1, "pending updates must get their own word");
        installer.watching = [{
                "spec": "bb22/dunes@1.0.0",
                "job": 8,
                "title": "dunes",
                "author": "Ana",
                "repoName": "Home"
            }];
        installer.announce([{
                "id": 8,
                "state": "failed",
                "error": {
                    "message": "signature mismatch"
                }
            }]);
        expect("firma", page.trouble && page.fallo.kind === "signature" && page.fallo.title === App.tr("The signature does not match"), "a bad signature must be explained");
        page.fallo.otherActed();
        expect("firma", installer.failure === null && !page.trouble, "Not now must close it");
        fill([examples, Object.assign({}, house, {
                "last_error": "connection refused"
            })], [], []);
        catalog.word = 2;
        expect("error", page.fallo.kind === "offline" && page.fallo.title === App.tr("No connection with “%1”", "Home"), "an offline repository must say so: " + page.fallo.title);
        page.fallo.otherActed();
        expect("error", catalog.picked.repo.bundled, "See the examples must go to them");
        fill([examples, Object.assign({}, house, {
                "broken": "bad index"
            })], [], []);
        catalog.word = 2;
        expect("repo", page.fallo.kind === "unreadable" && catalog.words[2].count === App.tr("cannot be read"), "an unreadable repository must say so");
        motion();
        copying();
        installer.said = [];
        notes.destroy();
        catalog.load();
    }
}
