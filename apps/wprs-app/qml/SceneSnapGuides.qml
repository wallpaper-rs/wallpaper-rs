import QtQuick
import org.kde.kirigami as Kirigami

Item {
    id: guides

    required property var page
    required property real ratio
    readonly property var gizmos: page.view.gizmos ?? []
    readonly property var chosen: page.moving && page.magnets ? gizmos.find(g => g.pointer === page.editor.selected) ?? null : null
    readonly property var lines: chosen ? found(chosen) : []

    function bounds(points) {
        return [0, 1].map(i => {
                const values = points.map(p => p[i]);
                const lo = Math.min(...values), hi = Math.max(...values);
                return [lo, (lo + hi) / 2, hi];
            });
    }

    function found(me) {
        const size = page.view.canvas?.size ?? [1920, 1080];
        const targets = [[0, size[0] / 2, size[0]], [0, size[1] / 2, size[1]]];
        for (const other of gizmos) {
            if (other.id === me.id || other.pointer.startsWith(me.pointer + "/") || other.parent === me.id)
                continue;
            const b = bounds(other.corners ?? other.outline);
            targets[0].push(...b[0]);
            targets[1].push(...b[1]);
        }
        const own = bounds(me.corners ?? me.outline);
        const out = [];
        for (const axis of [0, 1])
            for (const at of targets[axis])
                if (own[axis].some(e => Math.abs(e - at) < 0.5) && !out.some(l => l.axis === axis && l.at === at))
                    out.push({
                            "axis": axis,
                            "at": at
                        });
        return out;
    }

    visible: lines.length > 0

    Repeater {
        model: guides.lines

        delegate: Rectangle {
            required property var modelData

            x: modelData.axis === 0 ? modelData.at * guides.ratio : 0
            y: modelData.axis === 1 ? modelData.at * guides.ratio : 0
            width: modelData.axis === 0 ? 1 : guides.width
            height: modelData.axis === 1 ? 1 : guides.height
            color: Kirigami.Theme.focusColor
        }
    }
}
