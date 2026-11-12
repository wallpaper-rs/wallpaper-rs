import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.ToolButton {
    id: tools

    required property var owner

    text: App.tr("Add layer")
    icon.name: "list-add"
    onClicked: kinds.popup()

    QQC2.Menu {
        id: kinds

        SceneRepeater {
            id: listed2

            rows: tools.owner.page.view.kinds ?? []

            delegate: QQC2.MenuItem {
                required property int index
                readonly property var modelData: listed2.rows[index]

                visible: modelData !== "light"
                height: visible ? implicitHeight : 0
                enabled: modelData === "shapes" ? (tools.owner.page.view.pools ?? []).length > 0 : !["image", "video", "sound", "mesh", "model"].includes(modelData) || (tools.owner.page.view.assets ?? []).some(asset => asset.kind === (modelData === "mesh" ? "model" : modelData))
                text: SceneWords.kind(modelData) + (enabled ? "" : " (" + (modelData === "shapes" ? App.tr("add a pool first") : App.tr("import a file of this kind first")) + ")")
                icon.name: tools.owner.icon(modelData)
                onTriggered: choice.offer(modelData)
            }
        }

        QQC2.Menu {
            title: App.tr("Light")

            Repeater {
                model: [["point", App.tr("Point light")], ["spot", App.tr("Spot light")], ["tube", App.tr("Tube light")], ["directional", App.tr("Directional light")]]

                delegate: QQC2.MenuItem {
                    required property var modelData

                    text: modelData[1]
                    onTriggered: tools.owner.page.run("light", {
                            "add": {
                                "light": modelData[0],
                                "into": tools.owner.current?.type === "group" ? tools.owner.current.id : undefined
                            }
                        })
                }
            }
        }

        SceneLayerFiles {
            owner: tools.owner
        }

        QQC2.MenuItem {
            text: App.tr("Mesh from a picture…")
            icon.name: "draw-polyline"
            enabled: (tools.owner.page.view.assets ?? []).some(asset => asset.kind === "image")
            onTriggered: cutter.open()
        }
    }

    SceneAssetChoice {
        id: choice

        owner: tools.owner
    }

    SceneMeshCut {
        id: cutter

        page: tools.owner.page
    }
}
