import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: panel

    required property var page

    QQC2.Label {
        Layout.fillWidth: true
        visible: effects.count > 0
        text: App.tr("Effects")
        font.bold: true
    }

    ListView {
        id: effects

        Layout.fillWidth: true
        Layout.preferredHeight: contentHeight
        interactive: false
        model: panel.page.view.package_effects ?? []

        delegate: QQC2.ItemDelegate {
            id: effect

            required property var modelData

            width: ListView.view.width
            text: [modelData.id, App.bytes(modelData.size)].join(" · ")

            QQC2.ToolButton {
                id: codeEffect

                anchors.right: formEffect.left
                icon.name: "document-edit"
                text: App.tr("Edit the WGSL code…")
                display: QQC2.AbstractButton.IconOnly
                QQC2.ToolTip.text: text
                QQC2.ToolTip.visible: hovered
                onClicked: code.show(effect.modelData.id)
            }

            QQC2.ToolButton {
                id: formEffect

                anchors.right: reimportEffect.left
                icon.name: "configure"
                text: App.tr("Edit the descriptor…")
                display: QQC2.AbstractButton.IconOnly
                QQC2.ToolTip.text: text
                QQC2.ToolTip.visible: hovered
                onClicked: describer.show(effect.modelData.id)
            }

            QQC2.ToolButton {
                id: reimportEffect

                anchors.right: removeEffect.left
                icon.name: "view-refresh"
                text: App.tr("Reimport…")
                display: QQC2.AbstractButton.IconOnly
                QQC2.ToolTip.text: text
                QQC2.ToolTip.visible: hovered
                onClicked: files.open()
            }

            QQC2.ToolButton {
                id: removeEffect

                anchors.right: parent.right
                icon.name: "edit-delete"
                text: App.tr("Remove")
                display: QQC2.AbstractButton.IconOnly
                onClicked: panel.page.run("remove_effect", {
                        "effect": effect.modelData.id
                    })
            }
        }
    }

    QQC2.ToolButton {
        text: App.tr("Import an effect…")
        icon.name: "special-effects-symbolic"
        onClicked: files.open()
    }

    SceneNewEffect {
        Layout.fillWidth: true
        page: panel.page
    }

    SceneShaderCode {
        id: code

        page: panel.page
    }

    SceneEffectForm {
        id: describer

        page: panel.page
    }

    FileDialog {
        id: files

        fileMode: FileDialog.OpenFiles
        nameFilters: [App.tr("An effect descriptor, and its WGSL shader when it is not beside it") + " (*.json *.wgsl)"]
        onAccepted: {
            const paths = selectedFiles.map(file => App.path(file));
            const descriptor = paths.find(path => path.endsWith(".json"));
            const wgsl = paths.find(path => path.endsWith(".wgsl"));
            if (descriptor)
                panel.page.run("import_effect", wgsl ? {
                        "descriptor": descriptor,
                        "wgsl": wgsl
                    } : {
                        "descriptor": descriptor
                    });
        }
    }
}
