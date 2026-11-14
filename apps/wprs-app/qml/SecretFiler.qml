import QtQuick
import io.github.wallpaper_rs.app

PanelChico {
    id: filer

    signal chosen

    title: App.tr("Keep secrets in a private file")
    text: App.tr("Without a keyring, this computer would keep the passwords and API keys of your providers, and the token that identifies it to your server with the sync key, in a private file in its data folder. Only you can read that file on this computer, but anyone who gets a copy of your files, or who uses your account on this computer, could use them and read what you sync. Choose this only on a computer where you cannot use a keyring. You can move them back to the keyring at any time.")
    principal: App.tr("Keep them in a private file")
    principalIcon: "lock"
    quiet: App.tr("Cancel")
    onAboutToShow: {
        if (!opener && parent) {
            x = (parent.width - width) / 2;
            y = Math.max(Look.e4, (parent.height - implicitHeight) / 2);
        }
    }
    onAccepted: {
        close();
        App.act("SetSecretStore", ["file"], "Providers1", () => filer && filer.chosen());
    }
    onRejected: close()
}
