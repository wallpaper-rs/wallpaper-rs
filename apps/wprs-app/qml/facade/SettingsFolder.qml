pragma Singleton

import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    function name(source) {
        const parts = String(source.path ?? "").split("/").filter(part => part !== "");
        return parts.length > 0 ? parts[parts.length - 1] : String(source.provider ?? source.path ?? "");
    }

    function note(source) {
        const count = source.items === 1 ? App.tr("1 wallpaper") : App.tr("%1 wallpapers", source.items ?? 0);
        return [count, source.recursive ? App.tr("with subfolders") : App.tr("without subfolders"), source.provider ?? ""].filter(part => part !== "").join(" · ");
    }

    function keys(text) {
        const names = {
            "LOGO": "Meta",
            "CTRL": "Ctrl",
            "ALT": "Alt",
            "SHIFT": "Shift"
        };
        return String(text ?? "").split("+").filter(part => part !== "").map(part => names[part.toUpperCase()] ?? part);
    }

    function joined(keys) {
        const names = {
            "Meta": "LOGO",
            "Ctrl": "CTRL",
            "Alt": "ALT",
            "Shift": "SHIFT"
        };
        return keys.map(key => names[key] ?? String(key).toUpperCase()).join("+");
    }
}
