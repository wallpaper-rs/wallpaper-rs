import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: filer

    signal chosen

    title: App.tr("Keep secrets in a private file")
    standardButtons: Kirigami.Dialog.Cancel
    customFooterActions: [
        Kirigami.Action {
            text: App.tr("Keep them in a private file")
            icon.name: "document-encrypt"
            onTriggered: {
                filer.close();
                App.act("SetSecretStore", ["file"], "Providers1", () => filer && filer.chosen());
            }
        }
    ]

    ColumnLayout {
        QQC2.Label {
            Layout.fillWidth: true
            Layout.preferredWidth: Kirigami.Units.gridUnit * 26
            wrapMode: Text.Wrap
            text: App.tr("Without a keyring, this computer would keep the passwords and API keys of your providers, and the token that identifies it to your server with the sync key, in a private file in its data folder. Only you can read that file on this computer, but anyone who gets a copy of your files, or who uses your account on this computer, could use them and read what you sync. Choose this only on a computer where you cannot use a keyring. You can move them back to the keyring at any time.")
        }
    }
}
