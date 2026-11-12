import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function expect(ok, what) {
        if (!ok)
            throw new Error(what + ": " + JSON.stringify(page.manifest));
    }

    function web() {
        page.manifestEdit("web", {
                "values": {
                    "preload": "app.js",
                    "background": "#102030",
                    "storage_kib": 128
                }
            });
        const set = page.manifest.web ?? {};
        expect(set.preload === "app.js" && set.background === "#102030" && set.storage_kib === 128, "the page settings were not kept");
        page.manifestEdit("web", {
                "clear": ["preload", "storage_kib"]
            });
        expect(page.manifest.web?.preload === undefined && page.manifest.web?.background === "#102030", "a page setting was not cleared");
    }

    function network() {
        page.manifestEdit("network", {
            "origins": ["https://API.example.org/"]
        });
        expect(page.manifest.permissions?.network?.allow?.[0] === "https://api.example.org" && page.manifest.inputs?.includes("network"), "an origin was not kept with the network input");
        page.manifestEdit("network", {
            "origins": ["https://api.example.org"],
            "remove": true
        });
        expect(page.manifest.permissions === undefined && !(page.manifest.inputs ?? []).includes("network"), "the last origin left the network input behind");
    }

    function inputs() {
        page.manifestEdit("inputs", {
            "inputs": ["audio", "pointer"],
            "add": true
        });
        expect((page.manifest.inputs ?? []).includes("audio") && page.manifest.inputs.includes("pointer"), "declared inputs were not kept");
        page.manifestEdit("inputs", {
            "inputs": ["audio", "pointer"],
            "remove": true
        });
        expect(!(page.manifest.inputs ?? []).includes("audio"), "a declared input was not removed");
    }

    function playback() {
        page.manifestEdit("playback", {
            "values": {
                "max_fps": 30,
                "volume": 0.4,
                "muted": true
            }
        });
        const set = page.manifest.playback ?? {};
        expect(set.max_fps === 30 && set.volume === 0.4 && set.muted === true, "the playback was not kept");
        page.manifestEdit("playback", {
            "clear": ["max_fps", "volume", "muted"]
        });
        expect(page.manifest.playback === undefined, "the playback was not cleared");
    }

    function preview() {
        const shown = Qt.createComponent("PagePreview.qml").createObject(page, {
            "page": page
        });
        shown.rate(30);
        expect(JSON.parse(page.editor.look).fps === 30, "the frame rate of the preview was not kept");
        shown.rate(0);
        expect(JSON.parse(page.editor.look).fps === undefined, "the frame rate of the preview was not released");
        shown.destroy();
    }

    function properties() {
        const listed = Qt.createComponent("SceneProperties.qml").createObject(page, {
            "page": page
        });
        listed.add("folder");
        expect(page.manifest.properties?.folder_1?.type === "folder", "a folder property was not added to the page");
        expect(listed.properties.some(p => p.id === "folder_1"), "the property is not listed");
        page.run("remove", {
            "pointer": "/manifest/properties/folder_1"
        });
        listed.destroy();
    }

    function media() {
        const player = Qt.createComponent("PageMedia.qml").createObject(page, {
            "page": page
        });
        player.change("title", "Made-up song");
        expect(SceneLook.media?.title === "Made-up song" && JSON.parse(page.editor.look).media?.title === "Made-up song", "the made-up track was not kept for the preview");
        SceneLook.media = null;
        SceneLook.preview(page);
        expect(JSON.parse(page.editor.look).media === undefined, "the made-up track was not released");
        player.destroy();
    }

    function exercise() {
        web();
        media();
        properties();
        preview();
        playback();
        inputs();
        network();
    }
}
