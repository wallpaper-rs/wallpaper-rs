import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: over

    required property var tools
    readonly property var made: tools.info?.render ?? ({})
    readonly property var sizes: made.size_over_life ?? []
    readonly property var colors: made.color_over_life ?? []

    Kirigami.Heading {
        level: 4
        text: App.tr("Along the life of a particle")
    }

    RowLayout {
        QQC2.Label {
            text: App.tr("Size over life")
        }

        QQC2.Button {
            objectName: "sizeOverLife"
            text: over.sizes.length > 0 ? App.tr("Remove") : App.tr("Shape the size")
            icon.name: over.sizes.length > 0 ? "edit-delete" : "list-add"
            onClicked: over.tools.edit("over", {
                    "size": over.sizes.length > 0 ? [] : [[0, 1], [1, 1]]
                })
        }
    }

    SceneCurve {
        objectName: "sizeCurve"
        Layout.fillWidth: true
        visible: over.sizes.length > 0
        input: QtObject {
            readonly property var value: over.sizes
            readonly property var field: ({
                    "range": [0, 2]
                })

            function commit(points, gesture) {
                over.tools.edit("over", {
                    "size": points
                }, gesture);
            }
        }
    }

    RowLayout {
        QQC2.Label {
            text: App.tr("Color over life")
        }

        QQC2.Button {
            objectName: "colorOverLife"
            text: over.colors.length > 0 ? App.tr("Remove") : App.tr("Fade the color")
            icon.name: over.colors.length > 0 ? "edit-delete" : "list-add"
            onClicked: over.tools.edit("over", {
                    "color": over.colors.length > 0 ? [] : [[0, "#ffffffff"], [1, "#ffffff00"]]
                })
        }
    }

    SceneGradientBar {
        objectName: "colorBar"
        Layout.fillWidth: true
        visible: over.colors.length > 0
        stops: over.colors
        onEdited: (stops, gesture) => over.tools.edit("over", {
                "color": stops
            }, gesture)
    }
}
