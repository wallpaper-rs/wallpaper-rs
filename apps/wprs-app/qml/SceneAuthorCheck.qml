import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function expect(ok, what) {
        if (!ok)
            throw new Error("authoring: " + what);
    }

    function review() {
        const publish = Qt.createComponent("Publish.qml").createObject(page, {
                "page": page
            });
        publish.open();
        const review = publish.review;
        expect(publish.shown && publish.at === 1 && review.mode === "abierto", "publishing does not open on its first step");
        expect(Array.isArray(review.report.findings) && review.report.signature.state === "unsigned", "the review has no findings list or no signature state");
        expect(Object.keys(review.platforms).includes("desktop"), "the review does not say where the package plays");
        const signature = JSON.parse(page.editor.rig("", JSON.stringify({
            "signature": {
                "path": "/missing/package.wpak",
                "keys": [""]
            }
        })));
        expect(signature.state === "invalid", "a signature of a missing package was not refused");
        page.change("/manifest/photosensitive", false);
        page.change("/manifest/license", Publishing.reserved);
        review.refresh();
        expect(review.flashesAnswered && review.license === Publishing.reserved, "the answers of the first step are not kept in the manifest");
        expect(review.why() !== "" || review.ready, "the first step does not say why it cannot go on");
        publish.advance(2);
        expect(publish.sign.mode === "abierto" && review.mode === "hecho", "the second step does not open after the first");
        publish.sign.skip();
        expect(publish.at === 3 && publish.unsigned && publish.sign.done.includes("<b>"), "publishing without signing does not reach the third step");
        publish.where.pick("file");
        expect(publish.where.can && publish.where.target.endsWith(".wpak"), "the file destination has no file name");
        expect(Publishing.upload("409 Conflict: version 1.0.0 already exists") === "version" && Publishing.upload("unknown author key for account") === "registro" && Publishing.upload("network is unreachable (os error 101)") === "red" && Publishing.upload("cannot reach the server: error sending request") === "red", "an upload error is not told apart");
        expect(Publishing.nextVersion("1.2.0") === "1.2.1", "the next version is wrong");
        publish.where.pick("server");
        publish.work.refused("connect: network is unreachable (os error 101)");
        expect(publish.fault.kind === "red" && publish.fault.details.includes("os error"), "a network error is not shown with its details");
        publish.where.pick("repo");
        publish.where.repo.choose("/missing/repository");
        expect(!publish.where.repo.ready, "a folder that is no repository can be published to");
        publish.work.finish({
            "kind": "file",
            "path": "/tmp/check/package-1.wpak"
        });
        expect(publish.at === 4 && publish.result.kind === "file", "the result is not shown");
        publish.close();
        publish.open();
        expect(publish.at === 1 && publish.result === null, "publishing again does not start over");
        publish.review.lineage.inspect("/missing/package.wpak");
        expect(publish.review.lineage.confirmed, "a missing original holds the package back");
        publish.close();
        publish.destroy();
    }

    function keys() {
        const dialog = Qt.createComponent("SceneKeys.qml").createObject(page, {});
        dialog.refresh();
        const before = dialog.found.length;
        dialog.backend.create(dialog.where, "check-key", "", false);
        expect(!dialog.failed && dialog.made.endsWith("check-key.key"), "a key pair was not made: " + dialog.message);
        expect(dialog.message.includes("check-key.key") && dialog.message.includes("check-key.pub") && dialog.message.includes("SECRET"), "the key pair does not say which file is secret: " + dialog.message);
        expect(dialog.found.length === before + 1 && !JSON.stringify(dialog.found).includes("RWR"), "the new key is not listed alone");
        expect(dialog.backend.exists(dialog.where, "check-key"), "the new key pair is not on disk");
        const first = dialog.found.find(k => k.name === "check-key").fingerprint;
        dialog.backend.create(dialog.where, "check-key", "", false);
        expect(dialog.failed, "a key pair was replaced without asking");
        dialog.backend.create(dialog.where, "check-key", "", true);
        expect(!dialog.failed && dialog.found.find(k => k.name === "check-key").fingerprint !== first, "a key pair was not replaced on request: " + dialog.message);
        dialog.chosen = dialog.made;
        const peer = dialog.found.find(k => k.name === "check-key");
        dialog.backend.rotate(dialog.chosen, "", peer["public"], "studio");
        expect(!dialog.failed && dialog.handover.includes("untrusted comment"), "a hand-over was not signed: " + dialog.message);
        expect(dialog.backend.unlocks(dialog.made, ""), "a key without a password does not unlock");
        expect(JSON.parse(dialog.backend.keep(dialog.made, "/missing/folder")).error !== undefined, "a copy into a missing folder did not fail");
        dialog.destroy();
    }

    function copying() {
        const sample = JSON.parse(Qt.createQmlObject("import io.github.wallpaper_rs.app\nStall {}", page).sample("copying"));
        const dialog = Qt.createComponent("ItemCopy.qml").createObject(page, {});
        dialog.start(sample.wpak);
        expect(dialog.facts.verdict === "allowed" && dialog.ready, "a package with a free license cannot be copied: " + JSON.stringify(dialog.facts));
        expect((dialog.facts.summary ?? "") !== "" && Array.isArray(dialog.facts.conditions), "the copy dialog does not explain the license: " + JSON.stringify(dialog.facts));
        expect(JSON.stringify(JSON.parse(dialog.helper.copyLineage(sample.wpak))) === "[]" && JSON.parse(dialog.helper.copyLineage("/missing/package.wpak")).error !== undefined, "the warning about an original that allows no changes is wrong");
        const shelf = Qt.createComponent("SceneLineage.qml").createObject(page, {});
        shelf.inspect(sample.wpak);
        expect(shelf.restricted.length === 0 && shelf.confirmed, "a package with a free license is held back when it is published");
        shelf.destroy();
        expect(dialog.facts.new_id === "org.example.check.copy" && dialog.facts.kind === "image", "the copy does not say what it is");
        const folder = dialog.helper.folderFor("copy check");
        const made = JSON.parse(dialog.helper.copyInto(sample.wpak, folder, "org.example.check.mine", false));
        expect(made.id === "org.example.check.mine", "the copy was not unpacked: " + JSON.stringify(made));
        expect(JSON.parse(dialog.helper.copyInto(sample.wpak, folder, "", false)).error !== undefined, "a folder in use was overwritten");
        expect(JSON.parse(dialog.helper.copyFacts("/missing/package.wpak")).error !== undefined, "a missing package has facts");
        dialog.close();
        dialog.destroy();
    }

    function exercise() {
        copying();
        review();
        keys();
    }
}
