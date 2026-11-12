import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

RowLayout {
    id: tools

    required property var page
    readonly property var chosen: (page.view.gizmos ?? []).find(g => g.pointer === page.editor.selected) ?? null
    readonly property bool free: chosen !== null && !chosen.locked

    function fit(how) {
        page.run("layer", {
                "fit": {
                    "layer": chosen.id,
                    "fit": how,
                    "at": page.editor.time,
                    "replace": page.overwrite
                }
            });
    }

    SceneCanvasClock {
        page: tools.page
    }

    SceneCameraTool {
        page: tools.page
    }

    QQC2.ToolButton {
        icon.name: "snap-bounding-box-edges"
        text: App.tr("Magnets")
        display: QQC2.AbstractButton.IconOnly
        checkable: true
        checked: tools.page.magnets
        onToggled: tools.page.magnets = checked
        QQC2.ToolTip.text: App.tr("A dragged layer snaps to the edges and the middle of the canvas and of the other layers; hold Alt to place it freely, use the arrow keys (Shift for 10 px) to nudge it")
        QQC2.ToolTip.visible: hovered
    }

    QQC2.ToolButton {
        icon.name: "zoom-fit-best"
        text: App.tr("Fit to the canvas")
        display: QQC2.AbstractButton.IconOnly
        enabled: tools.free
        onClicked: fits.open()
        QQC2.ToolTip.text: text
        QQC2.ToolTip.visible: hovered

        QQC2.Menu {
            id: fits

            QQC2.MenuItem {
                text: App.tr("Cover the canvas")
                onTriggered: tools.fit("cover")
            }

            QQC2.MenuItem {
                text: App.tr("Fit inside the canvas")
                onTriggered: tools.fit("contain")
            }

            QQC2.MenuItem {
                text: App.tr("Stretch to the canvas")
                onTriggered: tools.fit("stretch")
            }
        }
    }

    QQC2.ToolButton {
        icon.name: "insert-link"
        text: App.tr("Follow a layer")
        display: QQC2.AbstractButton.IconOnly
        enabled: tools.free
        checkable: true
        checked: tools.page.picking === "parent"
        onToggled: tools.page.picking = checked ? "parent" : ""
        QQC2.ToolTip.text: checked ? App.tr("Tap the layer that %1 follows on the canvas; Escape cancels", tools.chosen?.id ?? "") : App.tr("Pick on the canvas the layer that the selected one follows, keeping it where it is")
        QQC2.ToolTip.visible: hovered || checked
    }

    QQC2.ToolButton {
        icon.name: "remove-link"
        text: App.tr("Stop following")
        display: QQC2.AbstractButton.IconOnly
        visible: !!tools.chosen?.parent
        enabled: tools.free
        onClicked: tools.page.run("layer", {
                "parent": {
                    "layer": tools.chosen.id,
                    "keep": true,
                    "at": tools.page.editor.time,
                    "replace": tools.page.overwrite
                }
            })
        QQC2.ToolTip.text: App.tr("Stop following %1, keeping the layer where it is", tools.chosen?.parent ?? "")
        QQC2.ToolTip.visible: hovered
    }
}
