import QtQuick
import io.github.wallpaper_rs.app

Timer {
    id: check

    required property var host
    property var publish: null
    property var sample: null
    property int ticks: 0
    readonly property Stall probe: Stall {}
    readonly property Signing keys: Signing {}

    function start() {
        host.waiting++;
        host.held = host.held.concat([check]);
        sample = JSON.parse(probe.sample("shelf"));
        publish = Qt.createComponent("Publish.qml").createObject(host.contentItem, {
            "width": 900,
            "height": 600
        });
        publish.openWith(sample.wpak);
        publish.where.pick("repo");
        const repo = publish.where.repo;
        repo.making = true;
        repo.naming.text = "Check shelf";
        repo.place.text = sample.repo;
        publish.where.featured = true;
        publish.where.note = "Calm mornings";
        if (!publish.where.can)
            return finish("a new repository with a name cannot be published to");
        publish.where.go();
        running = true;
    }

    function finish(problem) {
        running = false;
        host.waiting--;
        publish.destroy();
        host.held = host.held.filter(c => c !== check);
        if (problem)
            throw new Error("publishing to a repository: " + problem);
    }

    function verify() {
        const shown = JSON.parse(probe.open(publish.where.repo.target));
        const found = (shown.packages ?? [])[0] ?? {};
        if ((shown.root ?? {}).complete !== true)
            return "the root of the new repository is not signed";
        if (found.id !== "org.example.check" || found.featured !== true || found.note !== "Calm mornings")
            return "the package is not added and featured with its note: " + JSON.stringify(found);
        if (String(publish.result.expires ?? "") === "")
            return "the index was not published";
        const named = publish.where.repo.shelf.named;
        if (named === "" || JSON.parse(keys.keys(keys.folder())).some(k => k.name.startsWith(named)))
            return "a key of the repository is offered as an author signature";
        if (JSON.parse(keys.keys(keys.shelves())).filter(k => k.name.startsWith(named)).length !== 2)
            return "the root and online keys of the repository were not kept";
        return publish.result.kind === "repo" ? "" : "the page did not say where it was published";
    }

    interval: 20
    repeat: true
    onTriggered: {
        if (publish.fault !== null)
            return finish("it failed: " + publish.fault.title + " " + (publish.fault.details ?? ""));
        if (publish.result !== null)
            return finish(verify());
        if (++ticks > 1500)
            finish("it did not finish");
    }
}
