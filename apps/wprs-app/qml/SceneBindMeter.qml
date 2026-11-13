import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: dialog

    required property var page
    property var source: ({
            "audio": "level"
        })
    readonly property int of: [16, 32, 64].includes(source.of) ? source.of : 64
    readonly property var reply: strip.reply
    property var uses: ({})
    property string asked: ""
    readonly property bool canAllow: uses.audio === true && (uses.granted ?? []).includes("audio") === false

    function look() {
        App.query("GetInputs", [], "Inputs1", reply => dialog && (dialog.uses = (reply.shown ?? {})[dialog.page.editor.output] ?? ({})), () => {});
    }

    function allow() {
        asked = "";
        App.act("GrantPackage", [uses.scene, uses.key, "audio"], "Inputs1", () => dialog && dialog.look(), error => {
            if (!dialog)
                return;
            dialog.asked = String(error.name).endsWith(".Unconfirmed") ? App.tr("Confirm it in the desktop notification.") : error.message;
            dialog.look();
        });
    }

    title: App.tr("Levels of the sound")
    standardButtons: Kirigami.Dialog.Close
    padding: Kirigami.Units.largeSpacing
    onOpened: look()

    ColumnLayout {
        spacing: Kirigami.Units.smallSpacing

        Kirigami.InlineMessage {
            Layout.fillWidth: true
            visible: dialog.reply.granted === false
            text: App.tr("The scene may not follow the sound yet. Allow the sound input of the package to see the levels.")
            actions: [
                Kirigami.Action {
                    text: App.tr("Allow the sound")
                    icon.name: "audio-input-microphone"
                    enabled: dialog.canAllow
                    onTriggered: dialog.allow()
                }
            ]
        }

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            visible: dialog.reply.granted === false && dialog.uses.audio !== true
            text: App.tr("The shown scene does not use sound yet: save it and show it on a screen after binding a value to the sound.")
        }

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            visible: dialog.asked !== ""
            text: dialog.asked
        }

        Kirigami.InlineMessage {
            Layout.fillWidth: true
            visible: dialog.reply.granted !== false && dialog.reply.listening === false
            text: App.tr("Waiting for sound. Play something and keep the preview running; the levels are only read while this window is open and nothing is recorded.")
        }

        SceneBandMeter {
            id: strip

            Layout.fillWidth: true
            Layout.preferredWidth: Kirigami.Units.gridUnit * 30
            Layout.preferredHeight: Kirigami.Units.gridUnit * 5
            page: dialog.page
            source: dialog.source
            of: dialog.of
            live: dialog.visible
        }

        QQC2.ProgressBar {
            Layout.fillWidth: true
            from: 0
            to: 1
            value: Math.min(1, Math.max(0, dialog.reply.value ?? 0))
        }

        QQC2.Label {
            Layout.fillWidth: true
            text: dialog.reply.listening ? App.tr("The binding gives %1 now (%2 before shaping)", Number(dialog.reply.value).toFixed(2), Number(dialog.reply.raw).toFixed(2)) : ""
            color: Kirigami.Theme.disabledTextColor
        }
    }
}
