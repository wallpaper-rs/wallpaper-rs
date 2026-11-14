pragma Singleton

import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    readonly property var catalog: [{
        "kind": "bing",
        "icon": "sun-horizon",
        "name": "Bing",
        "line": App.tr("The picture of the day"),
        "title": App.tr("Bing: the picture of the day"),
        "quick": true
    }, {
        "kind": "apod",
        "icon": "planet",
        "name": "NASA",
        "line": App.tr("Astronomy picture of the day"),
        "title": App.tr("NASA: astronomy picture of the day"),
        "quick": true
    }, {
        "kind": "wikimedia",
        "icon": "books",
        "name": "Wikimedia Commons",
        "line": App.tr("Picture of the day"),
        "title": App.tr("Wikimedia Commons: picture of the day"),
        "quick": true
    }, {
        "kind": "wallhaven",
        "icon": "images-square",
        "name": "Wallhaven",
        "line": App.tr("Search pictures by topic"),
        "title": App.tr("Wallhaven: your search"),
        "quick": false
    }, {
        "kind": "nextcloud",
        "icon": "cloud",
        "name": "Nextcloud",
        "line": App.tr("Your photo folder"),
        "title": App.tr("Nextcloud: your folder"),
        "quick": false
    }, {
        "kind": "webdav",
        "icon": "hard-drives",
        "name": "WebDAV",
        "line": App.tr("A folder on your server"),
        "title": App.tr("WebDAV: your folder"),
        "quick": false
    }, {
        "kind": "immich",
        "icon": "images",
        "name": "Immich",
        "line": App.tr("Your photo library"),
        "title": App.tr("Immich: your library"),
        "quick": false
    }]

    function of(kind) {
        return catalog.find(k => k.kind === kind) || {
            "kind": kind,
            "icon": "images",
            "name": kind,
            "line": "",
            "title": kind,
            "quick": false
        };
    }

    function heading(provider) {
        const k = of(provider.kind);
        return new RegExp("^" + provider.kind + "(-[0-9]+)?$").test(provider.id) ? k.title : provider.id;
    }

    function slug(text) {
        const s = text.toLowerCase().replace(/[^a-z0-9_-]+/g, "-").replace(/^[^a-z]+/, "");
        return s.slice(0, 32);
    }

    function fresh(kind, taken) {
        for (let n = 1; ; n++) {
            const id = n === 1 ? kind : kind + "-" + n;
            if (!taken.includes(id))
                return id;
        }
    }

    function ago(seconds) {
        if (!seconds)
            return App.tr("never");
        const d = Math.max(0, Date.now() / 1000 - seconds);
        if (d < 90)
            return App.tr("a moment ago");
        if (d < 5400)
            return App.tr("%1 min ago", Math.round(d / 60));
        if (d < 129600)
            return App.tr("%1 h ago", Math.round(d / 3600));
        return App.tr("%1 d ago", Math.round(d / 86400));
    }
}
