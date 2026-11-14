import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.Button {
    id: tools

    required property var owner
    readonly property var common: ["image", "video", "text", "shape", "particles"]
    readonly property var kinds: (owner.page.view.kinds ?? []).filter(kind => kind !== "light")

    function usable(kind) {
        return kind === "shapes" ? (owner.page.view.pools ?? []).length > 0 : !["image", "video", "sound", "mesh", "model"].includes(kind) || (owner.page.view.assets ?? []).some(asset => asset.kind === (kind === "mesh" ? "model" : kind));
    }

    function open() {
        menu.popup(tools, 0, tools.height);
    }

    function offer(kind) {
        choice.offer(kind);
    }

    text: App.tr("Add a layer")
    icon.name: "plus"
    onClicked: open()

    QQC2.Menu {
        id: menu

        onAboutToShow: starts.refresh()

        SceneRepeater {
            id: firsts

            rows: tools.common.filter(kind => tools.kinds.includes(kind)).slice(0, 3)

            delegate: SceneLayerKind {
                required property int index

                owner: tools
                kind: firsts.rows[index]
            }
        }

        QQC2.Menu {
            title: App.tr("Time or date")
            icon.name: "clock"

            SceneRepeater {
                id: clocks

                rows: starts.formats

                delegate: QQC2.MenuItem {
                    required property int index
                    readonly property var modelData: clocks.rows[index]

                    text: modelData[1] + " → " + (starts.samples[modelData[1]] ?? "")
                    onTriggered: starts.addClock(modelData)
                }
            }
        }

        SceneRepeater {
            id: seconds

            rows: tools.common.filter(kind => tools.kinds.includes(kind)).slice(3)

            delegate: SceneLayerKind {
                required property int index

                owner: tools
                kind: seconds.rows[index]
            }
        }

        QQC2.MenuSeparator {
        }

        SceneRepeater {
            id: rest

            rows: tools.kinds.filter(kind => !tools.common.includes(kind))

            delegate: SceneLayerKind {
                required property int index

                owner: tools
                kind: rest.rows[index]
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

    SceneLayerStarts {
        id: starts

        owner: tools.owner
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
