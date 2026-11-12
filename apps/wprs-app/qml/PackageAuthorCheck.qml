import QtQuick

QtObject {
    required property var page

    function widget(key) {
        return page.fields.find(field => field.key === key)?.widget;
    }

    function exercise() {
        page.editor.select("/manifest");
        if (widget("credits") !== "package:credits" || widget("strings") !== "package:strings")
            throw new Error("the credits and the strings have no panel of their own: " + widget("credits") + " " + widget("strings"));
        const licenses = page.fields.find(field => field.key === "license");
        if (licenses?.widget !== "package:license" || licenses.options.length < 8 || !licenses.options.every(o => o.summary !== ""))
            throw new Error("the license is not offered as a list with an explanation each: " + JSON.stringify(licenses));
        page.run("manifest", {
                "text": {
                    "key": "greeting",
                    "language": "en",
                    "text": "Hello"
                }
            });
        if (page.manifest.strings?.en?.greeting !== "Hello")
            throw new Error("a text by key was not written");
        const host = {
            "page": page,
            "field": {}
        };
        const strings = Qt.createComponent("PackageStrings.qml").createObject(page, {
                "host": host
            });
        if (strings.keys.length !== 1 || (strings.info.engine ?? []).length === 0)
            throw new Error("the table of texts did not list the key and the engine texts: " + JSON.stringify(strings.info));
        strings.destroy();
        const credits = Qt.createComponent("PackageCredits.qml").createObject(page, {
                "host": host
            });
        if (!Array.isArray(credits.assets))
            throw new Error("the credits did not list the files");
        credits.destroy();
        const view = Qt.createComponent("MarkupView.qml").createObject(page, {
                "source": "**bold** text"
            });
        if (view.implicitHeight <= 0)
            throw new Error("the markup was not shown");
        view.destroy();
        page.run("undo");
    }
}
