import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

SceneSeat {
    id: knobs

    required property var gizmo
    readonly property var model: gizmo.modelData
    readonly property var corners: model.handles?.corners ?? model.outline
    readonly property var edges: model.handles?.edges ?? [0, 1, 2, 3].map(i => middle(model.outline[i], model.outline[(i + 1) % 4]))
    readonly property var topMid: edges[0]
    readonly property var mid: middle(corners[0], corners[2])
    readonly property real reach: Kirigami.Units.gridUnit / ratio
    readonly property bool captures: model.kind === "adjustment" && Array.isArray(JSON.parse(page.editor.value(model.pointer + "/size") || "null"))

    function middle(a, b) {
        return [(a[0] + b[0]) / 2, (a[1] + b[1]) / 2];
    }

    function out(from, by) {
        const dx = from[0] - mid[0], dy = from[1] - mid[1], l = Math.hypot(dx, dy) || 1;
        return [from[0] + dx / l * by, from[1] + dy / l * by];
    }

    page: gizmo.page
    ratio: gizmo.ratio
    layerId: model.id
    visible: gizmo.movable

    Repeater {
        model: [[0, 0], [1, 0], [1, 1], [0, 1]]

        delegate: SceneKnob {
            required property var modelData
            required property int index

            at: knobs.corners[index]
            ratio: knobs.ratio
            square: true
            visible: Array.isArray(at) && !knobs.captures
            tip: App.tr("Scale from the pivot; Shift keeps the proportions")
            onMoved: to => knobs.grip({
                    "scale": {
                        "corner": modelData,
                        "even": (held & Qt.ShiftModifier) !== 0
                    }
                }, to, false)
        }
    }

    Repeater {
        model: [[0.5, 0], [1, 0.5], [0.5, 1], [0, 0.5]]

        delegate: SceneKnob {
            required property var modelData
            required property int index

            at: knobs.edges[index]
            ratio: knobs.ratio
            square: true
            visible: Array.isArray(at) && !knobs.captures
            width: Kirigami.Units.gridUnit * 0.55
            tip: App.tr("Stretch one side; Ctrl skews the layer along this edge")
            onMoved: to => knobs.grip((held & Qt.ControlModifier) ? {
                    "skew": {
                        "edge": modelData
                    }
                } : {
                    "scale": {
                        "corner": modelData
                    }
                }, to, false)
        }
    }

    SceneKnob {
        at: knobs.out(knobs.topMid, knobs.reach * 1.5)
        ratio: knobs.ratio
        tip: App.tr("Turn around the pivot")
        onMoved: to => knobs.grip("turn", to, false)
    }

    SceneKnob {
        at: knobs.model.handles?.pivot ?? knobs.model.pivot
        ratio: knobs.ratio
        tint: Kirigami.Theme.neutralTextColor
        width: Kirigami.Units.gridUnit * 0.65
        tip: App.tr("Pivot: drag to move the origin of the layer; the layer stays where it is")
        onMoved: to => knobs.grip("origin", to, false)
    }

    SceneGizmoDepth {
        anchors.fill: parent
        knobs: knobs
    }
}
