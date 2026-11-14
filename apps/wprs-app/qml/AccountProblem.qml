import QtQuick
import io.github.wallpaper_rs.app

Column {
    id: problem

    property var page: null
    readonly property var account: page ? page.account : null
    readonly property string kind: page && page.failure !== "" ? page.failure : (account && account.link_problem ? account.link_problem : "")
    readonly property string detail: page && page.failure !== "" ? page.failureDetail : (account && account.link_error ? account.link_error : "")
    readonly property bool signedOut: !!account && account.state === "signed_out"
    readonly property bool failing: !!account && account.state === "error" && !!account.last_error
    readonly property alias link: link
    property bool hidden

    function words(code) {
        switch (code) {
        case "keyring":
            return App.tr("This computer could not keep its access to the server because no system keyring is available. Start or unlock your keyring and connect again. If this computer has none, you can keep it in a private file instead: anyone who can read your files could then use it.");
        case "refused":
            return App.tr("The connection was refused on the server. Connect again when you are ready to approve it on your own device.");
        case "expired":
            return App.tr("The code was not approved in time. Connect again to get a new code.");
        case "encryption":
            return App.tr("This account syncs without end-to-end encryption, so syncing was not turned on. Allow it under More only if you accept that the operator of the server can read your synced data.");
        }
        return App.tr("This computer could not be connected. Check that the server can be reached and connect again.");
    }

    onKindChanged: hidden = false
    spacing: Look.e4

    SecretNotice {
        width: problem.width
        filer: problem.page ? problem.page.filer : null
    }

    Fallo {
        id: link

        width: problem.width
        visible: problem.kind !== "" && !problem.hidden && !problem.signedOut
        title: App.tr("This computer is not connected")
        text: problem.words(problem.kind)
        details: problem.kind === "other" ? problem.detail : ""
        action: problem.kind === "keyring" ? App.tr("Keep secrets in a private file…") : App.tr("Connect again…")
        onActed: problem.kind === "keyring" ? problem.page.filer.open() : problem.page.panels.connect(link)
    }

    Fallo {
        id: lost

        width: problem.width
        visible: problem.signedOut
        title: App.tr("The server no longer accepts this computer")
        text: App.tr("It was removed from the account or the account was deleted. Remove the server here and connect this computer again to keep syncing.")
        action: App.tr("Remove server…")
        onActed: problem.page.panels.remove(lost)
    }

    Fallo {
        id: failed

        width: problem.width
        visible: problem.failing && !problem.signedOut
        title: App.tr("The last sync failed")
        text: App.tr("Your library stays as it is on this computer. Sync is tried again by itself; you can also try now.")
        details: problem.account && problem.account.last_error ? problem.account.last_error : ""
        action: App.tr("Try again")
        onActed: App.act("SyncNow", [], "Sync1")
    }
}
