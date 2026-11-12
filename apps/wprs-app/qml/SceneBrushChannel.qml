import QtQuick
import org.kde.kirigami as Kirigami

Item {
    id: channel

    required property var page
    required property real ratio
    readonly property var brush: page.brush
    readonly property var found: (brush?.pointer ?? "").match(/^(.*\/layers\/\d+)\/effects\/\d+\/params\/.+$/)
    readonly property var owner: found ? (page.view.gizmos ?? []).find(g => g.pointer === found[1]) ?? null : null
    readonly property var asset: owner ? (page.view, JSON.parse(page.editor.value(brush.pointer) || "null")) : null
    readonly property var shown: typeof asset === "string" ? JSON.parse(page.editor.rig("", JSON.stringify({
                "mask_picture": {
                    "asset": asset,
                    "channel": ["r", "g", "b", "a"][brush.channel] ?? null,
                    "tint": hex(Kirigami.Theme.highlightColor)
                }
            })) || "null") : null

    function hex(color) {
        return "#" + [color.r, color.g, color.b].map(v => Math.round(v * 255).toString(16).padStart(2, "0")).join("");
    }

    visible: shown !== null

    SceneBoxPicture {
        unit: channel.owner?.unit ?? [1, 0, 0, 1, 0, 0]
        ratio: channel.ratio
        source: channel.shown ? "file://" + channel.shown.path : ""
    }
}
