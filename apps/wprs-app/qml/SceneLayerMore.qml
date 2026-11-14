import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.Menu {
    id: more

    required property var owner
    required property var tools
    readonly property bool chosen: owner.current !== null

    QQC2.MenuItem {
        text: App.tr("Ungroup")
        enabled: more.chosen && more.owner.current.type === "group"
        onTriggered: more.owner.page.run("ungroup", {
                "id": more.owner.current.id
            })
    }

    QQC2.Menu {
        title: App.tr("Move into a group")
        enabled: more.owner.groups.length > 0

        SceneRepeater {
            id: listed

            rows: more.owner.groups

            delegate: QQC2.MenuItem {
                required property int index

                text: listed.rows[index].id
                onTriggered: more.owner.move(1000, listed.rows[index].id)
            }
        }
    }

    QQC2.MenuItem {
        text: App.tr("Move out of the group")
        enabled: more.owner.group !== ""
        onTriggered: more.owner.move(1000)
    }

    QQC2.MenuSeparator {
    }

    QQC2.MenuItem {
        text: App.tr("Copy the layers")
        enabled: more.chosen
        onTriggered: clip.copy()
    }

    QQC2.MenuItem {
        text: App.tr("Paste the layers")
        enabled: SceneClip.pasteable
        onTriggered: clip.paste()
    }

    QQC2.MenuItem {
        text: App.tr("Rename the layer")
        enabled: more.chosen
        onTriggered: clip.rename()
    }

    QQC2.Menu {
        title: App.tr("Move on its own")
        enabled: more.chosen

        Repeater {
            model: starts.moves

            delegate: QQC2.MenuItem {
                required property var modelData

                text: modelData[2]
                onTriggered: starts.keepMoving(modelData)
            }
        }
    }

    QQC2.MenuSeparator {
    }

    QQC2.MenuItem {
        text: App.tr("Make a template")
        enabled: more.chosen && ["group", "image", "text", "shape", "solid", "spectrum", "mesh"].includes(more.owner.current.type)
        onTriggered: {
            more.owner.page.run("template", {
                    "make": {
                        "layers": more.tools.picked()
                    }
                });
            more.owner.page.picked = [];
        }
    }

    QQC2.MenuItem {
        text: App.tr("Particle systems")
        onTriggered: more.owner.page.editor.select("/scene/particles")
    }

    SceneLayerStarts {
        id: starts

        owner: more.owner
    }

    SceneLayerClip {
        id: clip

        owner: more.owner
    }
}
