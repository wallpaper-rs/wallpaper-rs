import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.InlineMessage {
    id: notice

    required property var host
    property string phase

    function load() {
        App.query("GetStatus", [], "Sync1", reply => notice && (notice.phase = reply.account?.state ?? ""));
    }

    visible: phase === "signed_out"
    type: Kirigami.MessageType.Warning
    text: App.tr("This computer can no longer sync: the server no longer accepts it, because it was removed from the account or the account was deleted.")
    Component.onCompleted: load()
    actions: [
        Kirigami.Action {
            text: App.tr("Open Account")
            icon.name: "folder-cloud"
            onTriggered: notice.host.activate("account")
        }
    ]

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "account")
                notice.load();
        }
    }
}
