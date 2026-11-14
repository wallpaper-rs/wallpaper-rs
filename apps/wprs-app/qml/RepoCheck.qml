import QtQuick
import io.github.wallpaper_rs.app

Timer {
    id: check

    required property var host
    property var stall: null
    property var sample: null
    property int step: 0
    readonly property string pkg: "org.example.check"
    readonly property var steps: [[() => {
                sample = JSON.parse(stall.backend.sample("stall"));
                const charter = stall.admin.charter;
                charter.start(null);
                charter.roots.keys = [sample.root_pub];
                charter.onlines.keys = [sample.online_pub];
                if (charter.ready)
                    throw new Error("static repository: a new repository could be made without a folder");
                charter.folder = sample.repo;
                charter.named = "Check";
                if (!charter.ready)
                    throw new Error("static repository: a chosen folder did not enable the new repository");
                charter.accepted();
            }, () => stall.shown?.root?.version === 1 && stall.shown.root.complete === false], [() => {
                const rooting = stall.admin.rooting;
                rooting.open();
                rooting.key = sample.root_key;
                rooting.sign(rooting.key, rooting.secret);
            }, () => stall.shown?.root?.complete === true], [() => {
                stall.admin.rooting.close();
                stall.backend.add(stall.folder, JSON.stringify([sample.wpak]), false);
            }, () => (stall.shown?.packages ?? []).length === 1], [() => {
                stall.revoke(pkg, "1.0.0");
                stall.admin.revoking.accepted();
            }, () => stall.shown.packages[0].versions[0].revoked === true], [() => stall.unrevoke(pkg, "1.0.0"), () => stall.shown.packages[0].versions[0].revoked === false], [() => stall.list(pkg, "", false), () => stall.shown.packages[0].versions[0].listed === false], [() => stall.list(pkg, "", true), () => stall.shown.packages[0].versions[0].listed === true], [() => stall.feature(pkg, "Calm mornings", true), () => stall.shown.packages[0].featured === true && stall.shown.packages[0].note === "Calm mornings"], [() => stall.feature(pkg, "", false), () => stall.shown.packages[0].featured === false], [() => {
                const publishing = stall.publishing;
                publishing.open();
                publishing.key = sample.online_key;
                publishing.secret = "check";
                publishing.accepted();
            }, () => stall.shown?.published?.packages === 1], [() => stall.backend.revokeKey(stall.folder, "ab".repeat(32), "other"), () => (stall.shown?.revoked_keys ?? []).includes("ab".repeat(32))]]

    function start() {
        host.waiting++;
        stall = Qt.createComponent("RepoStatic.qml").createObject(host.contentItem, {
                "width": 900,
                "height": 600
            });
        host.held = host.held.concat([check]);
        steps[0][0]();
        running = true;
    }

    function finish(problem) {
        running = false;
        host.waiting--;
        stall.destroy();
        host.held = host.held.filter(c => c !== check);
        if (problem)
            throw new Error("static repository: " + problem);
    }

    interval: 20
    repeat: true
    onTriggered: {
        if (stall.failed)
            return finish("step " + step + " failed: " + stall.message);
        if (!steps[step][1]())
            return;
        if (++step < steps.length)
            return steps[step][0]();
        running = false;
        stall.grabToImage(shot => {
                if (shot.image === undefined || stall.width <= 0)
                    finish("the page did not draw");
                else
                    finish("");
            });
    }
}
