import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: revoker

    property var device: ({})

    title: App.tr("Remove %1 from the account", revoker.device.label ?? revoker.device.name ?? "")
    standardButtons: Kirigami.Dialog.Cancel
    onClosed: password.text = ""
    customFooterActions: [
        Kirigami.Action {
            text: App.tr("Remove device")
            icon.name: "edit-delete"
            enabled: password.text.length > 0
            onTriggered: {
                App.passphrase("RevokeDevice", revoker.device.id + "\n" + password.text);
                revoker.close();
            }
        }
    ]

    ColumnLayout {
        QQC2.Label {
            Layout.fillWidth: true
            Layout.preferredWidth: Kirigami.Units.gridUnit * 26
            wrapMode: Text.Wrap
            text: App.tr("This device can no longer sync with your account, and the sync key is replaced so that it cannot read what you sync from now on. Do it for a computer you lost or no longer use.")
        }

        Kirigami.PasswordField {
            id: password

            Layout.fillWidth: true
            placeholderText: App.tr("Password of the account")
        }

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            text: App.tr("Removing a device asks for the password of the account. An account that signs in through single sign-on removes devices on the account page of the server.")
        }
    }
}
