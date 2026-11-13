import QtQuick
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: notice

    required property var filer
    property string store: "keyring"
    property bool found: true

    function load() {
        App.query("List", [], "Providers1", reply => {
            if (!notice)
                return;
            notice.store = reply.store ?? "keyring";
            notice.found = reply.keyring_found !== false;
        });
    }

    Component.onCompleted: load()
    spacing: Kirigami.Units.smallSpacing

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "providers" || topic === "account")
                notice.load();
        }
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        visible: notice.store === "file"
        type: Kirigami.MessageType.Warning
        text: App.tr("This computer keeps its secrets (the passwords and API keys of providers, and its access to the sync server) in a private file, not in the system keyring: whoever can read your files can use them.")
        actions: [
            Kirigami.Action {
                text: App.tr("Use the system keyring")
                icon.name: "lock"
                onTriggered: App.act("SetSecretStore", ["keyring"], "Providers1")
            }
        ]
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        visible: notice.store === "keyring" && !notice.found
        type: Kirigami.MessageType.Warning
        text: App.tr("No system keyring was found on this computer, so passwords and API keys cannot be stored. Start or unlock your keyring, or keep them in a private file instead: anyone who can read your files could then use them.")
        actions: [
            Kirigami.Action {
                text: App.tr("Keep secrets in a private file…")
                icon.name: "document-encrypt"
                onTriggered: notice.filer.open()
            }
        ]
    }
}
