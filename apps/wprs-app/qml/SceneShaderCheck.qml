import QtQuick

QtObject {
    required property var page

    function ask(query) {
        return page.editor.rig("", JSON.stringify(query));
    }

    function exercise() {
        page.run("shader", {
                "new": {
                    "name": "bend",
                    "template": "distort"
                }
            });
        if (page.editor.error !== "")
            throw new Error("shaders: no effect was made: " + page.editor.error);
        const code = Qt.createComponent("SceneShaderCode.qml").createObject(page, {
                "page": page
            });
        code.show("bend");
        if (code.saved.indexOf("fn uv_bend") < 0)
            throw new Error("shaders: the code view shows " + code.saved);
        code.reported(ask({
                    "shader_check": {
                        "effect": "bend",
                        "wgsl": "fn uv_bend(uv: vec2<f32>) -> vec2<f32> {\n    return nope;\n}\n"
                    }
                }));
        if (code.line !== 2 || code.report.ok)
            throw new Error("shaders: the error is not on its line: " + JSON.stringify(code.report));
        code.close();
        code.destroy();
        const form = Qt.createComponent("SceneEffectForm.qml").createObject(page, {
                "page": page
            });
        form.show("bend");
        form.param("spin", {
                "set": {
                    "type": "float",
                    "unit": "degrees"
                }
            });
        if (!(form.parsed.params ?? []).some(p => p.id === "spin" && p.unit === "degrees"))
            throw new Error("shaders: the form did not add a dial: " + JSON.stringify(form.parsed));
        page.run("undo");
        form.load();
        if ((form.parsed.params ?? []).some(p => p.id === "spin"))
            throw new Error("shaders: undo kept the parameter");
        form.close();
        form.destroy();
        page.run("remove_effect", {
                "effect": "bend"
            });
    }
}
