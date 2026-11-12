import QtQuick
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: problem

    required property var page
    readonly property var account: page.account
    readonly property string kind: page.failure !== "" ? page.failure : (account?.link_problem ?? "")
    readonly property string detail: page.failure !== "" ? page.failureDetail : (account?.link_error ?? "")
    readonly property bool signedOut: account?.state === "signed_out"
    property bool hidden

    function words(code) {
        switch (code) {
        case "keyring":
            return App.tr("This computer could not keep its access to the server because no system keyring is available. Start or unlock your keyring and link again. If this computer has none, you can keep the access in a private file instead: anyone who can read your files could then use it.");
        case "refused":
            return App.tr("The link was refused on the server. Link again when you are ready to approve it on your own computer.");
        case "expired":
            return App.tr("The code was not approved in time. Link again to get a new code.");
        case "encryption":
            return App.tr("This account syncs without end-to-end encryption, so this computer was not linked. Link again and tick the box to sync without encryption only if you accept that the operator of the server can read your synced data.");
        }
        return App.tr("This computer could not be linked. Check that the server can be reached and link again.");
    }

    onKindChanged: hidden = false
    spacing: Kirigami.Units.smallSpacing

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        visible: problem.kind !== "" && !problem.hidden
        type: Kirigami.MessageType.Error
        showCloseButton: true
        text: problem.words(problem.kind)
        onVisibleChanged: if (!visible)
            problem.hidden = true
        actions: [
            Kirigami.Action {
                text: App.tr("Link again…")
                icon.name: "link"
                onTriggered: problem.page.dialogs.linker.open()
            },
            Kirigami.Action {
                text: App.tr("Keep the access in a private file…")
                icon.name: "document-encrypt"
                visible: problem.kind === "keyring"
                onTriggered: problem.page.filer.open()
            }
        ]
    }

    SmallNote {
        Layout.fillWidth: true
        visible: problem.kind === "other" && problem.detail !== "" && !problem.hidden
        text: App.tr("Details: %1", problem.detail)
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        visible: problem.account?.file_secrets === true
        type: Kirigami.MessageType.Warning
        text: App.tr("This computer keeps its access to the server in a private file, not in the system keyring: whoever can read your files can use it. To use the keyring again, remove the server and add it again.")
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        visible: problem.signedOut
        type: Kirigami.MessageType.Error
        text: App.tr("The server no longer accepts this computer: it was removed from the account or the account was deleted. Remove the server here and link this computer again to keep syncing.")
        actions: [
            Kirigami.Action {
                text: App.tr("Remove server…")
                icon.name: "edit-delete"
                onTriggered: problem.page.dialogs.remover.open()
            }
        ]
    }
}
