import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var host

    readonly property var mine: [{
            "folder": "/tmp/one",
            "title": "One",
            "kind": "scene",
            "version": "1.0.0",
            "edited": 1,
            "poster": "",
            "missing": false
        }, {
            "folder": "/tmp/two",
            "title": "Two",
            "kind": "scene",
            "version": "",
            "edited": 0,
            "poster": "",
            "missing": true
        }]
    readonly property var examples: [{
            "uid": "a",
            "title": "Pond",
            "thumb": "",
            "tags": [],
            "package": {
                "id": "io.github.wallpaper_rs.example.pond",
                "license": "CC0-1.0",
                "version": "1.0.0",
                "kind": "scene"
            }
        }]

    function build(projects, scenes) {
        return Qt.createComponent("CreatePage.qml").createObject(host, {
                "fixed": true,
                "projects": projects,
                "scenes": scenes
            });
    }

    function expect(ok, what) {
        if (!ok)
            throw new Error("create: " + what);
    }

    function exercise() {
        const empty = build([], []);
        empty.reset();
        expect(!empty.learn, "an empty profile without examples starts on the starters");
        expect(empty.facts.sceneModel.length === 4, "the scene starters are four");
        expect(empty.facts.singleModel.length === 3, "the single-file starters are three");
        expect(empty.facts.moreModel.length === 2, "the extra starters are two");
        expect(empty.facts.sceneModel.filter(entry => entry.picture !== undefined).length === 3, "three scene starters show what they draw");
        const first = build([], examples);
        first.reset();
        expect(first.learn && first.atExamples, "a first visit with examples opens on the examples");
        expect(first.facts.learnModel.length === 1, "the examples are listed");
        const late = build([], []);
        late.reset();
        late.facts.take(examples);
        expect(late.learn && late.atExamples && late.facts.learnModel.length === 1, "examples that arrive after the page opened are listed and lead");
        late.destroy();
        const tiles = empty.focused;
        tiles.currentIndex = 1;
        empty.foot.rename(true);
        expect(empty.changing && empty.foot.titleField.activeFocus, "Change… opens the name fields with the focus in the title");
        App.backend.press(Qt.Key_Escape, 0);
        expect(!empty.changing && empty.focused === tiles && tiles.activeFocus && tiles.currentIndex === 1, "Esc in the name fields goes back to the same tile");
        expect(empty.where.indexOf("/") >= 0, "the caption names the folder path: " + empty.where);
        const used = build(mine, examples);
        used.reset();
        expect(!used.learn, "a profile with projects opens on them");
        expect(used.facts.recentModel.length === 2, "the recent projects are listed");
        expect(!used.missing, "the newest project is present");
        expect(used.named === App.tr("My scene %1", 3), "the default title counts the projects: " + used.named);
        used.destroy();
        first.destroy();
        empty.destroy();
    }
}
