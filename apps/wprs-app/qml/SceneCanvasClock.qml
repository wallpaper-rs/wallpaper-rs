import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.ToolButton {
    id: clock

    required property var page
    readonly property var size: page.view.canvas?.size ?? [1920, 1080]
    readonly property bool fresh: page.editor.value("/scene/clock/seed") === "null"

    function canvas(args) {
        page.run("canvas", args);
    }

    icon.name: "view-fullscreen"
    text: App.tr("Canvas and clock")
    display: QQC2.AbstractButton.IconOnly
    onClicked: menu.open()
    QQC2.ToolTip.text: App.tr("Canvas %1 × %2: screen sizes, still frame and seed", size[0], size[1])
    QQC2.ToolTip.visible: hovered

    QQC2.Menu {
        id: menu

        QQC2.Menu {
            title: App.tr("Screen size")

            Repeater {
                model: clock.page.view.screen_sizes ?? []

                delegate: QQC2.MenuItem {
                    required property var modelData

                    text: SceneScreens.label(modelData.name) + " (" + modelData.size[0] + " × " + modelData.size[1] + ")"
                    checkable: true
                    checked: clock.size[0] === modelData.size[0] && clock.size[1] === modelData.size[1]
                    onTriggered: clock.canvas({
                        "size": {
                            "size": modelData.size
                        }
                    })
                }
            }
        }

        QQC2.MenuItem {
            text: App.tr("Use the playhead as the still frame (%1 s)", Math.round(clock.page.editor.time * 100) / 100)
            onTriggered: clock.canvas({
                "clock": {
                    "poster": clock.page.editor.time
                }
            })
        }

        QQC2.MenuItem {
            text: App.tr("A new seed each time the scene starts")
            checkable: true
            checked: clock.fresh
            onTriggered: clock.canvas({
                "clock": clock.fresh ? {
                    "seed": 1
                } : {
                    "fresh": true
                }
            })
        }
    }
}
