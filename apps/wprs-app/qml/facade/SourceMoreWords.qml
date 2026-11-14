pragma Singleton
import QtQuick

QtObject {
    readonly property var parents: ({
            "resolutions": "size",
            "ratios": "size",
            "colors": "size"
        })
    readonly property var keys: ({
            "size": ["atleast", "resolutions", "ratios", "colors"],
            "other": ["query", "top_range", "order", "tag", "uploader", "page", "pages", "max_items"]
        })
    readonly property var flagged: ["categories", "purity", "resolutions", "ratios", "colors"]

    function parent(level) {
        return parents[level] ?? "";
    }

    function level(key) {
        const names = {
            "categories": App.tr("Categories"),
            "purity": App.tr("Content"),
            "size": App.tr("Size and colors"),
            "other": App.tr("Other search options"),
            "collections": App.tr("Collections")
        };
        return names[key] ?? field(key);
    }

    function field(key) {
        return key === "atleast" ? App.tr("Minimum resolution") : key === "query" ? App.tr("What to look for") : Labels.field(key);
    }

    function choice(key, value) {
        const purity = {
            "sfw": App.tr("For everyone"),
            "sketchy": App.tr("A bit daring"),
            "nsfw": App.tr("For adults")
        };
        if (key === "purity")
            return purity[value] ?? value;
        if (key === "categories")
            return Labels.field("categories." + value);
        if (key === "colors")
            return "#" + value;
        if (key === "ratios" && value.includes("x"))
            return value.replace("x", ":");
        if (key === "resolutions" || key === "atleast")
            return value.replace("x", "×");
        return Labels.value(value);
    }
}
