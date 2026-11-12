import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

SceneSeat {
    id: overlay

    readonly property var found: page.editor.selected.match(/^(.*\/layers\/\d+)(\/.*)?$/)
    readonly property var owner: found ? (page.view.gizmos ?? []).find(g => g.pointer === found[1] && g.kind === "text") ?? null : null
    readonly property var shown: owner ? (page.view, JSON.parse(page.editor.rig("", JSON.stringify({
                        "text_box": {
                            "layer": owner.id,
                            "time": page.editor.time
                        }
                    })) || "null")) : null

    function wrap(to) {
        page.run("layer", {
                "figure": {
                    "text_width": {
                        "layer": owner.id,
                        "to": to,
                        "at": page.editor.time,
                        "replace": page.overwrite
                    }
                }
            }, true);
    }

    function unwrap() {
        page.run("layer", {
                "figure": {
                    "text_width": {
                        "layer": owner.id,
                        "remove": true
                    }
                }
            });
    }

    layerId: owner?.id ?? ""
    visible: shown !== null

    SceneOutline {
        corners: overlay.shown?.corners ?? []
        ratio: overlay.ratio
        chosen: true
        dashed: !(overlay.shown?.wrapped ?? false)
    }

    SceneKnob {
        at: overlay.shown?.handle ?? null
        ratio: overlay.ratio
        square: true
        tint: overlay.shown?.wrapped ? Kirigami.Theme.highlightColor : Kirigami.Theme.neutralTextColor
        tip: overlay.shown?.wrapped ? App.tr("Set the width of the text; double-click to stop wrapping") : App.tr("Drag to wrap the text at a width")
        onMoved: to => overlay.wrap(to)

        TapHandler {
            gesturePolicy: TapHandler.ReleaseWithinBounds
            onDoubleTapped: {
                if (overlay.shown?.wrapped)
                    overlay.unwrap();
            }
        }
    }

    QQC2.Label {
        x: (overlay.shown?.corners[0][0] ?? 0) * overlay.ratio
        y: (overlay.shown?.corners[0][1] ?? 0) * overlay.ratio - height
        visible: overlay.shown !== null
        text: App.tr("%1 × %2 px, %3 lines", Math.round(overlay.shown?.size[0] ?? 0), Math.round(overlay.shown?.size[1] ?? 0), overlay.shown?.lines ?? 0)
        style: Text.Outline
        styleColor: Kirigami.Theme.backgroundColor
    }
}
