import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

SceneSeat {
    id: knobs

    required property var gizmo
    readonly property var model: gizmo.modelData
    readonly property var corners: model.outline
    readonly property var topMid: middle(corners[0], corners[1])
    readonly property var bottomMid: middle(corners[2], corners[3])
    readonly property var leftMid: middle(corners[3], corners[0])
    readonly property var mid: middle(corners[0], corners[2])
    readonly property real reach: Kirigami.Units.gridUnit / ratio
    property var grabbed: null

    function middle(a, b) {
        return [(a[0] + b[0]) / 2, (a[1] + b[1]) / 2];
    }

    function out(from, by) {
        const dx = from[0] - mid[0], dy = from[1] - mid[1], l = Math.hypot(dx, dy) || 1;
        return [from[0] + dx / l * by, from[1] + dy / l * by];
    }

    function turned(knob, to) {
        const dx = to[0] - knob.grabbed[0], dy = to[1] - knob.grabbed[1];
        const a = -model.angle * Math.PI / 180;
        return [dx * Math.cos(a) - dy * Math.sin(a), dx * Math.sin(a) + dy * Math.cos(a)];
    }

    function leaned(delta) {
        const span = Math.max(1, model.extent[0], model.extent[1]);
        const angle = v => Math.max(-89, Math.min(89, Math.round(v * 10) / 10));
        return [angle(grabbed[0] - delta[1] * 180 / span), angle(grabbed[1] + delta[0] * 180 / span)];
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

            at: knobs.middle(knobs.corners[index], knobs.corners[(index + 1) % 4])
            ratio: knobs.ratio
            square: true
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
        at: knobs.model.pivot
        ratio: knobs.ratio
        tint: Kirigami.Theme.neutralTextColor
        width: Kirigami.Units.gridUnit * 0.65
        tip: App.tr("Pivot: drag to move the origin of the layer; the layer stays where it is")
        onMoved: to => knobs.grip("origin", to, false)
    }

    SceneKnob {
        id: tilting

        at: knobs.out(knobs.leftMid, knobs.reach * 1.2)
        ratio: knobs.ratio
        tint: Kirigami.Theme.backgroundColor
        tip: App.tr("Tilt %1°, %2°: drag up or down to tilt around the width, sideways around the height", knobs.model.tilt[0], knobs.model.tilt[1])
        onGrabbedChanged: knobs.grabbed = grabbed ? knobs.model.tilt : null
        onMoved: to => knobs.grabbed && knobs.gizmo.commit("tilt", knobs.leaned(knobs.turned(tilting, to)))
    }

    SceneKnob {
        id: lifting

        at: knobs.out(knobs.bottomMid, knobs.reach * 1.2)
        ratio: knobs.ratio
        square: true
        rotation: 45
        tint: Kirigami.Theme.backgroundColor
        tip: App.tr("Depth %1 px: drag up to bring the layer towards the viewer, down to push it back", knobs.model.z)
        onGrabbedChanged: knobs.grabbed = grabbed ? [knobs.model.z] : null
        onMoved: to => knobs.grabbed && knobs.gizmo.commit("z", Math.max(-100000, Math.min(100000, knobs.grabbed[0] - (to[1] - lifting.grabbed[1]) * 2)))
    }
}
