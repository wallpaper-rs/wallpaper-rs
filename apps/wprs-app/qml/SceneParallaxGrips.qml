import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: grips

    required property var page
    required property real ratio
    readonly property var size: page.view.canvas?.size ?? [1920, 1080]
    readonly property var glide: page.view.canvas?.parallax ?? null
    readonly property bool trying: (page.view.probe ?? null) !== null
    readonly property bool onCanvas: page.editor.selected.startsWith("/scene/canvas")
    readonly property var offset: Array.isArray(glide?.offset) ? glide.offset : (glide !== null && glide.offset === undefined ? [0, 0] : null)
    readonly property var chosen: (page.view.gizmos ?? []).find(g => g.pointer === page.editor.selected && g.pointer.startsWith("/scene/layers/")) ?? null
    readonly property real unit: size[0] / 4
    readonly property var stored: {
        void page.view;
        return chosen ? JSON.parse(page.editor.value(chosen.pointer + "/parallax") || "0") : 0;
    }
    readonly property bool bound: stored !== null && typeof stored === "object" && !Array.isArray(stored)
    readonly property var pair: Array.isArray(stored) ? stored : [bound ? 0 : stored || 0, bound ? 0 : stored || 0]
    readonly property var middle: chosen ? [0, 1].map(c => chosen.outline.reduce((sum, p) => sum + p[c], 0) / 4) : null

    function clamp(v, low, high) {
        return Math.max(low, Math.min(high, v));
    }

    function round(v, places) {
        const k = Math.pow(10, places);
        return Math.round(v * k) / k;
    }

    function slide(to, free) {
        const dx = round(clamp((to[0] - middle[0]) / unit, -10, 10), 3);
        const dy = round(clamp((to[1] - middle[1]) / unit, -10, 10), 3);
        const linked = !free && pair[0] === pair[1];
        const both = linked ? round(clamp(((to[0] - middle[0]) + (to[1] - middle[1])) / 2 / unit, -10, 10), 3) : 0;
        page.run("layer", {
            "depth": {
                "layer": chosen.id,
                "depth": linked ? [both, both] : [dx, dy]
            }
        }, true);
    }

    function shift(to) {
        page.run("canvas", {
            "parallax": {
                "offset": [round(clamp((to[0] - size[0] / 2) / (size[0] / 2), -1, 1), 3), round(clamp((to[1] - size[1] / 2) / (size[1] / 2), -1, 1), 3)]
            }
        }, true);
    }

    SceneOutline {
        anchors.fill: parent
        visible: grips.onCanvas && grips.offset !== null
        corners: grips.offset ? [[grips.size[0] / 2, grips.size[1] / 2], [grips.size[0] * (1 + grips.offset[0]) / 2, grips.size[1] * (1 + grips.offset[1]) / 2]] : []
        ratio: grips.ratio
        dashed: true
    }

    SceneKnob {
        visible: grips.onCanvas && grips.offset !== null
        at: grips.offset ? [grips.size[0] * (1 + grips.offset[0]) / 2, grips.size[1] * (1 + grips.offset[1]) / 2] : null
        ratio: grips.ratio
        tint: Kirigami.Theme.positiveTextColor
        label: App.tr("Parallax center")
        tip: App.tr("Where the parallax rests: drag it away from the middle of the canvas to shift the whole picture")
        onMoved: to => grips.shift(to)
    }

    SceneOutline {
        anchors.fill: parent
        visible: grips.trying && grips.chosen !== null && !grips.bound
        corners: grips.middle ? [grips.middle, [grips.middle[0] + grips.pair[0] * grips.unit, grips.middle[1] + grips.pair[1] * grips.unit]] : []
        ratio: grips.ratio
        dashed: true
    }

    SceneKnob {
        id: depth

        visible: grips.trying && grips.chosen !== null && !grips.bound
        at: grips.middle ? [grips.middle[0] + grips.pair[0] * grips.unit, grips.middle[1] + grips.pair[1] * grips.unit] : null
        ratio: grips.ratio
        tint: Kirigami.Theme.positiveTextColor
        label: App.tr("Depth %1, %2", grips.pair[0], grips.pair[1])
        tip: App.tr("Parallax depth of the layer: drag it away from the layer, a quarter of the canvas width is 1; hold Shift to set the depth across and the depth up and down apart")
        onMoved: to => grips.slide(to, (depth.held & Qt.ShiftModifier) !== 0)
    }
}
