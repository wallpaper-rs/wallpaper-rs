import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Flow {
    id: tools

    required property var owner

    SceneLayerAdd {
        owner: tools.owner
    }

    SceneLayerStarts {
        owner: tools.owner
    }

    Repeater {
        model: [["duplicate_layer", "edit-copy", App.tr("Duplicate")], ["remove_layer", "edit-delete", App.tr("Remove")], ["ungroup", "format-remove-node", App.tr("Ungroup")]]

        delegate: QQC2.ToolButton {
            required property var modelData

            icon.name: modelData[1]
            text: modelData[2]
            display: QQC2.AbstractButton.IconOnly
            enabled: tools.owner.current !== null && (modelData[0] !== "ungroup" || tools.owner.current.type === "group")
            onClicked: {
                const page = tools.owner.page;
                const id = tools.owner.current.id;
                const lost = modelData[0] === "remove_layer" ? (JSON.parse(page.editor.rig("", JSON.stringify({
                        "unkept": {
                            "id": id
                        }
                    })) || "{}").layers ?? []) : [];
                page.run(modelData[0], {
                        "id": id
                    });
                if (lost.length > 0 && page.editor.error === "")
                    page.notice = App.tr("%1 followed the removed layer and a value of its place is bound to something other than keyframes, so it could not be kept where it was: check where it sits now.", lost.join(", "));
            }
        }
    }

    QQC2.ToolButton {
        icon.name: "document-save-as-template"
        text: App.tr("Make a template")
        display: QQC2.AbstractButton.IconOnly
        QQC2.ToolTip.text: App.tr("Copy the selected layer and the ones picked with Ctrl to the templates that scripts spawn")
        QQC2.ToolTip.visible: hovered
        enabled: tools.owner.current !== null && ["group", "image", "text", "shape", "solid", "spectrum", "mesh"].includes(tools.owner.current.type)
        onClicked: {
            tools.owner.page.run("template", {
                    "make": {
                        "layers": [tools.owner.current.id].concat(tools.owner.page.picked.filter(id => id !== tools.owner.current.id))
                    }
                });
            tools.owner.page.picked = [];
        }
    }

    QQC2.ToolButton {
        icon.name: "weather-snow"
        text: App.tr("Particle systems")
        display: QQC2.AbstractButton.IconOnly
        QQC2.ToolTip.text: App.tr("Create, copy, rename and remove the particle systems of the scene")
        QQC2.ToolTip.visible: hovered
        onClicked: tools.owner.page.editor.select("/scene/particles")
    }

    SceneTemplates {
        page: tools.owner.page
    }

    SceneLayerClip {
        owner: tools.owner
    }

    QQC2.ToolButton {
        icon.name: "object-group"
        text: App.tr("Group the layers")
        display: QQC2.AbstractButton.IconOnly
        QQC2.ToolTip.text: App.tr("Group the selected layer and the ones picked with Ctrl")
        QQC2.ToolTip.visible: hovered
        enabled: tools.owner.current !== null
        onClicked: {
            tools.owner.page.run("group", {
                    "ids": [tools.owner.current.id].concat(tools.owner.page.picked.filter(id => id !== tools.owner.current.id))
                });
            tools.owner.page.picked = [];
        }
    }

    QQC2.ToolButton {
        icon.name: "folder-new"
        text: App.tr("Move into a group")
        display: QQC2.AbstractButton.IconOnly
        enabled: tools.owner.groups.length > 0
        onClicked: into.popup()

        QQC2.Menu {
            id: into

            SceneRepeater {
                id: listed

                rows: tools.owner.groups

                delegate: QQC2.MenuItem {
                    required property int index
                    readonly property var modelData: listed.rows[index]

                    text: modelData.id
                    onTriggered: tools.owner.move(1000, modelData.id)
                }
            }
        }
    }

    QQC2.ToolButton {
        icon.name: "go-up"
        text: App.tr("Raise")
        display: QQC2.AbstractButton.IconOnly
        enabled: tools.owner.current !== null && tools.owner.index < tools.owner.siblings.length - 1
        onClicked: tools.owner.move(tools.owner.index + 1, tools.owner.group)
    }

    QQC2.ToolButton {
        icon.name: "go-down"
        text: App.tr("Lower")
        display: QQC2.AbstractButton.IconOnly
        enabled: tools.owner.current !== null && tools.owner.index > 0
        onClicked: tools.owner.move(tools.owner.index - 1, tools.owner.group)
    }

    QQC2.ToolButton {
        icon.name: "go-parent-folder"
        text: App.tr("Move out of the group")
        display: QQC2.AbstractButton.IconOnly
        enabled: tools.owner.group !== ""
        onClicked: tools.owner.move(1000)
    }
}
