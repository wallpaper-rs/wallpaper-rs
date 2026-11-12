import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Dialogs
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: brush

    required property var holder
    property color ink: holder.map.slot === "normal" ? "#8080ff" : "#ffffff"
    property var points: []
    property int strokes: 0

    title: App.tr("Paint the map %1", holder.owner.names[holder.map.slot])
    standardButtons: Kirigami.Dialog.Close
    preferredWidth: Kirigami.Units.gridUnit * 24

    ColumnLayout {
        Image {
            id: canvas

            Layout.preferredWidth: Kirigami.Units.gridUnit * 20
            Layout.preferredHeight: Layout.preferredWidth
            fillMode: Image.Stretch
            cache: false
            source: (brush.holder.chosen ? brush.holder.url(brush.holder.chosen) : (brush.holder.map.thumb ?? "")) + (brush.holder.chosen ? "?" + brush.strokes : "")

            Rectangle {
                anchors.fill: parent
                color: "transparent"
                border.color: Kirigami.Theme.disabledTextColor
            }

            DragHandler {
                target: null
                onActiveChanged: {
                    if (!active && brush.points.length > 0) {
                        brush.holder.owner.panel.page.run("scene3d", {
                            "paint": {
                                "layer": brush.holder.owner.panel.figure,
                                "name": brush.holder.owner.material.name,
                                "slot": brush.holder.map.slot,
                                "color": [brush.ink.r, brush.ink.g, brush.ink.b],
                                "radius": size.value,
                                "hardness": hard.value,
                                "points": brush.points
                            }
                        });
                        brush.strokes += 1;
                    }
                    brush.points = [];
                }
                onCentroidChanged: {
                    if (active)
                        brush.points = brush.points.concat([[Math.min(Math.max(centroid.position.x / canvas.width, 0), 1), Math.min(Math.max(centroid.position.y / canvas.height, 0), 1)]]);
                }
            }
        }

        RowLayout {
            QQC2.Button {
                text: App.tr("Color")
                icon.name: "color-picker"
                onClicked: colors.open()

                Rectangle {
                    anchors.right: parent.right
                    anchors.verticalCenter: parent.verticalCenter
                    anchors.rightMargin: Kirigami.Units.smallSpacing
                    width: Kirigami.Units.gridUnit
                    height: width
                    color: brush.ink
                }
            }

            QQC2.Label {
                text: App.tr("Size")
            }

            QQC2.Slider {
                id: size

                from: 0.01
                to: 0.5
                value: 0.05
            }

            QQC2.Label {
                text: App.tr("Hardness")
            }

            QQC2.Slider {
                id: hard

                from: 0
                to: 0.99
                value: 0.5
            }
        }
    }

    ColorDialog {
        id: colors

        selectedColor: brush.ink
        onAccepted: brush.ink = selectedColor
    }
}
