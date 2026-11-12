import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

SceneSeat {
    id: overlay

    readonly property var found: page.editor.selected.match(/^(.*\/layers\/\d+)\/effects\/(\d+)(\/params(\/.*)?)?$/)
    readonly property var owner: found ? (page.view.gizmos ?? []).find(g => g.pointer === found[1]) ?? null : null
    readonly property var shown: owner ? (page.view, JSON.parse(page.editor.rig("", JSON.stringify({
                        "effect_handles": {
                            "layer": owner.id,
                            "index": Number(found[2]),
                            "time": page.editor.time
                        }
                    })) || "null")) : null
    readonly property var marks: shown?.handles ?? []
    readonly property var corners: marks.filter(h => h.kind === "corner").map(h => h.at)
    readonly property var links: marks.filter(h => h.partner && h.param < h.partner).map(h => ({
                    "param": h.param,
                    "ends": [h.at, marks.find(o => o.param === h.partner)?.at ?? h.at]
                }))
    readonly property var aimed: marks.filter(h => h.anchor)

    function sum(a, b, sign) {
        return [a[0] + sign * b[0], a[1] + sign * b[1]];
    }

    function put(param, to, both) {
        page.run("effects", {
                "grip": {
                    "layer": owner.id,
                    "index": Number(found[2]),
                    "param": param,
                    "to": to,
                    "both": both,
                    "at": page.editor.time,
                    "replace": page.overwrite
                }
            }, true);
    }

    layerId: owner?.id ?? ""
    visible: owner !== null && marks.length > 0

    SceneOutline {
        corners: overlay.corners
        ratio: overlay.ratio
        chosen: true
        visible: overlay.corners.length > 0
    }

    SceneRepeater {
        id: joins

        rows: overlay.links

        delegate: SceneOutline {
            required property int index
            readonly property var modelData: joins.rows[index]

            corners: modelData.ends
            ratio: overlay.ratio
            chosen: true
        }
    }

    SceneRepeater {
        id: arms

        rows: overlay.aimed

        delegate: Item {
            required property int index
            readonly property var modelData: arms.rows[index]
            readonly property real reach: Math.hypot(modelData.at[0] - modelData.anchor[0], modelData.at[1] - modelData.anchor[1]) * overlay.ratio

            SceneOutline {
                corners: [parent.modelData.anchor, parent.modelData.at]
                ratio: overlay.ratio
                chosen: true
                dashed: true
            }

            Rectangle {
                visible: parent.modelData.kind === "angle"
                x: parent.modelData.anchor[0] * overlay.ratio - width / 2
                y: parent.modelData.anchor[1] * overlay.ratio - height / 2
                width: parent.reach * 2
                height: width
                radius: width / 2
                color: "transparent"
                border.color: Kirigami.Theme.disabledTextColor
            }
        }
    }

    SceneRepeater {
        id: points

        rows: overlay.marks

        delegate: SceneKnob {
            required property int index
            readonly property var modelData: points.rows[index]
            readonly property bool turns: modelData.kind === "angle"

            at: modelData.at
            ratio: overlay.ratio
            square: modelData.kind === "corner"
            tint: modelData.kind === "vector" || turns ? Kirigami.Theme.neutralTextColor : Kirigami.Theme.highlightColor
            tip: turns ? App.tr("Turn the angle around its point") : modelData.kind === "vector" ? App.tr("Aim the direction") : modelData.partner ? App.tr("Place this end; hold Shift to move the whole line") : App.tr("Place this point")
            onMoved: to => overlay.put(modelData.param, to, (held & Qt.ShiftModifier) !== 0)
        }
    }

    SceneRepeater {
        id: middles

        rows: overlay.links

        delegate: SceneKnob {
            required property int index
            readonly property var modelData: middles.rows[index]
            property var origin: modelData.ends[0]

            at: [(modelData.ends[0][0] + modelData.ends[1][0]) / 2, (modelData.ends[0][1] + modelData.ends[1][1]) / 2]
            ratio: overlay.ratio
            square: true
            tint: Kirigami.Theme.neutralTextColor
            tip: App.tr("Move the whole line")
            onGrabbedChanged: {
                if (grabbed)
                    origin = modelData.ends[0];
            }
            onMoved: to => overlay.put(modelData.param, overlay.sum(origin, overlay.sum(to, grabbed ?? to, -1), 1), true)
        }
    }
}
