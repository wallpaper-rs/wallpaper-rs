import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: installer

    property var watching: []
    property var failure: null
    property var said: []
    property var asking: null

    signal finished(var entry, bool applied)

    function running(pkg) {
        const entry = watching.find(w => w.spec.startsWith(pkg.repo_id + "/" + pkg.id + "@") || w.spec === pkg.repo_id + "/" + pkg.id);
        return entry ? (App.jobs.find(j => j.id === entry.job) ?? {
                "done": 0,
                "total": 0
            }) : null;
    }

    function signed(message) {
        return /signature|signed|hash|digest|sha256/i.test(message ?? "");
    }

    function announce(jobs) {
        const ended = watching.filter(w => jobs.some(j => j.id === w.job && ["done", "failed", "cancelled"].includes(j.state)));
        if (ended.length === 0)
            return;
        watching = watching.filter(w => !ended.includes(w));
        ended.forEach(w => {
            const job = jobs.find(j => j.id === w.job);
            if (job.state === "done")
                return w.apply ? installer.apply(w) : installer.tell(App.tr("“%1” is installed.", w.title), w, false);
            if (job.state === "failed" && installer.signed(job.error ? job.error.message : ""))
                installer.failure = Object.assign({}, w, {
                    "message": job.error.message
                });
            else if (job.state === "failed")
                installer.tell(job.error ? job.error.message : App.tr("Could not install “%1”.", w.title), w, false);
        });
    }

    function tell(text, entry, applied) {
        said = said.concat([text]);
        if (applied)
            applicationWindow().showPassiveNotification(text, "long", App.tr("Go to your library"), () => applicationWindow().showPage(0));
        else
            applicationWindow().showPassiveNotification(text, "short");
        finished(entry, applied);
    }

    function apply(entry) {
        App.query("ListInstalled", [], "Repos1", reply => {
            if (!installer)
                return;
            const found = reply.packages.filter(p => p.repo_id === entry.repo && p.id === entry.id && p.item);
            if (found.length === 0)
                return installer.tell(App.tr("“%1” is installed.", entry.title), entry, false);
            App.act("Apply", [found[found.length - 1].item, "", "", ""], "Library1", () => installer && installer.tell(App.tr("“%1” is now your wallpaper.", entry.title), entry, true), failed => installer && installer.tell(failed.message, entry, false));
        });
    }

    function install(pkg, version, apply, opener, stale, downgrade) {
        const spec = pkg.repo_id + "/" + pkg.id + (version ? "@" + version : "");
        const entry = {
            "spec": spec,
            "repo": pkg.repo_id,
            "id": pkg.id,
            "title": pkg.title || pkg.id,
            "author": (pkg.authors ?? []).join(", "),
            "repoName": pkg.repo_name ?? "",
            "apply": apply === true,
            "pkg": pkg,
            "version": version
        };
        failure = null;
        App.act("Install", [spec, stale === true, downgrade === true], "Repos1", job => installer && (installer.watching = installer.watching.concat([Object.assign(entry, {
                        "job": job[0]
                    })]), installer.announce(App.jobs)), error => {
            if (!installer)
                return;
            if (error.name === App.stale)
                installer.ask(staleness, entry, opener, [stale, downgrade], true, downgrade);
            else if (error.name === App.invalidArgs && error.message.includes("downgrade"))
                installer.ask(older, entry, opener, [stale, downgrade], stale, true);
            else
                installer.tell(error.message, entry, false);
        });
    }

    function ask(panel, entry, opener, flags, stale, downgrade) {
        asking = {
            "entry": entry,
            "stale": stale === true,
            "downgrade": downgrade === true
        };
        panel.opener = opener ?? null;
        panel.open();
    }

    function retry() {
        const a = asking;
        asking = null;
        if (a)
            install(a.entry.pkg, a.entry.version, a.entry.apply, null, a.stale, a.downgrade);
    }

    function update(pkg) {
        install(pkg, "", false, null, false, false);
    }

    function uninstall(id, version) {
        App.act("Uninstall", [id, version], "Repos1", () => {}, failed => installer && applicationWindow().showPassiveNotification(failed.message, "short"));
    }

    Connections {
        target: App

        function onJobsChanged() {
            installer.announce(App.jobs);
        }
    }

    PanelChico {
        id: staleness

        title: App.tr("The repository is out of date")
        text: App.tr("Its signed list expired, so it may miss recent fixes or withdrawals. Install anyway?")
        quiet: App.tr("Cancel")
        principal: App.tr("Install anyway")
        principalIcon: "download-simple"
        onAccepted: installer.retry()
        onRejected: installer.asking = null
    }

    PanelChico {
        id: older

        readonly property var entry: installer.asking ? installer.asking.entry : null

        title: App.tr("Install an older version?")
        text: entry ? App.tr("The repository offers “%1” %2 and you have %3. The older one may bring errors already fixed.", entry.title, entry.version, (entry.pkg.installed ?? []).slice(-1)[0] ?? "") : ""
        quiet: App.tr("Cancel")
        principal: App.tr("Install the older one")
        onAccepted: installer.retry()
        onRejected: installer.asking = null
    }
}
