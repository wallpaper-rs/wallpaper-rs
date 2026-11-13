import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

SceneSeat {
    id: overlay

    readonly property var found: page.editor.selected.match(/^(.*\/layers\/\d+)(\/.*)?$/)
    readonly property var owner: found ? (page.view.gizmos ?? []).find(g => g.pointer === found[1] && g.kind === "shape") ?? null : null
    readonly property var shown: owner ? (page.view, JSON.parse(page.editor.rig("", JSON.stringify({
                        "shape_handles": {
                            "layer": owner.id,
                            "time": page.editor.time
                        }
                    })) || "null")) : null
    readonly property var marks: shown?.handles ?? []
    readonly property var gradient: shown?.gradient ?? null
    readonly property var tips: ({
            "width": App.tr("Set the width"),
            "height": App.tr("Set the height"),
            "size": App.tr("Resize; hold Shift to keep the proportions"),
            "radius": App.tr("Round the corners"),
            "band": App.tr("Set the thickness of the ring"),
            "sides": App.tr("Drag around the shape to set the number of sides"),
            "points": App.tr("Drag around the shape to set the number of points"),
            "inner": App.tr("Set the depth of the points"),
            "start": App.tr("Move the start of the line"),
            "end": App.tr("Move the end of the line"),
            "stroke": App.tr("Set the width of the outline"),
            "gradient_from": App.tr("Start of the gradient, or its center when it is radial"),
            "gradient_to": App.tr("End of the gradient, or the edge when it is radial")
        })

    function put(name, to, even) {
        page.run("layer", {
                "figure": {
                    "shape": {
                        "layer": owner.id,
                        "handle": name,
                        "to": to,
                        "even": even,
                        "at": page.editor.time,
                        "replace": page.overwrite
                    }
                }
            }, true);
    }

    layerId: owner?.id ?? ""
    visible: owner !== null && marks.length > 0

    SceneOutline {
        visible: overlay.gradient !== null
        corners: overlay.gradient ? [overlay.gradient.from, overlay.gradient.to] : []
        ratio: overlay.ratio
        chosen: true
        dashed: true
    }

    SceneRepeater {
        id: knobs

        rows: overlay.marks

        delegate: SceneKnob {
            required property int index
            readonly property var modelData: knobs.rows[index]
            readonly property bool gradient: modelData.name.startsWith("gradient_")

            at: modelData.at
            ratio: overlay.ratio
            square: modelData.kind === "corner" || modelData.kind === "edge"
            tint: gradient ? Look.brote : modelData.kind === "round" || modelData.kind === "count" || modelData.kind === "depth" ? Look.ambar : Kirigami.Theme.highlightColor
            tip: overlay.tips[modelData.name] ?? ""
            onMoved: to => overlay.put(modelData.name, to, (held & Qt.ShiftModifier) !== 0)
        }
    }
}
