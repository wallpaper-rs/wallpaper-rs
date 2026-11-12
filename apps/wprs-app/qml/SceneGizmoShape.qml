import QtQuick

QtObject {
    required property Item gizmo

    function contains(point: point): bool {
        const corners = gizmo.modelData.outline ?? [];
        if (corners.length < 3)
            return point.x >= 0 && point.y >= 0 && point.x <= gizmo.width && point.y <= gizmo.height;
        const at = gizmo.mapToItem(gizmo.parent, point.x, point.y);
        const x = at.x / gizmo.ratio;
        const y = at.y / gizmo.ratio;
        let hit = false;
        for (let i = 0, j = corners.length - 1; i < corners.length; j = i++) {
            const a = corners[i];
            const b = corners[j];
            if ((a[1] > y) !== (b[1] > y) && x < (b[0] - a[0]) * (y - a[1]) / (b[1] - a[1]) + a[0])
                hit = !hit;
        }
        return hit;
    }
}
