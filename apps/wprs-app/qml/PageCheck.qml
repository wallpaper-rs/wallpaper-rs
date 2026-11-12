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

    function exercise() {
        web();
    }
}
