import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Row {
    id: swatches

    property var colors: null

    visible: Array.isArray(swatches.colors)
    spacing: Kirigami.Units.smallSpacing

    Repeater {
        model: [App.tr("Primary"), App.tr("Secondary"), App.tr("Tertiary"), App.tr("Text"), App.tr("Contrast")]

        delegate: Rectangle {
            required property int index
            required property string modelData

            width: Kirigami.Units.gridUnit * 1.5
            height: width
            radius: Kirigami.Units.smallSpacing
            border.color: Kirigami.Theme.textColor
            color: (swatches.colors ?? [])[index] ?? "transparent"
            QQC2.ToolTip.text: modelData + " " + (swatches.colors ?? [])[index]
            QQC2.ToolTip.visible: hover.hovered

            HoverHandler {
                id: hover
            }
        }
    }
}
