import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: depth

    required property var host
    readonly property real scale: host.rig.depth?.scale ?? 1
    readonly property real most: 10
    property real dragged: -1
    readonly property real shown: dragged >= 0 ? dragged : scale

    function commit(value) {
        host.gizmo.page.run("parts", {
            "layer": host.layerId,
            "depth": {
                "scale": Math.round(Math.max(0, Math.min(most, value)) * 10) / 10
            }
        }, true);
    }

    Rectangle {
        x: Kirigami.Units.gridUnit
        y: depth.height - Kirigami.Units.gridUnit * 1.5
        width: depth.width - Kirigami.Units.gridUnit * 2
        height: 3
        radius: 1
        color: Kirigami.Theme.neutralTextColor
        opacity: 0.7
    }

    Rectangle {
        id: knob

        x: Kirigami.Units.gridUnit + (depth.width - Kirigami.Units.gridUnit * 2) * depth.shown / depth.most - width / 2
        y: depth.height - Kirigami.Units.gridUnit * 1.5 - height / 2 + 1.5
        width: Kirigami.Units.gridUnit * 0.9
        height: width
        radius: 2
        color: drag.active ? Kirigami.Theme.positiveTextColor : Kirigami.Theme.neutralTextColor
        border.color: Kirigami.Theme.backgroundColor
        z: 6

        QQC2.Label {
            anchors.bottom: parent.top
            anchors.horizontalCenter: parent.horizontalCenter
            text: depth.shown.toFixed(1)
            style: Text.Outline
            styleColor: Kirigami.Theme.backgroundColor
        }

        QQC2.ToolTip.visible: hover.hovered
        QQC2.ToolTip.text: App.tr("Perspective of the depth")

        HoverHandler {
            id: hover
        }

        DragHandler {
            id: drag

            property real from: 1

            target: null
            yAxis.enabled: false
            onActiveChanged: {
                if (active) {
                    from = depth.scale;
                    return;
                }
                depth.dragged = -1;
            }
            onTranslationChanged: {
                if (!active)
                    return;
                depth.dragged = Math.max(0, Math.min(depth.most, from + translation.x / (depth.width - Kirigami.Units.gridUnit * 2) * depth.most));
                depth.commit(depth.dragged);
            }
        }
    }
}
