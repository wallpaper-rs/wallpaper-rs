import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Column {
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
    spacing: Look.e2

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "providers" || topic === "account")
                notice.load();
        }
    }

    Nota {
        visible: notice.store === "file"
        tone: "ambar"
        text: App.tr("This computer keeps its keys in a private file, not in the system keyring.")
    }

    QQC2.Button {
        visible: notice.store === "file"
        flat: true
        text: App.tr("Use the system keyring")
        icon.name: "key"
        onClicked: App.act("SetSecretStore", ["keyring"], "Providers1")
    }

    Nota {
        visible: notice.store === "keyring" && !notice.found
        tone: "ambar"
        text: App.tr("No system keyring was found, so keys cannot be stored.")
    }

    QQC2.Button {
        visible: notice.store === "keyring" && !notice.found
        flat: true
        text: App.tr("Keep keys in a private file…")
        onClicked: notice.filer.open()
    }
}
