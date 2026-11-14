pragma Singleton

import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    function rows(item, out) {
        const found = out ?? [];
        if (!item || !item.visible)
            return found;
        if (item.kind !== undefined && item.explain !== undefined && item.activeFocusOnTab)
            found.push(item);
        for (const child of item.children)
            rows(child, found);
        return found;
    }

    function all(item, out) {
        const found = out ?? [];
        if (!item)
            return found;
        if (item.kind !== undefined && item.explain !== undefined && item.activeFocusOnTab)
            found.push(item);
        for (const child of item.children)
            all(child, found);
        return found;
    }

    function inside(item, owner) {
        for (let at = item; at; at = at.parent) {
            if (at === owner)
                return true;
        }
        return false;
    }

    function reveal(scroll, item) {
        const top = item.mapToItem(scroll.contentItem, 0, 0).y;
        const bottom = top + item.height;
        if (top < scroll.contentY)
            scroll.contentY = Math.max(0, top - Look.e4);
        else if (bottom > scroll.contentY + scroll.height)
            scroll.contentY = Math.min(scroll.contentHeight - scroll.height, bottom - scroll.height + Look.e4);
    }

    function plain(text) {
        return String(text).toLowerCase().normalize("NFD").replace(/[̀-ͯ]/g, "");
    }

    function matches(text, query) {
        const wanted = plain(query).trim();
        return wanted !== "" && plain(text).includes(wanted);
    }
}
