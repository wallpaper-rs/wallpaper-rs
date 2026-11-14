pragma Singleton

import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    readonly property string reserved: "LicenseRef-all-rights-reserved"
    readonly property var licenses: [
        {
            "id": "CC0-1.0",
            "name": "CC0",
            "says": App.tr("anyone may use it and change it, without naming you")
        },
        {
            "id": "CC-BY-4.0",
            "name": "CC BY 4.0",
            "says": App.tr("they may use it and change it if they name you")
        },
        {
            "id": "CC-BY-SA-4.0",
            "name": "CC BY-SA 4.0",
            "says": App.tr("the same, and their changes carry this license")
        },
        {
            "id": "CC-BY-NC-4.0",
            "name": "CC BY-NC 4.0",
            "says": App.tr("the same, but without selling it")
        },
        {
            "id": reserved,
            "name": App.tr("All rights"),
            "says": App.tr("they may use it, not change it or publish it")
        },
        {
            "id": "CC-BY-NC-SA-4.0",
            "name": "CC BY-NC-SA 4.0",
            "says": App.tr("without selling it, and their changes carry this license")
        },
        {
            "id": "CC-BY-ND-4.0",
            "name": "CC BY-ND 4.0",
            "says": App.tr("they may share it if they name you, without changing it")
        },
        {
            "id": "CC-BY-NC-ND-4.0",
            "name": "CC BY-NC-ND 4.0",
            "says": App.tr("they may share it if they name you, without changing it or selling it")
        },
        {
            "id": "MIT",
            "name": "MIT",
            "says": App.tr("anyone may use it and change it if they keep your notice")
        },
        {
            "id": "Apache-2.0",
            "name": "Apache 2.0",
            "says": App.tr("like MIT, and they note what they changed")
        },
        {
            "id": "GPL-3.0-or-later",
            "name": "GPL 3.0",
            "says": App.tr("anyone may change it, and every shared version stays free")
        }
    ]

    function escaped(text) {
        return String(text ?? "").replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
    }

    function licenseName(id) {
        const known = licenses.find(l => l.id === id);
        return escaped(known ? known.name : id);
    }

    function text(value) {
        if (typeof value === "string")
            return value;
        const all = value ?? {};
        const language = Qt.locale().name.split("_")[0];
        return all[language] ?? all.en ?? Object.values(all)[0] ?? "";
    }

    function title(manifest, file) {
        const own = text(manifest.title);
        return own !== "" ? own : String(file).split("/").pop().replace(/\.wpak$/, "");
    }

    function base(path) {
        return String(path).split("/").pop();
    }

    function parent(path) {
        return String(path).replace(/\/+$/, "").split("/").slice(0, -1).join("/");
    }

    function folderName(path) {
        const parts = String(path).split("/");
        return parts[parts.length - 2] ?? "";
    }

    function nextVersion(version) {
        const parts = String(version).split(".");
        const last = parseInt(parts[parts.length - 1], 10);
        if (isNaN(last))
            return version + ".1";
        parts[parts.length - 1] = String(last + 1);
        return parts.join(".");
    }

    function thumbnail(manifest) {
        return String((manifest.previews ?? {}).thumbnail ?? manifest.preview ?? "");
    }

    function authorId(name) {
        const id = String(name).toLowerCase().replace(/[^a-z0-9_-]+/g, "-").replace(/^[^a-z]+/, "");
        return (id !== "" ? id : "author").slice(0, 32);
    }

    function said(error) {
        return String(error ?? "").toLowerCase();
    }

    function upload(error) {
        const text = said(error);
        if (text.includes("already published") || text.includes("already exists"))
            return "version";
        if (text.includes("no author key") || text.includes("unknown author key"))
            return "registro";
        if (["connect", "unreachable", "cannot reach", "timed out", "dns", "resolve", "network", "os error"].some(word => text.includes(word)))
            return "red";
        return "otro";
    }
}
