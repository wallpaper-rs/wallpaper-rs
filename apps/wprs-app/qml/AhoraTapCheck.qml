import QtQuick
import io.github.wallpaper_rs.app

Timer {
    id: check

    required property var host
    readonly property var frame: host.shell.titleFrame
    readonly property var pages: [0, 4, 1, 3, 7]
    property int at: -1
    property int tries: 0
    property int tested: 0

    function must(ok, text) {
        if (!ok)
            throw new Error("now block taps: " + text);
    }

    function start() {
        host.waiting++;
        host.held = host.held.concat([check]);
        restart();
    }

    function next() {
        tries = 0;
        if (++at < pages.length) {
            host.showPage(pages[at]);
            return restart();
        }
        host.waiting--;
        host.held = host.held.filter(c => c !== check);
        if (tested === 0)
            throw new Error("now block taps: no page shows the block");
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
            must(all.length >= 7, "page " + pages[at] + " shows only " + all.length + " buttons");
            for (const button of all) {
                const centre = button.mapToItem(null, button.width / 2, button.height / 2);
                const top = topmost(centre);
                must(inside(top, button), "on page " + pages[at] + " «" + button.text + "» lies under " + chain(top) + (inside(top, frame.seat) ? " of the page" : "") + "; the button: " + chain(button));
            }
            tested++;
        } finally {
            line.playlist = Qt.binding(() => block.none ? null : block.playlist);
        }
    }

    interval: Look.paso * 6
    onTriggered: {
        if (at < 0)
            return host.waiting > 1 && ++tries < 400 ? restart() : next();
        const block = frame.ahora;
        if ((host.kind !== "title" || !block.visible || block.none || block.width <= 0) && ++tries < 30)
            return restart();
        if (host.kind === "title" && block.visible && !block.none)
            probe();
        next();
    }
}
