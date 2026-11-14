import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: more

    required property var provider
    required property string level
    readonly property var schema: ((App.schema.providers || []).find(p => p.kind === provider.kind) || ({})).options || []
    readonly property var shown: Object.assign({}, provider.options, values)
    readonly property Item rows: level === "" ? main : list
    readonly property Item listed: list
    property var values: ({})
    property var found: []
    property string owner: (provider.options.collection || "").split("/")[0]
    property string error

    signal deeper(string next)
    signal closed

    function enter() {
        rows.enter();
    }

    function field(key) {
        return schema.find(o => o.key === key) || ({
                "key": key,
                "kind": "text",
                "values": []
            });
    }

    function set(changes) {
        values = Object.assign({}, values, changes);
        App.act("Update", [provider.id, changes], "Providers1", () => more && (more.error = ""), e => {
            if (!more)
                return;
            more.values = {};
            more.error = e.message;
        });
    }

    function picked(key) {
        const f = field(key);
        const value = String(shown[key] ?? "");
        if (f.kind === "flags")
            return f.values.filter((v, at) => value[at] === "1");
        return value === "" ? [] : value.split(",");
    }

    function toggle(key, value) {
        const f = field(key);
        const on = picked(key);
        const next = on.includes(value) ? on.filter(v => v !== value) : on.concat([value]);
        set({
            [key]: f.kind === "flags" ? f.values.map(v => next.includes(v) ? "1" : "0").join("") : f.values.filter(v => next.includes(v)).join(",")
        });
    }

    function words(key) {
        const on = picked(key);
        return on.length === 0 ? App.tr("Any") : on.map(v => SourceMoreWords.choice(key, v)).join(" · ");
    }

    function collections(name) {
        owner = name;
        App.query("Collections", [provider.id, name], "Providers1", reply => {
            if (!more)
                return;
            more.found = reply.collections;
            more.error = "";
            more.deeper("collections");
        }, e => more && (more.error = e.message));
    }

    function collect(id) {
        const chosen = owner + "/" + id;
        set(shown.collection === chosen ? {
            "collection": ""
        } : {
            "collection": chosen,
            "query": "",
            "similar": ""
        });
    }

    implicitHeight: rows.implicitHeight + (problem.visible ? Look.e3 + problem.implicitHeight : 0)
    Keys.onEscapePressed: closed()

    SourceMoreMain {
        id: main

        more: more
        visible: more.level === ""
    }

    SourceMoreList {
        id: list

        more: more
        visible: more.level !== ""
    }

    Nota {
        id: problem

        anchors.top: more.rows.bottom
        anchors.topMargin: Look.e3
        visible: more.error !== ""
        tone: "coral"
        text: more.error
    }
}
