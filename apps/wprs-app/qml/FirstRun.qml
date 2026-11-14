import QtQuick
import QtCore as Core
import io.github.wallpaper_rs.app

Item {
    id: run

    required property bool bare
    property var examples: []
    property var shelf: []
    property var pending: null
    property bool automatic: false
    property string phase
    property bool allowed: !App.checking
    readonly property bool nothing: Object.values(App.state.outputs ?? {}).every(o => !o.item)
    readonly property string marker: Core.StandardPaths.writableLocation(Core.StandardPaths.ConfigLocation).toString() + "/wallpaper-rs/first-run"
    property bool tried: false

    function load() {
        if (!bare)
            return;
        App.query("List", [], "Repos1", repos => {
            if (!run)
                return;
            const ids = repos.repos.filter(r => r.bundled).map(r => r.repo_id);
            App.query("Browse", ["", 0, 100], "Repos1", reply => {
                if (!run)
                    return;
                const fit = reply.packages.filter(p => ids.includes(p.repo_id) && p.installed.length === 0 && !p.flash_disabled && (p.asks ?? []).length === 0);
                run.shelf = fit.filter(p => p.kind === "scene").concat(fit.filter(p => p.kind !== "scene"));
                run.previews();
                run.begin();
            });
        });
    }

    property var previewed: ({})

    function previews() {
        App.query("RequestPreviews", [shelf.filter(p => p.preview).map(run.spec)], "Repos1", found => run && (run.previewed = Object.assign({}, run.previewed, found[0])));
    }

    function spec(pkg) {
        return pkg.repo_id + "/" + pkg.id + "@" + pkg.version;
    }

    function begin() {
        if (!bare || tried || App.backend.readText(marker) !== "" || phase !== "" || shelf.length === 0 || !nothing || !allowed)
            return;
        App.backend.writeText(marker, "1");
        tried = true;
        place(shelf[0], true);
    }

    function place(pkg, auto) {
        if (phase !== "")
            return;
        phase = "installing";
        pending = pkg;
        automatic = auto;
        App.act("Install", [spec(pkg), false, false], "Repos1", () => run && run.watch(), () => run && run.reset());
    }

    function reset() {
        phase = "";
        pending = null;
    }

    function watch() {
        if (phase !== "installing" || !pending)
            return;
        const pkg = pending;
        if (App.jobs.some(j => j.kind === "install" && j.state === "failed" && String(j.target).includes(pkg.id)))
            return reset();
        App.query("ListInstalled", [], "Repos1", reply => {
            const found = run ? reply.packages.find(p => p.id === pkg.id && p.item) : null;
            if (!found || !run || run.pending !== pkg || run.phase !== "installing")
                return;
            run.phase = "applying";
            App.act("Apply", [found.item, "", "", ""], "Library1", () => run && run.announce(pkg), () => run && run.reset());
        });
    }

    function announce(pkg) {
        const several = App.outputs.length > 1;
        const text = several ? App.tr("“%1” is now the wallpaper of every screen.", pkg.title) : App.tr("“%1” is now your wallpaper. You can change it whenever you like from here.", pkg.title);
        applicationWindow().showPassiveNotification({
            "text": text,
            "action": App.tr("Undo"),
            "undo": () => run && run.undo()
        });
        reset();
    }

    function undo() {
        App.act("ClearWallpaper", [""], "Daemon1");
    }

    onBareChanged: load()
    onNothingChanged: begin()
    Component.onCompleted: load()

    Connections {
        target: App.backend

        function onPreviewReady(spec, path) {
            if (path !== "")
                run.previewed = Object.assign({}, run.previewed, {
                    [spec]: path
                });
        }

        function onChanged(topic) {
            if (topic === "repos")
                run.load();
            if (["repos", "jobs", "library"].includes(topic))
                run.watch();
        }
    }
}
