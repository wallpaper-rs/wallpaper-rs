import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.ToolButton {
    id: seeAs

    required property var page
    readonly property alias window: window

    icon.name: "caret-down"
    text: App.tr("See as: %1", SceneLook.names[Math.max(0, SceneLook.levels.indexOf(SceneLook.quality))])
    checked: window.visible
    QQC2.ToolTip.text: App.tr("Show the preview as the daemon draws it at a quality level, on battery or with reduced motion, and compare two levels")
    QQC2.ToolTip.visible: hovered && !window.visible
    onClicked: window.visible ? window.close() : window.open()

    QQC2.Popup {
        id: window

        x: seeAs.width - width
        y: seeAs.height
        width: Look.panelChico
        padding: Look.e3
        closePolicy: QQC2.Popup.CloseOnEscape | QQC2.Popup.CloseOnPressOutsideParent
        onClosed: seeAs.forceActiveFocus()

        contentItem: ScenePreviewPanel {
            page: seeAs.page
        }
    }
}
