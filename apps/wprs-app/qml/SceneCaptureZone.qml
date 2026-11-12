import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: zone

    required property var page
    required property real ratio
    readonly property var found: page.editor.selected.match(/^(.*\/layers\/\d+)(\/.*)?$/)
    readonly property var owner: found ? (page.view.gizmos ?? []).find(g => g.pointer === found[1] && g.kind === "adjustment") ?? null : null
    readonly property var shown: owner ? (page.view, JSON.parse(page.editor.rig("", JSON.stringify({
                        "capture_view": {
                            "layer": owner.id,
                            "time": page.editor.time
                        }
                    })) || "null")) : null
    readonly property color alarm: Kirigami.Theme.negativeTextColor

    function size(corner, to) {
        page.run("layer", {
                "grip": {
                    "layer": owner.id,
                    "grip": {
                        "size": {
                            "corner": corner
                        }
                    },
                    "to": to,
                    "at": page.editor.time,
                    "replace": page.overwrite
                }
            }, true);
    }

    visible: shown !== null

    SceneArea {
        loops: zone.shown ? [zone.shown.limit] : []
        ratio: zone.ratio
        ink: zone.shown?.over ? zone.alarm : Kirigami.Theme.disabledTextColor
        fill: 0
        dashed: true
    }

    SceneArea {
        loops: zone.shown ? [zone.shown.zone] : []
        ratio: zone.ratio
        ink: zone.shown?.over ? zone.alarm : Kirigami.Theme.highlightColor
        fill: 0.15
    }

    SceneArea {
        visible: zone.shown?.redraw ?? false
        loops: zone.shown?.drawn ? [zone.shown.drawn] : []
        ratio: zone.ratio
        ink: Kirigami.Theme.positiveTextColor
        fill: 0
        dashed: true
    }

    QQC2.Label {
        x: (zone.shown?.zone[0][0] ?? 0) * zone.ratio
        y: (zone.shown?.zone[0][1] ?? 0) * zone.ratio - height
        text: App.tr("Captures %1 × %2 px; the most is %3 × %4", Math.round(zone.shown?.size[0] ?? 0), Math.round(zone.shown?.size[1] ?? 0), Math.round(zone.shown?.max[0] ?? 0), Math.round(zone.shown?.max[1] ?? 0))
        style: Text.Outline
        styleColor: Kirigami.Theme.backgroundColor
    }

    SceneRepeater {
        id: holds

        rows: zone.shown?.handles ?? []

        delegate: SceneKnob {
            required property int index
            readonly property var modelData: holds.rows[index]

            at: modelData.at
            ratio: zone.ratio
            square: true
            tip: App.tr("Size the captured part; the layer keeps its scale")
            onMoved: to => zone.size(modelData.corner, to)
        }
    }
}
