import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Item {
    id: secrets

    property var page: null
    readonly property alias escrower: escrower
    readonly property alias restorer: restorer
    readonly property alias unlocker: unlocker

    function show(panel, opener) {
        panel.opener = opener;
        panel.open();
    }

    PanelChico {
        id: escrower

        title: App.tr("Recovery passphrase")
        text: App.tr("The server will keep the sync key locked with this passphrase, so that a new device can get it when no device is left. Whoever gets a copy of the server's data can try to guess it without limit: use at least 20 characters, several unrelated words that you use nowhere else, and keep them somewhere safe.")
        quiet: App.tr("Cancel")
        onClosed: {
            first.text = "";
            second.text = "";
        }

        QQC2.TextField {
            id: first

            width: escrower.width - 2 * Look.e4
            echoMode: TextInput.Password
            placeholderText: App.tr("Passphrase")
            Accessible.name: placeholderText
        }

        QQC2.TextField {
            id: second

            width: escrower.width - 2 * Look.e4
            echoMode: TextInput.Password
            placeholderText: App.tr("Repeat the passphrase")
            Accessible.name: placeholderText
        }

        QQC2.Button {
            highlighted: true
            text: App.tr("Lock the key with it")
            icon.name: "lock"
            enabled: first.text.length >= 20 && first.text === second.text
            onClicked: {
                App.passphrase("SetEscrow", first.text);
                escrower.close();
            }
        }
    }

    PanelChico {
        id: restorer

        title: App.tr("Restore the sync key")
        text: App.tr("Use it only when none of your devices holds the key any more: give the recovery passphrase that you set on one of them.")
        quiet: App.tr("Cancel")
        onClosed: phrase.text = ""

        QQC2.TextField {
            id: phrase

            width: restorer.width - 2 * Look.e4
            echoMode: TextInput.Password
            placeholderText: App.tr("Passphrase")
            Accessible.name: placeholderText
        }

        QQC2.Button {
            highlighted: true
            text: App.tr("Restore")
            icon.name: "lock-open"
            enabled: phrase.text.length >= 20
            onClicked: {
                App.passphrase("Restore", phrase.text);
                restorer.close();
            }
        }
    }

    PanelChico {
        id: unlocker

        title: App.tr("Remove the recovery passphrase?")
        text: App.tr("Without it, only a device that holds the sync key can give it to a new one; if you lose every device, the synced data cannot be read any more.")
        quiet: App.tr("Cancel")
        principal: App.tr("Remove it")
        principalIcon: "trash"
        borra: true
        onAccepted: App.act("SetEscrow", [""], "Sync1")
    }
}
