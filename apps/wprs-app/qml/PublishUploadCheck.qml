import QtQuick
import io.github.wallpaper_rs.app

Timer {
    id: check

    required property var host
    property var publish: null
    property var sample: null
    property var calls: []
    property int ticks: 0
    readonly property Stall probe: Stall {}
    readonly property var account: ({
            "state": "ready",
            "name": "Check server",
            "url": "http://127.0.0.1:9/"
        })

    function serve(name, args, iface, done, failed) {
        calls = calls.concat([name]);
        if (name === "AddAuthorKey")
            return Qt.callLater(() => args[0].includes("untrusted comment") ? done() : failed({
                    "message": "not a public key"
                }));
        if (calls.filter(c => c === "UploadAs").length === 1)
            return Qt.callLater(() => failed({
                    "message": "the server refused the package: the manifest signature matches no author key of the repository"
                }));
        Qt.callLater(() => {
            done(["upload-2"]);
            publish.work.watched = {
                "state": "done",
                "output": {
                    "package": "org.example.check",
                    "state": "pending"
                }
            };
        });
    }

    function start() {
        host.waiting++;
        host.held = host.held.concat([check]);
        sample = JSON.parse(probe.sample("upload"));
        publish = Qt.createComponent("Publish.qml").createObject(host.contentItem, {
            "width": 900,
            "height": 600
        });
        publish.work.act = serve;
        publish.openWith(sample.wpak);
        publish.account = account;
        publish.sign.chosen = sample.root_key;
        publish.where.pick("server");
        if (!publish.where.can)
            return finish("a linked server cannot be uploaded to");
        publish.where.go();
        running = true;
    }

    function finish(problem) {
        running = false;
        host.waiting--;
        publish.destroy();
        host.held = host.held.filter(c => c !== check);
        if (problem)
            throw new Error("uploading to a server: " + problem);
    }

    function verify() {
        if (JSON.stringify(calls) !== JSON.stringify(["UploadAs", "AddAuthorKey", "UploadAs"]))
            return "the signature was not registered once and the upload retried: " + JSON.stringify(calls);
        if (publish.result.kind !== "server" || publish.result.id !== "org.example.check")
            return "the page did not say it was uploaded: " + JSON.stringify(publish.result);
        return publish.at === 4 ? "" : "the page did not reach its last part";
    }

    interval: 20
    repeat: true
    onTriggered: {
        publish.account = account;
        if (publish.fault !== null)
            return finish("it failed: " + publish.fault.title + " " + (publish.fault.details ?? ""));
        if (publish.result !== null)
            return finish(verify());
        if (++ticks > 1500)
            finish("it did not finish");
    }
}
