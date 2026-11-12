import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

QtObject {
    id: tools

    required property var page
    required property var dialogs
    required property var codes
    readonly property list<QtObject> list: [
        Kirigami.Action {
            text: App.tr("Add server…")
            icon.name: "list-add"
            visible: !page.account
            onTriggered: dialogs.adder.open()
        },
        Kirigami.Action {
            text: App.tr("Link this computer…")
            icon.name: "link"
            visible: page.phase === "unlinked"
            onTriggered: dialogs.linker.open()
        },
        Kirigami.Action {
            text: App.tr("Sync now")
            icon.name: "view-refresh"
            visible: page.keyed
            onTriggered: App.act("SyncNow", [], "Sync1")
        },
        Kirigami.Action {
            text: page.account?.paused ? App.tr("Resume sync") : App.tr("Pause sync")
            icon.name: page.account?.paused ? "media-playback-start" : "media-playback-pause"
            visible: page.keyed
            onTriggered: App.act("SetPaused", [!page.account.paused], "Sync1")
        },
        Kirigami.Action {
            text: App.tr("Approve a device…")
            icon.name: "dialog-ok-apply"
            visible: !!page.account?.device
            onTriggered: codes.approver.open()
        },
        Kirigami.Action {
            text: App.tr("Restore with the recovery passphrase…")
            icon.name: "document-decrypt"
            visible: page.phase === "needs_key" && page.account?.encrypted === true
            onTriggered: dialogs.restorer.open()
        },
        Kirigami.Action {
            text: App.tr("Replace the sync key")
            icon.name: "view-refresh"
            visible: page.keyed && page.account?.encrypted === true
            onTriggered: App.act("Rotate", [], "Sync1")
        },
        Kirigami.Action {
            text: App.tr("Reset synced data…")
            icon.name: "edit-clear-all"
            visible: !!page.account?.device
            onTriggered: dialogs.resetter.open()
        },
        Kirigami.Action {
            text: App.tr("Remove server…")
            icon.name: "edit-delete"
            visible: !!page.account
            onTriggered: dialogs.remover.open()
        }
    ]
}
