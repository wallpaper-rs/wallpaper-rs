import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

RowLayout {
    id: seen

    required property var row
    readonly property var page: row.panel.page
    readonly property var debug: JSON.parse(page.editor.value("/scene/ext/editor/debug3d") || "null")
    readonly property bool mine: debug?.layer === row.panel.figure && debug?.material === row.material.name
    readonly property var kinds: ["", "mask", "matte", "shadows"]

    QQC2.Label {
        text: App.tr("Debug view")
    }

    QQC2.ComboBox {
        Layout.fillWidth: true
        model: [App.tr("None"), App.tr("Sway mask of the vertex colors"), App.tr("Matte"), App.tr("Where shadows fall")]
        currentIndex: seen.mine ? Math.max(0, seen.kinds.indexOf(seen.debug.show)) : 0
        onActivated: index => seen.page.run("scene3d", {
                    "debug": index > 0 ? {
                        "show": seen.kinds[index],
                        "layer": seen.row.panel.figure,
                        "material": seen.row.material.name
                    } : {}
                }, true)
        QQC2.ToolTip.visible: hovered
        QQC2.ToolTip.text: App.tr("Only this material shows it, only in this preview; the wallpaper never does")
    }
}
