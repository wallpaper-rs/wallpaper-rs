import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: rings

    required property var gizmo
    readonly property var turns: gizmo.model.turns ?? null
    readonly property real ratio: gizmo.ratio

    anchors.fill: parent
    visible: gizmo.chosen && turns !== null

    SceneRepeater {
        id: listed

        rows: rings.visible ? rings.turns.rings : []

        delegate: SceneLightLines {
            required property int index
            readonly property var modelData: listed.rows[index]

            anchors.fill: parent
            ratio: rings.ratio
            tint: rings.gizmo.tints[index]
            segments: modelData
        }
    }

    SceneRepeater {
        id: grips

        rows: rings.visible ? rings.turns.grips : []

        delegate: SceneKnob {
            required property int index
            readonly property var modelData: grips.rows[index]

            at: modelData
            ratio: rings.ratio
            label: App.tr("Turn %1", ["X", "Y", "Z"][index])
            tint: rings.gizmo.tints[index]
            tip: rings.gizmo.free("rotation") ? App.tr("Drag along the ring to turn the model around this axis") : App.tr("The rotation follows a binding; drag with Shift to replace it with a value")
            opacity: rings.gizmo.free("rotation") ? 1 : 0.4
            onMoved: (to, done) => {
                if (done)
                    rings.gizmo.grab(["turn_x", "turn_y", "turn_z"][index], modelData, to, held);
            }
        }
    }

    SceneKnob {
        at: rings.turns?.deep ?? null
        ratio: rings.ratio
        square: true
        label: App.tr("Depth")
        tint: Kirigami.Theme.neutralTextColor
        tip: rings.gizmo.free("position") ? App.tr("Drag up to push the model away along the line of sight, down to bring it closer") : App.tr("The position follows a binding; drag with Shift to replace it with a value")
        opacity: rings.gizmo.free("position") ? 1 : 0.4
        onMoved: (to, done) => {
            if (done)
                rings.gizmo.grab("depth", rings.turns.deep, to, held);
        }
    }
}
