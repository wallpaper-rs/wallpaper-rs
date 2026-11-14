import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    function settle(item) {
        if (typeof item.ensurePolished === "function")
            item.ensurePolished();
        for (const kid of item.children)
            settle(kid);
    }

    function laid(root) {
        for (let pass = 0; pass < 4; pass++)
            settle(root);
        return typeof root.ensurePolished === "function";
    }

    function clipped(item) {
        return "elide" in item && "wrapMode" in item && "implicitWidth" in item && item.elide === Text.ElideNone && item.wrapMode === Text.NoWrap && String(item.text ?? "") !== "" && item.implicitWidth > item.width + 1;
    }

    function cuts(root) {
        const found = [];
        const walk = item => {
            if (!item.visible || item.opacity <= 0)
                return;
            const right = item.mapToItem(root, item.width, 0).x;
            const content = item.parent !== null && "contentX" in item.parent && item.parent.contentItem === item;
            if (!content && item.width > 0 && right > root.width + 1)
                found.push(String(item).split("(")[0] + " «" + String(item.text ?? item.objectName ?? "") + "» ends at " + Math.round(right));
            if (clipped(item))
                return found.push(String(item).split("(")[0] + " «" + item.text + "» needs " + Math.round(item.implicitWidth) + " in " + Math.round(item.width));
            for (const kid of item.children)
                walk(kid);
        };
        if (laid(root))
            for (const kid of root.children)
                walk(kid);
        return found;
    }

    function words(root) {
        const found = [];
        const walk = item => {
            if (!item.visible)
                return;
            if ("text" in item && typeof item.text === "string" && item.text !== "")
                found.push(item.text);
            for (const kid of item.children)
                walk(kid);
        };
        walk(root);
        return found;
    }

    function fits(what, root, wanted) {
        const cut = cuts(root);
        if (cut.length > 0)
            throw new Error(what + ": cut at " + Math.round(root.width) + " px: " + cut.slice(-12).join("; "));
        const shown = words(root);
        const missing = wanted.filter(word => !shown.includes(word));
        if (missing.length > 0)
            throw new Error(what + ": does not show " + JSON.stringify(missing) + " in " + JSON.stringify(shown.slice(0, 30)));
    }
}
