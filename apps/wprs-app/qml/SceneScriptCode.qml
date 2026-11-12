import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: dialog

    required property var page
    property string path: ""
    property string saved: ""
    property var report: ({})
    readonly property string stamp: JSON.stringify((page.view.scripts ?? []).filter(s => s.path === dialog.path))
    readonly property bool changed: area.text !== dialog.saved

    function load() {
        dialog.saved = JSON.parse(page.editor.code("source", JSON.stringify({
                        "path": dialog.path
                    })) || "\"\"");
        area.text = dialog.saved;
        dialog.report = JSON.parse(page.editor.code("audit", JSON.stringify({
                        "path": dialog.path
                    })) || "{}");
    }

    function show(path) {
        dialog.path = path;
        dialog.load();
        dialog.open();
    }

    function save() {
        page.run("script", {
                "write": {
                    "path": dialog.path,
                    "text": area.text
                }
            });
        if (page.editor.error === "")
            dialog.load();
    }

    title: dialog.changed ? App.tr("%1 (not saved)", dialog.path) : dialog.path
    standardButtons: Kirigami.Dialog.Close
    preferredWidth: Kirigami.Units.gridUnit * 38
    onStampChanged: {
        if (dialog.opened && !dialog.changed)
            dialog.load();
        else if (dialog.opened)
            dialog.report = JSON.parse(page.editor.code("audit", JSON.stringify({
                            "path": dialog.path
                        })) || "{}");
    }
    customFooterActions: [
        Kirigami.Action {
            text: App.tr("Save")
            icon.name: "document-save"
            enabled: dialog.changed
            onTriggered: dialog.save()
        },
        Kirigami.Action {
            text: App.tr("Open in your editor…")
            icon.name: "document-open"
            onTriggered: Qt.openUrlExternally("file://" + dialog.page.editor.folder + "/" + dialog.path)
        },
        Kirigami.Action {
            text: App.tr("API reference…")
            icon.name: "help-contents"
            onTriggered: reference.open()
        }
    ]

    ColumnLayout {
        implicitWidth: Kirigami.Units.gridUnit * 36

        SceneScriptAudit {
            Layout.fillWidth: true
            page: dialog.page
            path: dialog.path
            report: dialog.report
        }

        SceneScriptRigMissing {
            Layout.fillWidth: true
            report: dialog.report
        }

        SceneScriptRigPick {
            Layout.fillWidth: true
            page: dialog.page
            onInserted: code => area.insert(area.cursorPosition, code + "\n")
        }

        QQC2.ScrollView {
            Layout.fillWidth: true
            implicitHeight: Kirigami.Units.gridUnit * 20

            Area {
                id: area

                wrapMode: TextEdit.NoWrap
                font.family: "monospace"
                selectByMouse: true
                Keys.onPressed: event => {
                    if (event.matches(StandardKey.Save)) {
                        dialog.save();
                        event.accepted = true;
                    }
                }
            }
        }
    }

    SceneScriptApi {
        id: reference

        page: dialog.page
    }
}
