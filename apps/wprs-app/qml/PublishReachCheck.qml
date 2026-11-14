import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Timer {
    id: check

    required property var host
    property var publish: null
    property var sample: null
    property int step: 0
    property int ticks: 0
    readonly property Stall probe: Stall {}
    readonly property var heights: [900, Kirigami.Units.gridUnit * 24]
    readonly property PublishBench bench: PublishBench {}
    readonly property string problem: '{"level": "error", "pointer": "/objects", "message": "The image this object draws is missing from the package, so it cannot be published; add it again or remove the object."}'
    readonly property var cases: ["review", "review error", "server", "repo", "server red", "repo otro"]

    function start() {
        host.waiting++;
        host.held = host.held.concat([check]);
        sample = JSON.parse(probe.sample("upload"));
        open();
        running = true;
    }

    function open() {
        if (publish)
            publish.destroy();
        const parts = cases[step % cases.length].split(" ");
        bench.findings = parts.length > 1 && parts[0] === "review" ? JSON.parse("[" + Array(6).fill(problem).join(", ") + "]") : [];
        publish = Qt.createComponent("Publish.qml").createObject(host.contentItem, {
            "width": 1000,
            "height": heights[Math.floor(step / cases.length)],
            "page": parts[0] === "review" ? bench : null
        });
        if (parts[0] === "review")
            publish.open();
        else
            publish.openWith(sample.wpak);
        publish.account = {
            "state": "ready",
            "name": "Check server",
            "url": "http://127.0.0.1:9/"
        };
        publish.sign.chosen = sample.root_key;
    }

    function act() {
        const parts = cases[step % cases.length].split(" ");
        publish.revealing.flick.contentY = 0;
        if (parts[0] === "review")
            return publish.revealing.restart();
        publish.where.pick(parts[0]);
        if (parts.length > 1)
            publish.fault = {
                "kind": parts[1],
                "title": "Check",
                "text": "Nothing was lost.",
                "details": ""
            };
    }

    function verify() {
        const item = publish.revealing.target();
        if (item === null || !item.visible)
            return "its main action is not shown";
        return publish.revealing.seen(item) ? "" : "its main action is out of view";
    }

    function finish(problem) {
        running = false;
        host.waiting--;
        publish.destroy();
        host.held = host.held.filter(c => c !== check);
        if (problem)
            throw new Error("the main action of the place to publish: " + problem);
    }

    interval: 20
    repeat: true
    onTriggered: {
        ticks++;
        if (ticks === 2)
            return act();
        if (ticks < 15)
            return;
        const problem = verify();
        if (problem !== "")
            return finish(cases[step % cases.length] + " at " + publish.height + " pixels: " + problem);
        ticks = 0;
        if (++step === cases.length * heights.length)
            return finish("");
        open();
    }
}
