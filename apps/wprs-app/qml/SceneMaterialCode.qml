import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: dialog

    required property var page
    property string name: ""
    property string saved: ""
    property var report: ({})
    readonly property bool changed: area.text !== dialog.saved
    readonly property int line: report.ok === false && report.line ? report.line : 0

    function ask(text) {
        return JSON.parse(page.editor.rig("", JSON.stringify({
                    "material_check": text === undefined ? {
                        "name": dialog.name
                    } : {
                        "name": dialog.name,
                        "text": text
                    }
                })) || "{}");
    }

    function load() {
        const found = dialog.ask();
        dialog.saved = found.source ?? "";
        area.text = dialog.saved;
        dialog.report = found;
    }

    function show(name) {
        dialog.name = name;
        dialog.load();
        dialog.open();
    }

    function save() {
        page.run("scene3d", {
                "material_code": {
                    "name": dialog.name,
                    "text": area.text
                }
            });
        if (page.editor.error === "")
            dialog.saved = area.text;
        dialog.report = dialog.ask();
    }

    function mark() {
        const lines = area.text.split("\n");
        if (dialog.line < 1 || dialog.line > lines.length)
            return Qt.rect(0, 0, 0, 0);
        const start = lines.slice(0, dialog.line - 1).reduce((sum, l) => sum + l.length + 1, 0);
        return area.positionToRectangle(start);
    }

    title: dialog.changed ? App.tr("%1 (not saved)", dialog.name) : dialog.name
    standardButtons: Kirigami.Dialog.Close
    preferredWidth: Kirigami.Units.gridUnit * 38
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
            onTriggered: Qt.openUrlExternally("file://" + dialog.page.editor.folder + "/effects/" + dialog.name + ".wgsl")
        }
    ]

    Timer {
        id: later

        interval: 400
        onTriggered: dialog.report = dialog.ask(area.text)
    }

    ColumnLayout {
        implicitWidth: Kirigami.Units.gridUnit * 36

        Kirigami.InlineMessage {
            Layout.fillWidth: true
            visible: true
            type: dialog.report.ok ? Kirigami.MessageType.Positive : Kirigami.MessageType.Error
            text: dialog.report.ok ? App.tr("%1 builds", dialog.name) : dialog.line > 0 ? App.tr("%1, line %2, column %3: %4", dialog.name, dialog.line, dialog.report.column ?? 1, dialog.report.error ?? "") : App.tr("%1: %2", dialog.name, dialog.report.error ?? "")
        }

        QQC2.ScrollView {
            Layout.fillWidth: true
            implicitHeight: Kirigami.Units.gridUnit * 20

            Area {
                id: area

                wrapMode: TextEdit.NoWrap
                font.family: "monospace"
                selectByMouse: true
                onTextChanged: {
                    if (dialog.opened)
                        later.restart();
                }
                Keys.onPressed: event => {
                    if (event.matches(StandardKey.Save)) {
                        dialog.save();
                        event.accepted = true;
                    }
                }

                Rectangle {
                    readonly property rect where: dialog.mark()

                    z: -1
                    visible: dialog.line > 0
                    x: 0
                    y: where.y
                    width: area.width
                    height: where.height
                    color: Kirigami.Theme.negativeBackgroundColor
                }
            }
        }
    }
}
