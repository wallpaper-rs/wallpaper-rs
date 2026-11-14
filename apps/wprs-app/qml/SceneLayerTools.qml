import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: tools

    required property var owner
    readonly property bool chosen: owner.current !== null

    function picked() {
        return [owner.current.id].concat(owner.page.picked.filter(id => id !== owner.current.id));
    }

    function add() {
        adder.open();
    }

    function remove() {
        const page = owner.page;
        const id = owner.current.id;
        const lost = JSON.parse(page.editor.rig("", JSON.stringify({
                "unkept": {
                    "id": id
                }
            })) || "{}").layers ?? [];
        page.run("remove_layer", {
                "id": id
            });
        if (lost.length > 0 && page.editor.error === "")
            page.notice = App.tr("%1 followed the removed layer and a value of its place is bound to something other than keyframes, so it could not be kept where it was: check where it sits now.", lost.join(", "));
    }

    spacing: Look.e2

    SceneLayerAdd {
        id: adder

        Layout.fillWidth: true
        implicitHeight: Look.altoBotonArriba
        owner: tools.owner
    }

    RowLayout {
        Layout.fillWidth: true
        Layout.minimumWidth: 0
        spacing: 0

        QQC2.ToolButton {
            padding: Look.e1
            icon.name: "copy"
            text: App.tr("Duplicate")
            display: QQC2.AbstractButton.IconOnly
            enabled: tools.chosen
            QQC2.ToolTip.text: text
            QQC2.ToolTip.visible: hovered
            onClicked: tools.owner.page.run("duplicate_layer", {
                    "id": tools.owner.current.id
                })
        }

        QQC2.ToolButton {
            padding: Look.e1
            icon.name: "stack"
            text: App.tr("Group the layers")
            display: QQC2.AbstractButton.IconOnly
            enabled: tools.chosen
            QQC2.ToolTip.text: App.tr("Group the selected layer and the ones picked with Ctrl")
            QQC2.ToolTip.visible: hovered
            onClicked: {
                tools.owner.page.run("group", {
                        "ids": tools.picked()
                    });
                tools.owner.page.picked = [];
            }
        }

        QQC2.ToolButton {
            padding: Look.e1
            icon.name: "trash"
            text: App.tr("Remove")
            display: QQC2.AbstractButton.IconOnly
            enabled: tools.chosen
            QQC2.ToolTip.text: text
            QQC2.ToolTip.visible: hovered
            onClicked: tools.remove()
        }

        QQC2.ToolButton {
            padding: Look.e1
            icon.name: "arrow-up"
            text: App.tr("Raise")
            display: QQC2.AbstractButton.IconOnly
            enabled: tools.chosen && tools.owner.index < tools.owner.siblings.length - 1
            QQC2.ToolTip.text: text
            QQC2.ToolTip.visible: hovered
            onClicked: tools.owner.move(tools.owner.index + 1, tools.owner.group)
        }

        QQC2.ToolButton {
            padding: Look.e1
            icon.name: "arrow-down"
            text: App.tr("Lower")
            display: QQC2.AbstractButton.IconOnly
            enabled: tools.chosen && tools.owner.index > 0
            QQC2.ToolTip.text: text
            QQC2.ToolTip.visible: hovered
            onClicked: tools.owner.move(tools.owner.index - 1, tools.owner.group)
        }

        Item {
            Layout.fillWidth: true
        }

        QQC2.ToolButton {
            id: more

            padding: Look.e1
            text: App.tr("More")
            icon.name: "caret-down"
            onClicked: menu.popup(more, 0, more.height)

            SceneLayerMore {
                id: menu

                owner: tools.owner
                tools: tools
            }
        }
    }

    SceneTemplates {
        Layout.fillWidth: true
        page: tools.owner.page
    }
}
