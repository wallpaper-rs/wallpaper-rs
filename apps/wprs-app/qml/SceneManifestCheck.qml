import QtQuick

QtObject {
    required property var page

    function exercise() {
        const field = key => page.fields.find(item => item.key === key) ?? {};
        page.editor.select("/manifest");
        if (field("title").widget !== "texts" || field("description").widget !== "markup" || field("authors").widget !== "lines")
            throw new Error("the manifest has no text editors");
        page.run("translate", {
            "pointer": "/manifest/title",
            "language": "es",
            "text": "Prueba"
        });
        if (!field("title").options.some(row => row.language === "es" && row.text === "Prueba"))
            throw new Error("the title has no Spanish text");
        for (const language of ["es", "de"])
            page.run("translate", {
                "pointer": "/manifest/title",
                "language": language,
                "add": true
            });
        const rows = field("title").options;
        if (!rows.some(row => row.language === "es" && row.text === "Prueba") || !rows.some(row => row.language === "de" && row.text === rows[0].text))
            throw new Error("adding a language changed a text");
        if (field("default_locale").widget !== "ref:language" || !field("default_locale").required)
            throw new Error("the default language can be reset");
        page.change("/manifest/content", {
            "rating": "mature",
            "descriptors": ["violence"]
        });
        page.change("/manifest/photosensitive", true);
        page.change("/manifest/inputs", ["media"]);
        page.run("set", {
            "pointer": "/manifest/origin",
            "value": {
                "kind": "derived",
                "derived_from": [
                    {
                        "author": "Someone"
                    }
                ]
            }
        });
        page.run("set", {
            "pointer": "/manifest/links",
            "value": {
                "homepage": "https://example.org"
            }
        });
        if (!field("inputs").value.includes("pointer") || !field("inputs").fixed.includes("pointer"))
            throw new Error("an input the scene uses was dropped");
        page.editor.select("/manifest/requires");
        if (field("features").widget !== "computed" || !field("features").value.includes("inputs.media") || !field("features").options.includes("locale_files"))
            throw new Error("the features do not follow the inputs");
        page.run("set", {
            "pointer": "/manifest/schema",
            "value": "wallpaper-rs.wpak/1"
        });
        page.run("upgrade");
        page.editor.select("/manifest/content");
        if (!field("descriptors").options.includes("gore") || page.view.rating !== "mature" || page.view.v2 !== true)
            throw new Error("the rating did not stay");
    }
}
