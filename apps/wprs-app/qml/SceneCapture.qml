import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.ToolButton {
    id: capture

    required property var page
    readonly property alias window: window

    icon.name: "download-simple"
    text: App.tr("Capture")
    checked: window.visible
    QQC2.ToolTip.text: App.tr("Save a picture or a clip of what the canvas shows, with the values of Try")
    QQC2.ToolTip.visible: hovered && !window.visible
    onClicked: window.visible ? window.close() : window.open()

    QQC2.Popup {
        id: window

        x: capture.width - width
        y: capture.height
        width: Look.panelChico
        padding: Look.e3
        closePolicy: QQC2.Popup.CloseOnEscape | QQC2.Popup.CloseOnPressOutsideParent
        onClosed: capture.forceActiveFocus()

        contentItem: ScenePreviewSave {
            page: capture.page
        }
    }
}
