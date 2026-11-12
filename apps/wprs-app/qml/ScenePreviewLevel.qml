import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

QQC2.ToolButton {
    id: level

    required property var page

    icon.name: "preferences-system-performance"
    text: App.tr("Preview level")
    display: QQC2.AbstractButton.IconOnly
    highlighted: window.visible
    QQC2.ToolTip.text: App.tr("Show the preview as the daemon draws it at a quality level, on battery or with reduced motion, and compare two levels")
    QQC2.ToolTip.visible: hovered && !window.visible
    onClicked: window.visible ? window.close() : window.open()

    QQC2.Popup {
        id: window

        y: level.height
        width: Kirigami.Units.gridUnit * 22
        padding: Kirigami.Units.largeSpacing
        closePolicy: QQC2.Popup.CloseOnEscape | QQC2.Popup.CloseOnPressOutsideParent

        contentItem: ScenePreviewPanel {
            page: level.page
        }
    }
}
