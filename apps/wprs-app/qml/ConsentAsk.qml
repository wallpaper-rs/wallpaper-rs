import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.PromptDialog {
    id: asking

    required property var host
    property var asked: ({})
    readonly property Consent consent: Consent {
        visible: false
        daemon: App
        inputs: asking.host.inputs
        onAskChanged: {
            if (ask.scene === "")
                return asking.close();
            if (asking.asked[ask.scene])
                return;
            asking.asked[ask.scene] = true;
            asking.open();
        }
    }

    y: parent ? Math.round((parent.height - implicitHeight) / 2) : 0
    title: App.tr("Allow this scene?")
    subtitle: consent.text
    standardButtons: Kirigami.Dialog.NoButton
    customFooterActions: [
        Kirigami.Action {
            text: App.tr("Allow")
            icon.name: "dialog-ok-apply"
            onTriggered: {
                asking.consent.answer(true);
                asking.close();
            }
        },
        Kirigami.Action {
            text: App.tr("Not now")
            icon.name: "dialog-cancel"
            onTriggered: {
                asking.consent.answer(false);
                asking.close();
            }
        }
    ]
}
