import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

SceneSeat {
    id: overlay

    readonly property var found: page.editor.selected.match(/^(.*\/layers\/\d+)(?:\/effects\/(\d+))?(\/mask(\/.*)?)?$/)
    readonly property var owner: found ? (page.view.gizmos ?? []).find(g => g.pointer === found[1]) ?? null : null
    readonly property int index: found && found[2] !== undefined ? Number(found[2]) : -1
    readonly property var shown: owner ? (page.view, JSON.parse(page.editor.rig("", JSON.stringify({
                        "mask_view": {
                            "layer": owner.id,
                            "index": index >= 0 ? index : null,
                            "time": page.editor.time
                        }
                    })) || "null")) : null
    readonly property bool masking: shown !== null
    readonly property string kind: shown?.kind ?? ""
    readonly property string texture: shown?.texture ?? ""
    readonly property var picture: texture ? JSON.parse(page.editor.rig("", JSON.stringify({
                    "mask_picture": {
                        "asset": texture,
                        "channel": shown.channel ?? null,
                        "invert": shown.invert,
                        "strength": shown.strength ?? 1,
                        "tint": hex(Kirigami.Theme.highlightColor)
                    }
                })) || "null") : null

    function hex(color) {
        return "#" + [color.r, color.g, color.b].map(v => Math.round(v * 255).toString(16).padStart(2, "0")).join("");
    }

    function put(fields) {
        page.run("effects", {
                "mask": Object.assign({
                        "layer": owner.id,
                        "index": index >= 0 ? index : null
                    }, fields)
            }, true);
    }

    function hold(grip, to) {
        page.run("effects", {
                "mask_grip": {
                    "layer": owner.id,
                    "index": index >= 0 ? index : null,
                    "hold": grip,
                    "to": to,
                    "at": page.editor.time
                }
            }, true);
    }

    function tip(grip) {
        return ({
                "size": App.tr("Drag to size the mask"),
                "feather": App.tr("Soft edge %1 px: drag away from the shape to soften its edge", shown?.feather ?? 0),
                "width": App.tr("Drag to widen or narrow the ring"),
                "radius": App.tr("Drag to round the corners"),
                "inner": App.tr("Drag to sharpen or blunt the points")
            })[grip];
    }

    layerId: owner?.id ?? ""
    visible: owner !== null

    SceneCaptureZone {
        anchors.fill: parent
        page: overlay.page
        ratio: overlay.ratio
    }

    Item {
        anchors.fill: parent
        visible: overlay.masking

        SceneArea {
            loops: overlay.shown ? [overlay.shown.box] : []
            ratio: overlay.ratio
            ink: Kirigami.Theme.disabledTextColor
            fill: 0
            dashed: true
        }

        SceneBoxPicture {
            visible: overlay.picture !== null && overlay.shown?.unit !== undefined
            unit: overlay.shown?.unit ?? [1, 0, 0, 1, 0, 0]
            ratio: overlay.ratio
            source: overlay.picture ? "file://" + overlay.picture.path : ""
        }

        SceneArea {
            loops: overlay.shown?.from_box ? [overlay.shown.from_box] : []
            ratio: overlay.ratio
            ink: Kirigami.Theme.neutralTextColor
            fill: 0.15
        }

        SceneArea {
            loops: overlay.shown ? (overlay.shown.invert && overlay.kind !== "" ? [overlay.shown.box].concat(overlay.shown.area) : overlay.shown.area) : []
            ratio: overlay.ratio
            fill: 0.25
        }

        SceneArea {
            loops: overlay.shown?.outer ?? []
            ratio: overlay.ratio
            fill: 0
            dashed: true
        }

        SceneArea {
            loops: overlay.shown?.inner ?? []
            ratio: overlay.ratio
            fill: 0
            dashed: true
        }

        SceneRepeater {
            id: holds

            rows: overlay.shown?.holds ?? []

            delegate: SceneKnob {
                required property int index
                readonly property var modelData: holds.rows[index]

                at: modelData.at
                ratio: overlay.ratio
                square: modelData.grip === "size"
                tint: modelData.grip === "feather" ? Kirigami.Theme.neutralTextColor : Kirigami.Theme.highlightColor
                tip: overlay.tip(modelData.grip)
                onMoved: to => overlay.hold(modelData.grip, to)
            }
        }
    }
}
