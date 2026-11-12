import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: panel

    required property var page
    readonly property var scripts: visible ? (page.view, JSON.parse(page.editor.rig("", JSON.stringify({
                    "uses": {}
                })) || "{}").scripts ?? []) : []

    QQC2.Label {
        Layout.fillWidth: true
        visible: panel.scripts.length > 0
        text: App.tr("What the scripts declare")
        font.bold: true
    }

    ListView {
        Layout.fillWidth: true
        Layout.preferredHeight: contentHeight
        interactive: false
        model: panel.scripts

        delegate: QQC2.ItemDelegate {
            id: row

            required property var modelData

            width: ListView.view.width
            text: modelData.path + " · " + (modelData.declared.length ? modelData.declared.join(", ") : App.tr("declares nothing")) + (modelData.missing.length ? " · " + App.tr("lacks %1", modelData.missing.join(", ")) : "")
            icon.name: modelData.missing.length ? "data-warning" : "dialog-ok"
            onClicked: panel.page.jump(modelData.pointer)

            QQC2.ToolButton {
                anchors.right: parent.right
                visible: row.modelData.missing.length > 0
                icon.name: "dialog-ok-apply"
                text: App.tr("Declare what it uses")
                QQC2.ToolTip.text: App.tr("Adds %1 to the uses of the script", row.modelData.missing.join(", "))
                QQC2.ToolTip.visible: hovered
                onClicked: panel.page.run("uses", {
                    "path": row.modelData.path
                })
            }
        }
    }
}
