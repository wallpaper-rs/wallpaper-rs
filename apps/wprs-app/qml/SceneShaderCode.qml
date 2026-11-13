import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: dialog

    required property var page
    property string effect: ""
    property string saved: ""
    property string descriptor: ""
    property string path: ""
    property string stage: "layer"
    property var report: ({})
    readonly property bool changed: area.text !== dialog.saved
    readonly property int line: report.line ?? 0
    readonly property string layer: page.editor.selected.startsWith("/scene/layers") ? (JSON.parse(page.editor.value(page.editor.selected + "/id") || "null") ?? "") : ""

    function load() {
        const source = JSON.parse(page.editor.rig("", JSON.stringify({
                        "shader_source": {
                            "effect": dialog.effect
                        }
                    })) || "null") ?? {};
        dialog.saved = source.wgsl ?? "";
        dialog.descriptor = source.descriptor ?? "";
        dialog.path = source.wgsl_path ?? "";
        dialog.stage = source.stage ?? "layer";
        area.text = dialog.saved;
        checker.check(dialog.effect, dialog.descriptor, area.text);
    }

    function show(effect) {
        dialog.effect = effect;
        dialog.report = {};
        dialog.load();
        dialog.open();
    }

    function write(gesture) {
        page.run("shader", {
                "write": {
                    "effect": dialog.effect,
                    "wgsl": area.text
                }
            }, gesture);
        if (page.editor.error === "")
            dialog.saved = area.text;
    }

    function start(line) {
        const lines = area.text.split("\n");
        let at = 0;
        for (let i = 0; i < Math.min(line - 1, lines.length); i++)
            at += lines[i].length + 1;
        return at;
    }

    function reported(text) {
        dialog.report = JSON.parse(text || "{}");
        if (dialog.report.ok && live.checked && dialog.changed)
            dialog.write(true);
    }

    function jump() {
        area.cursorPosition = dialog.start(dialog.line) + Math.max(0, (dialog.report.column ?? 1) - 1);
        area.forceActiveFocus();
    }

    title: dialog.changed ? App.tr("%1 (not saved)", dialog.effect) : dialog.effect
    standardButtons: Kirigami.Dialog.Close
    preferredWidth: Kirigami.Units.gridUnit * 40
    customFooterActions: [
        Kirigami.Action {
            text: App.tr("Save")
            icon.name: "document-save"
            enabled: dialog.changed
            onTriggered: dialog.write(false)
        },
        Kirigami.Action {
            text: App.tr("Use on the selected layer")
            icon.name: "list-add"
            visible: dialog.stage === "layer"
            enabled: dialog.layer !== ""
            onTriggered: dialog.page.run("effects", {
                    "add": {
                        "layer": dialog.layer,
                        "effect": dialog.effect
                    }
                })
        }
    ]

    ShaderCheck {
        id: checker

        onReportChanged: dialog.reported(report)
    }

    Timer {
        id: settle

        interval: 500
        onTriggered: checker.check(dialog.effect, dialog.descriptor, area.text)
    }

    ColumnLayout {
        implicitWidth: Kirigami.Units.gridUnit * 38

        Kirigami.InlineMessage {
            Layout.fillWidth: true
            visible: true
            type: checker.checking ? Kirigami.MessageType.Information : dialog.report.ok ? Kirigami.MessageType.Positive : Kirigami.MessageType.Error
            text: checker.checking ? App.tr("Checking…") : dialog.report.ok ? App.tr("Builds; costs %1 units", Math.round((dialog.report.units ?? 0) * 100) / 100) : dialog.line > 0 ? App.tr("Line %1, column %2: %3", dialog.line, dialog.report.column ?? 1, dialog.report.error ?? "") : (dialog.report.error ?? "")
            actions: [
                Kirigami.Action {
                    text: App.tr("Go to the line")
                    icon.name: "go-jump"
                    visible: dialog.line > 0
                    onTriggered: dialog.jump()
                }
            ]
        }

        RowLayout {
            Layout.fillWidth: true

            QQC2.CheckBox {
                id: live

                Layout.fillWidth: true
                text: App.tr("Preview while typing: write each version that builds")
            }

            QQC2.Button {
                flat: true
                icon.name: "document-open"
                text: App.tr("Open in your editor…")
                enabled: dialog.path !== ""
                onClicked: Qt.openUrlExternally("file://" + dialog.page.editor.folder + "/" + dialog.path)
            }
        }

        QQC2.ScrollView {
            Layout.fillWidth: true
            implicitHeight: Kirigami.Units.gridUnit * 22

            Area {
                id: area

                wrapMode: TextEdit.NoWrap
                font.family: "monospace"
                selectByMouse: true
                onTextChanged: settle.restart()
                Keys.onPressed: event => {
                    if (event.matches(StandardKey.Save)) {
                        dialog.write(false);
                        event.accepted = true;
                    }
                }

                Rectangle {
                    readonly property rect at: area.positionToRectangle(dialog.start(dialog.line))

                    visible: dialog.line > 0 && !dialog.report.ok
                    x: 0
                    y: at.y
                    width: area.width
                    height: at.height
                    color: Look.coral
                    opacity: 0.5
                    z: -1
                }
            }
        }
    }
}
