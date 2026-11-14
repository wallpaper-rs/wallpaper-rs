import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

PanelChico {
    id: revoker

    property var device: ({})

    function ask(target, from) {
        device = target;
        opener = from;
        open();
    }

    title: App.tr("Remove %1 from the account?", revoker.device.label || revoker.device.name || "")
    text: App.tr("This device can no longer sync with your account, and the sync key is replaced so that it cannot read what you sync from now on. Do it for a device you lost or no longer use.")
    quiet: App.tr("Cancel")
    onClosed: password.text = ""

    QQC2.TextField {
        id: password

        width: revoker.width - 2 * Look.e4
        echoMode: TextInput.Password
        placeholderText: App.tr("Password of the account")
        Accessible.name: placeholderText
    }

    Nota {
        width: revoker.width - 2 * Look.e4
        text: App.tr("An account that signs in through single sign-on removes devices on the account page of the server.")
    }

    QQC2.Button {
        highlighted: true
        palette.highlight: Look.coral
        enabled: password.text.length > 0
        text: App.tr("Remove device")
        icon.name: "trash"
        onClicked: {
            App.passphrase("RevokeDevice", revoker.device.id + "\n" + password.text);
            revoker.close();
        }
    }
}
