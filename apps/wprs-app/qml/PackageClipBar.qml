import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: bar

    required property var owner

    Layout.fillWidth: true
    Layout.preferredHeight: Kirigami.Units.gridUnit * 3
    visible: !!owner.info?.present

    Rectangle {
        anchors.fill: parent
        radius: Kirigami.Units.smallSpacing
        color: Kirigami.Theme.alternateBackgroundColor
    }

    Row {
        anchors.fill: parent

        Repeater {
            model: owner.frames

            delegate: Image {
                required property var modelData

                width: bar.width / Math.max(1, owner.frames.length)
                height: bar.height
                source: App.url(modelData.file)
                fillMode: Image.PreserveAspectCrop
                asynchronous: true
                opacity: 0.55
            }
        }
    }

    Rectangle {
        x: owner.from / owner.length * bar.width
        width: Math.max(2, (owner.to - owner.from) / owner.length * bar.width)
        height: bar.height
        color: "transparent"
        border.color: Kirigami.Theme.highlightColor
        border.width: 3
    }

    Repeater {
        model: ["start", "end"]

        delegate: Rectangle {
            id: grip

            required property string modelData
            readonly property real seconds: modelData === "start" ? owner.from : owner.to
            readonly property string held: ((x + width / 2) / bar.width * owner.length).toFixed(2)

            x: seconds / owner.length * bar.width - width / 2
            width: Kirigami.Units.gridUnit * 0.7
            height: bar.height
            radius: 2
            color: Kirigami.Theme.highlightColor
            border.color: Kirigami.Theme.backgroundColor

            HoverHandler {
                id: over
            }

            QQC2.ToolTip.visible: over.hovered || move.active
            QQC2.ToolTip.text: modelData === "start" ? App.tr("Start %1 s", held) : App.tr("End %1 s", held)

            DragHandler {
                id: move

                target: grip
                yAxis.enabled: false
                xAxis.minimum: -grip.width / 2
                xAxis.maximum: bar.width - grip.width / 2
                onActiveChanged: {
                    if (!active)
                        owner.trim(grip.modelData, (grip.x + grip.width / 2) / bar.width * owner.length);
                }
            }
        }
    }
}
