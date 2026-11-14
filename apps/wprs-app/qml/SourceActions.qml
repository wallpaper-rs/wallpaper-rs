import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: acts

    required property var host

    function apply(next) {
        const was = host.seen;
        const marks = {};
        next.forEach(p => {
            marks[p.id] = p.items;
            if (was[p.id] === 0 && p.items > 0)
                applicationWindow().showPassiveNotification(App.tr("%1 brought %2 photos to your library.", SourceKinds.of(p.kind).name, p.items));
        });
        const offline = next.some(p => p.state === "error") && !host.providers.some(p => p.state === "error");
        if (offline)
            applicationWindow().showPassiveNotification(App.tr("No connection. Your sources cannot bring new photos. What you already have stays in your library."), 8000, App.tr("Try again"), () => App.act("Sync", [""], "Providers1"));
        host.seen = marks;
        host.providers = next;
    }

    function rotate(provider) {
        const query = "source:" + provider.source;
        const start = uid => App.act("Start", [uid, ""], "Playlists1", () => host.load());
        const have = host.playlists.find(l => (l.query || "").includes(query));
        if (have)
            return start(have.uid);
        App.act("Create", [SourceKinds.heading(provider), query], "Playlists1", reply => host && start(reply[0]));
    }

    function start(chosenKind) {
        host.kind = chosenKind;
        if (SourceKinds.of(chosenKind).quick)
            return App.addProvider(SourceKinds.fresh(chosenKind, host.providers.map(p => p.id)), chosenKind, {}, "", () => host && host.open("list"), e => applicationWindow().showPassiveNotification(e.message));
        host.open("add");
    }
}
