import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: line

    required property var glow
    property real start

    visible: glow.shown

    Rectangle {
        x: line.glow.width * line.glow.at - width / 2
        width: 2
        height: line.glow.height
        color: Kirigami.Theme.highlightColor
    }

    QQC2.Label {
        x: Kirigami.Units.smallSpacing
        text: App.tr("With the glow")
        color: "white"
        style: Text.Outline
    }

    QQC2.Label {
        x: line.glow.width - width - Kirigami.Units.smallSpacing
        text: App.tr("Without the glow")
        color: "white"
        style: Text.Outline
    }

    Rectangle {
        x: line.glow.width * line.glow.at - width / 2
        y: (line.glow.height - height) / 2
        width: Kirigami.Units.gridUnit
        height: Kirigami.Units.gridUnit * 2
        radius: width / 2
        color: drag.active ? Kirigami.Theme.positiveTextColor : Kirigami.Theme.highlightColor
        border.color: Kirigami.Theme.backgroundColor

        QQC2.ToolTip.text: App.tr("Drag to move the split between the glow and no glow")
        QQC2.ToolTip.visible: hover.hovered

        HoverHandler {
            id: hover
        }

        DragHandler {
            id: drag

            target: null
            yAxis.enabled: false
            onActiveChanged: {
                if (active)
                    line.start = line.glow.at;
            }
            onTranslationChanged: {
                if (active)
                    line.glow.drag(line.start * line.glow.width + translation.x);
            }
        }
    }
}
