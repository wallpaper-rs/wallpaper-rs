import QtQuick
import io.github.wallpaper_rs.app

Fallo {
    id: trouble

    required property var catalog
    required property Installer installer
    readonly property var failure: installer.failure
    readonly property string kind: failure ? "signature" : catalog.troubled
    readonly property var repo: catalog.picked && catalog.picked.repo ? catalog.picked.repo : null
    readonly property string name: repo ? catalog.repoName(repo) : ""
    readonly property string host: repo ? (/^[a-z]+:\/\/([^/?#]+)/i.exec(repo.url) ?? ["", repo.url])[1] : ""
    readonly property string author: failure && failure.author ? failure.author : App.tr("its author")

    signal repositories

    big: true
    principal: true
    actionIcon: "arrow-clockwise"
    icon: kind === "signature" ? "seal-warning" : kind === "offline" ? "wifi-slash" : "file-x"
    title: kind === "signature" ? App.tr("The signature does not match") : kind === "offline" ? App.tr("No connection with “%1”", name) : App.tr("“%1” cannot be read", name)
    text: kind === "signature" ? App.tr("“%1” is not signed by %2, who published it, so nothing was installed and your library is the same. It may be a damaged download or a change made by someone who is not %2. Try again; if it happens again, tell whoever keeps “%3”.", failure.title, author, failure.repoName) : kind === "offline" ? App.tr("Could not reach %1. What you already installed stays in your library, and the bundled examples need no connection.", host) : App.tr("Its list of wallpapers arrived damaged or incomplete. What you already installed from it keeps working.")
    action: kind === "signature" ? App.tr("Download again") : App.tr("Try again")
    other: kind === "signature" ? App.tr("Not now") : kind === "offline" ? App.tr("See the examples") : App.tr("Repositories…")
    details: kind === "signature" ? failure.message : repo ? repo.broken || repo.last_error || "" : ""
    detailsOpen: kind === "signature"
    onActed: {
        if (kind === "signature")
            installer.install(failure.pkg, failure.version, failure.apply, button, false, false);
        else if (repo)
            App.act("Refresh", [repo.repo_id], "Repos1");
    }
    onOtherActed: {
        if (kind === "signature")
            installer.failure = null;
        else if (kind === "offline")
            catalog.word = Math.max(0, catalog.words.findIndex(w => w.repo && w.repo.bundled));
        else
            repositories();
    }
}
