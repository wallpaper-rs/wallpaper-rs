import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: panel

    required property var page
    readonly property var pools: page.view.pools ?? []

    QQC2.Label {
        Layout.fillWidth: true
        visible: panel.pools.length > 0
        text: App.tr("Shape pools")
        font.bold: true
    }

    ListView {
        Layout.fillWidth: true
        Layout.preferredHeight: contentHeight
        interactive: false
        model: panel.pools

        delegate: QQC2.ItemDelegate {
            id: row

            required property var modelData

            width: ListView.view.width
            text: [modelData.id, App.tr("%1 shapes of %2 vertices", modelData.max, modelData.max_vertices)].concat(modelData.layers.length ? [App.tr("drawn by %1", modelData.layers.join(", "))] : []).join(" · ")
            highlighted: panel.page.editor.selected === modelData.pointer
            onClicked: panel.page.editor.select(modelData.pointer)

            QQC2.ToolButton {
                anchors.right: change.left
                icon.name: "draw-polyline"
                text: App.tr("Add a shapes layer")
                display: QQC2.AbstractButton.IconOnly
                QQC2.ToolTip.text: text
                QQC2.ToolTip.visible: hovered
                onClicked: panel.page.run("add_layer", {
                    "kind": "shapes",
                    "source": row.modelData.id,
                    "name": row.modelData.id
                })
            }

            QQC2.ToolButton {
                id: change

                anchors.right: remove.left
                icon.name: "document-edit"
                text: App.tr("Change the pool…")
                display: QQC2.AbstractButton.IconOnly
                QQC2.ToolTip.text: text
                QQC2.ToolTip.visible: hovered
                onClicked: editing.show(row.modelData)
            }

            QQC2.ToolButton {
                id: remove

                anchors.right: parent.right
                icon.name: "edit-delete"
                text: App.tr("Remove")
                display: QQC2.AbstractButton.IconOnly
                QQC2.ToolTip.text: text
                QQC2.ToolTip.visible: hovered
                onClicked: panel.page.run("pool", {
                    "id": row.modelData.id,
                    "remove": true
                })
            }
        }
    }

    QQC2.ToolButton {
        text: App.tr("Add a pool…")
        icon.name: "list-add"
        onClicked: editing.show(null)
    }

    ScenePoolEdit {
        id: editing

        page: panel.page
    }
}
