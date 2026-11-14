import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: check

    required property var host
    readonly property var frame: host.shell.titleFrame

    function must(ok, text) {
        if (!ok)
            throw new Error("now block taps on " + (host.pageStack.currentItem?.title ?? "?") + ": " + text);
    }

    function start() {
        if (frame.owned)
            return check.destroy();
        must(host.kind === "title" && frame.ahora.visible && !frame.ahora.none, "the block is not on the page");
        probe();
        check.destroy();
    }

    function buttons(item, found) {
        if (!item.visible || item.width <= 0)
            return found;
        if ("first" in item && "off" in item && "active" in item && "hovered" in item)
            return found.concat([item]);
        let all = found;
        for (const child of item.children)
            all = buttons(child, all);
        return all;
    }

    function topmost(point) {
        let item = frame.parent;
        for (;;) {
            const order = Array.from(item.children).map((c, i) => [c, i]).sort((a, b) => a[0].z - b[0].z || a[1] - b[1]).map(p => p[0]).reverse();
            const child = order.find(c => c.visible && c.contains(c.mapFromItem(null, point.x, point.y)));
            if (!child)
                return item;
            item = child;
        }
    }

    function inside(item, owner) {
        for (let up = item; up; up = up.parent)
            if (up === owner)
                return true;
        return false;
    }

    function chain(item) {
        let names = [];
        for (let up = item; up && names.length < 8; up = up.parent)
            names.push(String(up).replace(/\(0x[0-9a-f]+,? ?/, "(") + "@" + Math.round(up.x) + "," + Math.round(up.y) + " " + Math.round(up.width) + "x" + Math.round(up.height) + " z" + up.z);
        return names.join(" < ");
    }

    function probe() {
        const block = frame.ahora;
        const line = block.rotating;
        line.playlist = {
            "uid": "probe",
            "name": "probe"
        };
        try {
            line.forceLayout();
            block.forceLayout();
            block.parent.forceLayout();
            const all = buttons(block, []);
            must(all.length >= (block.compact ? 3 : 7), "the block shows only " + all.length + " buttons");
            for (const button of all) {
                const centre = button.mapToItem(null, button.width / 2, button.height / 2);
                const top = topmost(centre);
                must(inside(top, button), "«" + button.text + "» lies under " + chain(top) + (inside(top, frame.seat) ? " of the page" : "") + "; the button: " + chain(button));
            }
        } finally {
            line.playlist = Qt.binding(() => block.none ? null : block.playlist);
        }
    }
}
