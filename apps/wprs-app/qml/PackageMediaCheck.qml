import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function expect(ok, what) {
        if (!ok)
            throw new Error(what + ": " + JSON.stringify(page.manifest));
    }

    function made(file) {
        return Qt.createComponent(file).createObject(page, {
                "page": page
            });
    }

    function display() {
        const framing = made("PackageFraming.qml");
        framing.put({
                "fit": "fit",
                "flip": "horizontal",
                "background": "#102030"
            });
        expect(page.manifest.display?.fit === "fit" && page.manifest.display?.flip === "horizontal", "the display was not kept");
        if (page.kind === "video") {
            framing.put({
                    "zoom": 2
                });
            framing.place(0.5, -0.5);
            expect(page.manifest.display.zoom === 2 && page.manifest.display.position[0] === 0.5, "the part shown was not kept");
        }
        framing.put({}, ["fit", "flip", "background", "zoom", "position"]);
        expect(page.manifest.display === undefined, "the display was not cleared");
        framing.destroy();
    }

    function clip() {
        const line = made("PackageClip.qml");
        line.trim("start", 2);
        line.trim("end", 5);
        expect(page.manifest.playback?.start === 2 && page.manifest.playback?.end === 5, "the trim was not kept");
        line.trim("end", 1000);
        expect(page.manifest.playback?.end === undefined, "the end of the video was not released");
        page.manifestEdit("playback", {
                "values": {
                    "speed": 0.5,
                    "loop": false
                }
            });
        expect(page.manifest.playback.speed === 0.5 && page.manifest.playback.loop === false, "the speed was not kept");
        page.manifestEdit("playback", {
                "clear": ["start", "speed", "loop"]
            });
        expect(page.manifest.playback === undefined, "the playback was not cleared");
        line.destroy();
    }

    function presets() {
        const list = made("PackagePresets.qml");
        list.add();
        expect((page.manifest.presets ?? []).length === 1 && page.editor.selected === "/manifest/presets/0", "the preset was not added");
        const speed = page.fields.find(f => f.key === "speed");
        expect(speed?.widget === "number" && speed.pointer === "/manifest/presets/0/values/speed", "the preset has no speed row");
        page.change(speed.pointer, 0.5);
        expect(page.manifest.presets[0].values.speed === 0.5, "the preset value was not kept");
        page.run("remove", {
                "pointer": "/manifest/presets/0"
            });
        list.destroy();
    }

    function file() {
        const chosen = made("PackageMediaFile.qml");
        expect(chosen.info?.present === false, "a new package has a file");
        chosen.destroy();
    }

    function exercise() {
        file();
        display();
        if (page.kind === "video") {
            clip();
            presets();
        }
        const shown = made("PackagePreview.qml");
        shown.rate(30);
        shown.rate(0);
        shown.destroy();
    }
}
