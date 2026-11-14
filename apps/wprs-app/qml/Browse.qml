import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: catalog

    property var packages: []
    property int total
    property var repos: []
    property var installed: []
    property var previews: ({})
    property var shown: ({})
    property var votes: ({})
    property var versions: ({})
    property bool counted
    property string voteRepo
    property string query
    property string error
    property int word: 0
    property bool loaded
    readonly property bool battery: App.state.on_battery ?? false
    readonly property var updates: installed.filter((p, i) => (p.update || p.revoked) && installed.findIndex(q => q.id === p.id) === i)
    readonly property var plain: updates.filter(p => p.update && (p.asks ?? []).length === 0)
    readonly property var featured: packages.filter(p => p.featured)
    readonly property bool lone: repos.length <= 1
    readonly property bool fresh: lone && installed.length === 0
    readonly property var words: {
        const list = [];
        if (!lone && featured.length > 0)
            list.push({
                "text": App.tr("Featured"),
                "count": featured.length,
                "kind": "featured"
            });
        if (!lone)
            list.push({
                "text": App.tr("All"),
                "count": packages.length,
                "kind": "all"
            });
        repos.forEach(r => {
            const trouble = catalog.trouble(r);
            list.push({
                "text": catalog.repoName(r),
                "count": trouble === "offline" ? App.tr("offline") : trouble === "unreadable" ? App.tr("cannot be read") : packages.filter(p => p.repo_id === r.repo_id).length,
                "tone": trouble === "" ? "" : "coral",
                "kind": "repo",
                "repo": r
            });
        });
        if (updates.length > 0)
            list.push({
                "text": App.tr("Updates"),
                "count": updates.length,
                "kind": "updates"
            });
        return list;
    }
    readonly property var picked: words.length > 0 ? words[Math.max(0, Math.min(word, words.length - 1))] : null
    readonly property string troubled: picked && picked.kind === "repo" ? trouble(picked.repo) : ""
    readonly property var chosen: picked === null ? [] : picked.kind === "featured" ? featured : picked.kind === "repo" ? packages.filter(p => p.repo_id === picked.repo.repo_id) : picked.kind === "updates" ? updates.map(u => Object.assign({}, packages.find(p => p.repo_id === u.repo_id && p.id === u.id) ?? {
                    "repo_id": u.repo_id,
                    "id": u.id,
                    "title": u.id,
                    "kind": "",
                    "authors": [],
                    "size": u.size,
                    "installed": [u.version],
                    "versions": [u.version],
                    "asks": []
                }, {
                    "own": u
                })) : packages
    readonly property var covers: chosen.map(p => cover(p))

    function spec(pkg) {
        return pkg.repo_id + "/" + pkg.id + "@" + pkg.version;
    }

    function repoName(repo) {
        return repo.bundled ? App.tr("Examples") : repo.name || repo.repo_id.slice(0, 8);
    }

    function trouble(repo) {
        if (repo.broken)
            return "unreadable";
        if (!repo.last_error)
            return "";
        return /connect|resolve|dns|timed out|timeout|unreachable|refused|network|offline/i.test(repo.last_error) ? "offline" : "unreadable";
    }

    function veiled(pkg) {
        return pkg.content === "mature" && !pkg.revealed && !shown[pkg.repo_id + "/" + pkg.id] && !App.unveiled;
    }

    function cover(pkg) {
        const note = notes.of(pkg);
        return {
            "picture": veiled(pkg) ? "" : App.url(previews[spec(pkg)] ?? ""),
            "name": pkg.title,
            "caption": notes.caption(pkg),
            "note": note[0],
            "noteIcon": note[1],
            "noteTone": note[2],
            "pkg": pkg
        };
    }

    function reveal(pkg) {
        const key = pkg.repo_id + "/" + pkg.id;
        shown = Object.assign({}, shown, {
            [key]: true
        });
        App.act("Reveal", [key], "Repos1");
    }

    function load() {
        App.query("List", [], "Repos1", reply => catalog && (catalog.repos = reply.repos, catalog.error = ""), failed => catalog && (catalog.error = failed.message));
        App.query("ListInstalled", [], "Repos1", reply => catalog && (catalog.installed = reply.packages));
        search();
    }

    function search() {
        App.query("Browse", [query, 0, 200], "Repos1", reply => {
            if (!catalog)
                return;
            catalog.packages = reply.packages;
            catalog.total = reply.total;
            catalog.loaded = true;
            App.query("GetVotes", [reply.packages.map(p => p.repo_id + "/" + p.id)], "Sync1", found => catalog && (catalog.votes = found.items ?? {}, catalog.counted = found.shown === true, catalog.voteRepo = found.repo_id ?? ""), () => catalog && (catalog.counted = false, catalog.voteRepo = ""));
            App.query("RequestPreviews", [reply.packages.filter(p => p.preview).map(catalog.spec)], "Repos1", found => catalog && (catalog.previews = Object.assign({}, catalog.previews, found[0])));
        });
    }

    onQueryChanged: search()
    Component.onCompleted: load()

    BrowseNotes {
        id: notes
    }

    Connections {
        target: App.backend

        function onPreviewReady(spec, path) {
            if (!spec.startsWith("animated:"))
                catalog.previews = Object.assign({}, catalog.previews, {
                    [spec]: path
                });
        }

        function onChanged(topic) {
            if (topic === "repos")
                catalog.load();
        }
    }
}
