import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: filer

    required property var page

    title: App.tr("Keep the access in a private file")
    standardButtons: Kirigami.Dialog.Cancel
    customFooterActions: [
        Kirigami.Action {
            text: App.tr("Keep it in a private file")
            icon.name: "document-encrypt"
            onTriggered: {
                filer.close();
                App.act("SetFileSecrets", [true], "Sync1", () => filer && filer.page.link(filer.page.lastName, filer.page.lastPlain));
            }
        }
    ]

    ColumnLayout {
        QQC2.Label {
            Layout.fillWidth: true
            Layout.preferredWidth: Kirigami.Units.gridUnit * 26
            wrapMode: Text.Wrap
            text: App.tr("Without a keyring, this computer would keep the token that identifies it to your server, and the sync key, in a private file in its data folder. Only you can read that file on this computer, but anyone who gets a copy of your files, or who uses this account on this computer, could use your account and read what you sync. Choose this only on a computer where you cannot use a keyring.")
        }
    }
}
