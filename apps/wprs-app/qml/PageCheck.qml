import QtQuick

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

    function exercise() {
        web();
        playback();
        inputs();
        network();
    }
}
