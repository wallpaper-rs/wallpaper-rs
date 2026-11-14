import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: host

    property string uid
    property string preview
    property var details: ({})
    property string repo
    readonly property var item: details.item ?? {}
    readonly property var credit: details.credit ?? null
    readonly property var pkg: item.package ?? null
    readonly property bool flashing: !!pkg && pkg.photosensitive === true && !(pkg.confirmed ?? []).includes("photosensitive")
    readonly property string animated: flashing ? "" : App.url(details.about ? details.about.animated ?? "" : "")

    signal apply(string uid)
    signal closed

    function show(uid) {
        host.uid = uid;
        details = {};
        preview = "";
        repo = "";
        if (uid !== "")
            load(true);
    }

    function close() {
        show("");
        closed();
    }

    function load(first) {
        const uid = host.uid;
        App.query("GetItem", [uid], "Library1", reply => {
            if (!host || host.uid !== uid)
                return;
            host.details = reply;
            if (first)
                App.query("Thumbnail", [uid, 1024], "Library1", path => host && host.uid === uid && (host.preview = path[0] ?? ""), () => {});
            if (reply.installed)
                App.query("List", [], "Repos1", list => {
                    if (!host || host.uid !== uid)
                        return;
                    const found = list.repos.find(repo => repo.repo_id === reply.installed.repo_id);
                    host.repo = found ? found.name : "";
                });
        });
    }

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (host.uid !== "" && ["library", "playlists", "profiles", "repos"].includes(topic))
                host.load(false);
        }
    }
}
