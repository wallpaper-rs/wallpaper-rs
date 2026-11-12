import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.ToolButton {
    id: tool

    required property var page
    readonly property bool glowing: (page.view.canvas?.bloom ?? null) !== null
    readonly property bool halved: (JSON.parse(page.editor.split || "{}").on ?? false) === true

    icon.name: "view-split-left-right"
    text: App.tr("Compare the glow")
    display: QQC2.AbstractButton.IconOnly
    visible: glowing
    checkable: true
    checked: halved
    onToggled: page.editor.halve(checked)
    onGlowingChanged: {
        if (!glowing && halved)
            page.editor.halve(false);
    }
    QQC2.ToolTip.text: App.tr("Show the right part of the canvas without the glow and the left part with it; drag the line to move the split")
    QQC2.ToolTip.visible: hovered
}
