import QtQuick
import QtCore as Core
import io.github.wallpaper_rs.app

Item {
    id: work

    required property var publish
    property string stage: ""
    property var job: null
    property bool registered: false
    property string lastPath: ""
    readonly property bool busy: stage !== "" || publish.where.repo.shelf.stage !== ""
    readonly property string doing: stage === "build" ? App.tr("Making the package…") : stage === "upload" ? App.tr("Uploading…") : stage === "register" ? App.tr("Registering your signature…") : App.tr("Publishing…")
    readonly property var watched: job === null ? null : App.jobs.find(j => j.id === job) ?? null
    readonly property string host: publish.where.host
    readonly property string spare: App.path(Core.StandardPaths.writableLocation(Core.StandardPaths.CacheLocation)) + "/publish"

    function run() {
        publish.fault = null;
        if (!publish.editing)
            return deliver(publish.file);
        stage = "build";
        if (publish.page.editor.dirty)
            publish.page.editor.save();
        const out = publish.place === "file" ? publish.where.target : spare + "/" + Publishing.base(publish.where.suggested);
        publish.page.editor.build(out, publish.sign.secret, publish.sign.word);
    }

    function deliver(path) {
        if (publish.place === "file")
            return finish({
                "kind": "file",
                "path": path
            });
        if (publish.place === "repo") {
            stage = "";
            return publish.where.repo.shelf.begin(path);
        }
        upload(path);
    }

    function upload(path) {
        stage = "upload";
        job = null;
        lastPath = path;
        App.act("UploadAs", [path, publish.where.hidden ? "unlisted" : "public"], "Sync1", reply => work && (work.job = Array.isArray(reply) ? reply[0] : reply), error => work && work.refused(String(error.message ?? error)));
    }

    function check() {
        if (watched === null || stage !== "upload")
            return;
        if (watched.state === "failed")
            return refused((watched.error ?? {}).message ?? "");
        if (watched.state !== "done")
            return;
        const output = watched.output ?? {};
        if ((output.errors ?? []).length > 0)
            return refused(output.errors.join("\n"));
        const id = output.package ?? output.id ?? publish.manifest.id ?? "";
        finish({
            "kind": "server",
            "id": id,
            "state": output.state ?? "",
            "link": String(publish.account.url ?? "").replace(/\/$/, "") + "/packages/" + id
        });
    }

    function refused(message) {
        const kind = Publishing.upload(message);
        if (kind === "registro" && !registered && publish.sign.current !== null)
            return register(true);
        stage = "";
        fail({
            "kind": kind,
            "title": ({
                    "version": App.tr("Version %1 already exists at %2", publish.version, host),
                    "registro": App.tr("%1 does not know your signature “%2”", host, publish.sign.keyName),
                    "red": App.tr("There is no connection with %1", host)
                })[kind] ?? App.tr("Could not upload to %1", host),
            "text": ({
                    "version": App.tr("A published version is not replaced. Upload it with the next number."),
                    "registro": App.tr("It is registered once in your account and afterwards it is not needed."),
                    "red": App.tr("Nothing was lost. Try again, or save it as a file and upload it later.")
                })[kind] ?? App.tr("Nothing was lost. Try again, or save it as a file."),
            "details": message
        });
    }

    function register(retry) {
        registered = true;
        stage = "register";
        const text = App.readText("file://" + publish.sign.current["public"]);
        App.act("AddAuthorKey", [text], "Sync1", () => {
            if (!work)
                return;
            applicationWindow().showPassiveNotification(App.tr("Your signature “%1” is now registered in your account at %2.", publish.sign.keyName, host), "long");
            if (retry && work.lastPath !== "")
                work.upload(work.lastPath);
            else
                work.stage = "";
        }, error => work && work.refused(String(error.message ?? error)));
    }

    function fail(fault) {
        stage = "";
        publish.fault = fault;
    }

    function finish(result) {
        stage = "";
        job = null;
        registered = false;
        publish.memory.place = publish.place;
        publish.result = result;
        publish.at = 4;
        if (publish.editing)
            publish.page.notice = "";
        Qt.callLater(() => publish && publish.focusStep());
    }

    onWatchedChanged: check()

    Connections {
        target: work.publish.editing ? work.publish.page.editor : null

        function onFinished(message) {
            if (work.stage !== "build")
                return;
            Qt.callLater(() => work && (work.publish.page.notice = ""));
            work.deliver(String(message).split("\n")[0]);
        }

        function onErrorChanged() {
            const error = work.publish.page.editor.error;
            if (work.stage !== "build" || error === "")
                return;
            work.fail({
                "kind": "otro",
                "title": App.tr("Could not make “%1”", work.publish.name),
                "text": App.tr("Nothing was lost. Fix it in the editor and try again."),
                "details": error
            });
            Qt.callLater(() => work && (work.publish.page.editor.error = ""));
        }
    }
}
