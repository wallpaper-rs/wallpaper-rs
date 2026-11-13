import QtQuick
import QtQuick.Shapes
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: depth

    required property var knobs
    readonly property var model: knobs.model.depth ?? null
    readonly property real ratio: knobs.ratio

    function line(a, b) {
        return [a, b];
    }

    visible: model !== null

    SceneOutline {
        anchors.fill: parent
        corners: depth.model?.ring ?? []
        ratio: depth.ratio
        dashed: true
    }

    Repeater {
        model: depth.model ? [depth.line(depth.model.pivot, depth.model.height), depth.line(depth.model.pivot, depth.model.width), depth.line(depth.model.foot, depth.model.lift)] : []

        delegate: Shape {
            required property var modelData

            anchors.fill: parent

            ShapePath {
                strokeColor: Look.ambar
                strokeWidth: 1
                fillColor: "transparent"
                startX: modelData[0][0] * depth.ratio
                startY: modelData[0][1] * depth.ratio

                PathLine {
                    x: modelData[1][0] * depth.ratio
                    y: modelData[1][1] * depth.ratio
                }
            }
        }
    }

    SceneKnob {
        at: depth.model?.height ?? null
        ratio: depth.ratio
        tint: Kirigami.Theme.backgroundColor
        tip: App.tr("Tilt around the width %1°: drag this end of the upright axis toward or away from the pivot", depth.knobs.model.tilt[0])
        onMoved: to => depth.knobs.grip({
                "tilt": {
                    "axis": 0
                }
            }, to, false)
    }

    SceneKnob {
        at: depth.model?.width ?? null
        ratio: depth.ratio
        tint: Kirigami.Theme.backgroundColor
        tip: App.tr("Tilt around the height %1°: drag this end of the level axis toward or away from the pivot", depth.knobs.model.tilt[1])
        onMoved: to => depth.knobs.grip({
                "tilt": {
                    "axis": 1
                }
            }, to, false)
    }

    SceneKnob {
        at: depth.model?.lift ?? null
        ratio: depth.ratio
        square: true
        rotation: 45
        tint: Kirigami.Theme.backgroundColor
        tip: App.tr("Depth %1 px: drag along the line out of the layer to bring it toward the viewer or send it back", depth.knobs.model.z)
        onMoved: to => depth.knobs.grip("lift", to, false)
    }
}
