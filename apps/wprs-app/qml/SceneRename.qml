import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.PromptDialog {
    id: dialog

    required property var page
    property string kind: "layer"
    property string current
    property var lines: []
    property alias text: field.text

    function ask(kind, id) {
        dialog.kind = kind;
        dialog.current = id;
        lines = JSON.parse(page.editor.scripts(id) || "[]");
        field.text = id;
        open();
        field.forceActiveFocus();
    }

    function rename() {
        if (field.text === current || field.text === "")
            return;
        page.run("rename", {
            "kind": kind,
            "id": current,
            "to": field.text
        });
        if (page.editor.error === "")
            close();
    }

    title: App.tr("Rename %1", current)
    standardButtons: QQC2.Dialog.Ok | QQC2.Dialog.Cancel
    onAccepted: rename()

    ColumnLayout {
        QQC2.TextField {
            id: field

            Layout.fillWidth: true
            validator: RegularExpressionValidator {
                regularExpression: /[a-z0-9_]{1,32}/
            }
            onAccepted: dialog.rename()
        }

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            text: App.tr("Bindings, parents, masks, effects, triggers and timelines that name it follow the new id.")
        }

        QQC2.Label {
            Layout.fillWidth: true
            visible: dialog.lines.length > 0
            wrapMode: Text.Wrap
            color: Kirigami.Theme.neutralTextColor
            text: App.tr("These lines of scripts name %1 and are not rewritten: %2", dialog.current, dialog.lines.join(", "))
        }
    }
}
