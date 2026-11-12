import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: tree

    required property var page
    readonly property var templates: page.view.templates ?? []
    property string target: ""

    function look(name) {
        const kept = JSON.parse(page.editor.look || "{}");
        if (name)
            kept.template = name;
        else
            delete kept.template;
        page.editor.look = Object.keys(kept).length > 0 ? JSON.stringify(kept) : "";
        page.editor.seek(page.clock, page.editor.playing);
        if (name)
            page.editor.select("/scene/templates/" + name);
    }

    visible: templates.length > 0

    QQC2.Label {
        Layout.fillWidth: true
        text: App.tr("Templates")
        font.bold: true
    }

    SceneRepeater {
        id: list

        rows: tree.templates

        delegate: ColumnLayout {
            id: item

            required property int index
            readonly property var template: list.rows[index] ?? ({})

            Layout.fillWidth: true
            spacing: 0

            RowLayout {
                Layout.fillWidth: true

                QQC2.Label {
                    Layout.fillWidth: true
                    elide: Text.ElideRight
                    text: (item.template.scripts ?? []).length > 0 ? App.tr("%1 · named by %2", item.template.name, item.template.scripts.join(", ")) : item.template.name
                    font.bold: true
                }

                QQC2.ToolButton {
                    icon.name: "view-preview"
                    text: App.tr("Show it alone")
                    display: QQC2.AbstractButton.IconOnly
                    QQC2.ToolTip.text: text
                    QQC2.ToolTip.visible: hovered
                    onClicked: tree.look(JSON.parse(tree.page.editor.look || "{}").template === item.template.name ? "" : item.template.name)
                }

                QQC2.ToolButton {
                    icon.name: "edit-rename"
                    text: App.tr("Rename…")
                    display: QQC2.AbstractButton.IconOnly
                    QQC2.ToolTip.text: text
                    QQC2.ToolTip.visible: hovered
                    onClicked: {
                        tree.target = item.template.name;
                        renaming.open();
                    }
                }

                QQC2.ToolButton {
                    icon.name: "edit-delete"
                    text: App.tr("Remove")
                    display: QQC2.AbstractButton.IconOnly
                    QQC2.ToolTip.text: text
                    QQC2.ToolTip.visible: hovered
                    onClicked: tree.page.run("template", {
                            "remove": {
                                "name": item.template.name
                            }
                        })
                }
            }

            Repeater {
                model: item.template.layers ?? []

                delegate: QQC2.ItemDelegate {
                    id: node

                    required property var modelData

                    Layout.fillWidth: true
                    leftPadding: Kirigami.Units.largeSpacing * (node.modelData.depth + 1)
                    highlighted: tree.page.editor.selected === node.modelData.pointer
                    text: [node.modelData.id, SceneValues.kinds[node.modelData.type] ?? node.modelData.type].join(" · ")
                    onClicked: tree.page.editor.select(node.modelData.pointer)
                }
            }
        }
    }

    Kirigami.PromptDialog {
        id: renaming

        title: App.tr("Rename the template")
        subtitle: App.tr("Scripts that spawn it by name are not rewritten.")
        standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel
        onOpened: {
            name.text = tree.target;
            name.forceActiveFocus();
            name.selectAll();
        }
        onAccepted: tree.page.run("template", {
                "rename": {
                    "name": tree.target,
                    "to": name.text.trim()
                }
            })

        QQC2.TextField {
            id: name

            Layout.fillWidth: true
            onAccepted: renaming.accept()
        }
    }
}
