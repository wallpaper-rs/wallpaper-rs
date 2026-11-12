import QtQuick

Item {
    id: seat

    required property var page
    required property real ratio
    property string layerId
    readonly property var gizmo: (page.view.gizmos ?? []).find(g => g.id === layerId) ?? null

    function at(u) {
        const m = gizmo?.unit ?? [1, 0, 0, 1, 0, 0];
        return [m[0] * u[0] + m[2] * u[1] + m[4], m[1] * u[0] + m[3] * u[1] + m[5]];
    }

    function grip(kind, to, snap) {
        const args = {
            "layer": layerId,
            "grip": kind,
            "to": to,
            "at": page.editor.time,
            "replace": page.overwrite
        };
        if (snap && page.magnets)
            args.snap = 8 / ratio;
        page.run("layer", {
                "grip": args
            }, true);
    }

    visible: gizmo !== null
}
