import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

PanelChico {
    id: revoking

    property string subject: ""
    property bool noted: true
    readonly property var reasons: ["malware", "license", "takedown", "broken", "other"]

    signal revoke(string reason, string message)

    function ask(text, withMessage) {
        subject = text;
        noted = withMessage;
        message.text = "";
        open();
    }

    title: App.tr("Revoke")
    quiet: App.tr("Cancel")
    principal: App.tr("OK")
    borra: true
    onAccepted: revoke(reasons[reason.currentIndex], noted ? message.text : "")

    Kirigami.FormLayout {
        width: revoking.width - 2 * Look.e4

        QQC2.Label {
            Layout.maximumWidth: revoking.width - 2 * Look.e4
            wrapMode: Text.Wrap
            text: revoking.subject
        }

        QQC2.ComboBox {
            id: reason

            Kirigami.FormData.label: App.tr("Reason:")
            model: [App.tr("Malware"), App.tr("License"), App.tr("Takedown"), App.tr("Broken"), App.tr("Other")]
        }

        QQC2.TextField {
            id: message

            Kirigami.FormData.label: App.tr("Message for users:")
            visible: revoking.noted
            maximumLength: 500
            placeholderText: App.tr("Optional")
        }
    }
}
