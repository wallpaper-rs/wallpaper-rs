import QtQuick

QtObject {
    required property var page

    function listing() {
        return JSON.parse(page.editor.code("list", "{}"));
    }

    function written(path, text) {
        page.run("script", {
            "write": {
                "path": path,
                "text": text
            }
        });
        if (page.editor.error !== "")
            throw new Error("scripts: the code was not written: " + page.editor.error);
    }

    function settings() {
        const table = Qt.createComponent("SceneScriptParams.qml").createObject(page, {
            "page": page,
            "script": listing().scripts[0]
        });
        if (table.ids.join() !== "size,speed")
            throw new Error("scripts: the settings table lists " + table.ids);
        table.commit({
            "id": "speed",
            "type": "number",
            "default": 9,
            "min": 0,
            "max": 5
        });
        if (page.editor.error === "" || table.revision !== 1)
            throw new Error("scripts: a setting outside its range was accepted");
        table.commit({
            "id": "speed",
            "to": "pace"
        });
        if (listing().scripts[0].params.pace === undefined)
            throw new Error("scripts: a setting was not renamed");
        table.destroy();
    }

    function following() {
        const menu = Qt.createComponent("SceneBindScript.qml").createObject(page, {
            "page": page
        });
        menu.load();
        const bound = menu.rows.find(row => row.source.fn === "wave");
        if (!bound || bound.source.params.pace !== 1 || !menu.rows.some(row => row.source.fn === undefined))
            throw new Error("scripts: the binding menu lists " + JSON.stringify(menu.rows));
        menu.destroy();
    }

    function declared() {
        written("scripts/wave.js", "import { audio } from \"wprs\";\nexport const wave = () => audio.bass;");
        const report = JSON.parse(page.editor.code("audit", JSON.stringify({
            "path": "scripts/wave.js"
        })));
        if (report.synced || report.undeclared.uses.join() !== "audio" || report.unused.params.join() !== "pace,size")
            throw new Error("scripts: the audit reports " + JSON.stringify(report));
        page.run("script", {
            "declare": {
                "path": "scripts/wave.js"
            }
        });
        const after = JSON.parse(page.editor.code("audit", JSON.stringify({
            "path": "scripts/wave.js"
        })));
        if (after.undeclared.uses.length > 0)
            throw new Error("scripts: declaring left uses undeclared");
    }

    function templates() {
        page.run("add_layer", {
            "kind": "solid",
            "id": "dot"
        });
        page.run("template", {
            "make": {
                "layers": ["dot"]
            }
        });
        const tree = Qt.createComponent("SceneTemplateTree.qml").createObject(page, {
            "page": page
        });
        if (tree.templates.length !== 1 || tree.templates[0].layers.length !== 1)
            throw new Error("templates: the tree lists " + JSON.stringify(tree.templates));
        page.editor.select(tree.templates[0].pointer);
        if (!page.fields.some(field => field.key === "color"))
            throw new Error("templates: the inspector shows nothing of the template");
        page.run("template", {
            "rename": {
                "name": "dot_template",
                "to": "spot"
            }
        });
        page.run("template", {
            "remove": {
                "name": "spot"
            }
        });
        page.run("remove_layer", {
            "id": "dot"
        });
        tree.destroy();
    }

    function exercise() {
        page.run("import_script", {
            "file": "",
            "template": "spawn"
        });
        page.run("import_script", {
            "file": "",
            "template": "wave"
        });
        page.run("script", {
            "move": {
                "path": "scripts/wave.js",
                "to": 0
            }
        });
        const found = listing();
        if (found.scripts.map(s => s.path).join() !== "scripts/wave.js,scripts/spawn.js" || !found.scripts.every(s => s.synced))
            throw new Error("scripts: the list is " + JSON.stringify(found.scripts));
        settings();
        following();
        if (JSON.parse(page.editor.code("reference", "{}")).length < 5)
            throw new Error("scripts: the API reference is empty");
        if (page.editor.code("changes", "{}") === "[]")
            throw new Error("scripts: an import did not say what it changed");
        declared();
        templates();
        for (const path of ["scripts/wave.js", "scripts/spawn.js"])
            page.run("script", {
                "remove": {
                    "path": path
                }
            });
        if (listing().scripts.length !== 0 || listing().free.length !== 2)
            throw new Error("scripts: the picker does not offer the removed files");
    }
}
