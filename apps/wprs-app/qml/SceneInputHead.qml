import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

RowLayout {
    id: head

    required property var input
    property Component editor: null
    readonly property var field: input.field

    Layout.fillWidth: true
    Layout.minimumHeight: Look.altoCampo
    spacing: Look.e2

    QQC2.Label {
        Layout.fillWidth: head.editor === null
        Layout.preferredWidth: head.editor === null ? -1 : Math.min(implicitWidth, Look.e8 + Look.e5)
        text: (head.field.title || SceneWords.label(head.field.key, head.field.pointer)) + (head.field.unit ? " (" + head.field.unit + ")" : "")
        color: !enabled ? Look.brumaBaja : head.editor === null ? Look.escarcha : Look.bruma
        font.bold: head.field.widget === "group" || head.field.widget === "variant"
        elide: Text.ElideRight
        QQC2.ToolTip.text: head.field.help ?? ""
        QQC2.ToolTip.visible: hint.hovered && (head.field.help ?? "") !== ""

        HoverHandler {
            id: hint
        }
    }

    Loader {
        Layout.fillWidth: true
        active: head.editor !== null
        visible: active
        sourceComponent: head.editor
    }

    QQC2.ToolButton {
        visible: head.field.widget === "group" && !head.field.set
        icon.name: "list-add"
        text: App.tr("Add")
        onClicked: head.input.commit(head.field.sample)
    }

    QQC2.ToolButton {
        visible: head.field.set && !head.field.required && head.field.widget !== "computed"
        icon.name: "edit-reset"
        text: head.field.widget === "group" ? App.tr("Remove") : App.tr("Default")
        display: QQC2.AbstractButton.IconOnly
        QQC2.ToolTip.text: text
        QQC2.ToolTip.visible: hovered
        onClicked: head.input.page.run("remove", {
                "pointer": head.field.pointer
            })
    }

    SceneBind {
        visible: head.field.bindable
        page: head.input.page
        field: head.field
    }
}
