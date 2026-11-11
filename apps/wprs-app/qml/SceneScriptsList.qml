import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: panel

    required property var page

    QQC2.Label {
        Layout.fillWidth: true
        visible: scripts.count > 0
        text: App.tr("Scripts")
        font.bold: true
    }

    ListView {
        id: scripts

        Layout.fillWidth: true
        Layout.preferredHeight: contentHeight
        interactive: false
        model: panel.page.view.scripts ?? []

        delegate: QQC2.ItemDelegate {
            id: script

            required property var modelData

            width: ListView.view.width
            text: [modelData.path, App.bytes(modelData.size)].join(" · ")

            QQC2.ToolButton {
                anchors.right: openScript.left
                icon.name: "document-preview"
                text: App.tr("View source…")
                display: QQC2.AbstractButton.IconOnly
                QQC2.ToolTip.text: text
                QQC2.ToolTip.visible: hovered
                onClicked: viewer.show(script.modelData.path)
            }

            QQC2.ToolButton {
                id: openScript

                anchors.right: removeScript.left
                icon.name: "document-open"
                text: App.tr("Open in your editor…")
                display: QQC2.AbstractButton.IconOnly
                QQC2.ToolTip.text: text
                QQC2.ToolTip.visible: hovered
                onClicked: Qt.openUrlExternally("file://" + panel.page.editor.folder + "/" + script.modelData.path)
            }

            QQC2.ToolButton {
                id: removeScript

                anchors.right: parent.right
                icon.name: "edit-delete"
                text: App.tr("Remove")
                display: QQC2.AbstractButton.IconOnly
                onClicked: panel.page.run("remove", {
                    "pointer": script.modelData.pointer
                })
            }
        }
    }

    SceneScriptCode {
        id: viewer

        page: panel.page
    }
}
