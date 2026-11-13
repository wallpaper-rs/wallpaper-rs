import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: panel

    required property var page
    readonly property var problems: page.view.problems ?? []
    readonly property string output: page.editor.output
    property var scriptLines: App.state.previews?.[panel.output]?.console ?? []
    property string scriptError: App.state.previews?.[panel.output]?.error ?? ""
    readonly property real wanted: Kirigami.Units.gridUnit * (scriptError !== "" || scriptLines.length > 0 ? 16 : problems.length ? 7 : 2)
    Layout.preferredHeight: wanted
    property var inputs: ({})
    readonly property var uses: (inputs.shown ?? {})[panel.output] ?? {}
    readonly property var off: panel.unmet(panel.uses)

    function unmet(used) {
        const granted = used.granted ?? [];
        const wanted = ["audio", "pointer", "clicks", "network", "rgb", "media"].filter(i => used[i] === true);
        if (used.media && used.network)
            wanted.push("media_network");
        if (used.audio && used.network)
            wanted.push("audio_network");
        return wanted.filter(i => !granted.includes(i));
    }

    function load() {
        App.query("GetInputs", [], "Inputs1", reply => panel && (panel.inputs = reply));
    }

    Component.onCompleted: load()

    Connections {
        target: App

        function onUpdated(topic) {
            if (topic === "inputs")
                reload.restart();
        }
    }

    Timer {
        id: reload

        interval: 300
        onTriggered: panel.load()
    }

    Kirigami.Heading {
        level: 4
        text: panel.problems.length ? App.tr("Problems (%1)", panel.problems.length) : panel.scriptError !== "" ? App.tr("Script error") : App.tr("No problems")
    }

    QQC2.ScrollView {
        Layout.fillWidth: true
        Layout.fillHeight: true
        visible: panel.problems.length > 0

        ListView {
            model: panel.problems
            clip: true

            delegate: QQC2.ItemDelegate {
                required property var modelData

                width: ListView.view.width
                text: modelData.message
                icon.name: modelData.warning ? "data-warning" : "data-error"
                onClicked: panel.page.jump(modelData.pointer)

                QQC2.ToolButton {
                    readonly property string fixing: Object.keys(modelData.fix ?? {})[0] ?? ""

                    anchors.right: parent.right
                    visible: fixing !== ""
                    icon.name: "dialog-ok-apply"
                    text: App.tr("Fix")
                    onClicked: panel.page.run(fixing, modelData.fix[fixing])
                }
            }
        }
    }

    QQC2.ScrollView {
        Layout.fillWidth: true
        Layout.preferredHeight: Kirigami.Units.gridUnit * 5
        visible: panel.scriptError !== ""

        Area {
            objectName: "scriptError"
            readOnly: true
            wrapMode: TextEdit.Wrap
            text: panel.scriptError
            color: Look.coral
        }
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        visible: panel.off.length > 0
        type: Kirigami.MessageType.Warning
        text: App.tr("A script wants %1 but it is not granted here, so that part of the scene is off while you try it.", panel.off.join(", "))
    }

    Kirigami.Heading {
        level: 4
        visible: panel.scriptLines.length > 0
        text: App.tr("Script console")
    }

    QQC2.ScrollView {
        Layout.fillWidth: true
        Layout.preferredHeight: Kirigami.Units.gridUnit * 6
        visible: panel.scriptLines.length > 0

        ListView {
            model: panel.scriptLines
            clip: true

            delegate: QQC2.Label {
                required property string modelData

                width: ListView.view.width
                text: modelData
                textFormat: Text.PlainText
                font.family: "monospace"
                elide: Text.ElideRight
                color: /^error\b/.test(modelData) ? Look.coral : Kirigami.Theme.textColor
            }
        }
    }
}
