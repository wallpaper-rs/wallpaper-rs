import QtQuick

QtObject {
    required property var page

    function expect(ok, what) {
        if (!ok)
            throw new Error("authoring: " + what);
    }

    function review() {
        const dialog = Qt.createComponent("SceneReview.qml").createObject(page, {
            "page": page
        });
        dialog.refresh();
        expect(Array.isArray(dialog.report.findings) && dialog.report.signature.state === "unsigned", "the review has no findings list or no signature state");
        expect(Object.keys(dialog.platforms).includes("desktop"), "the review does not say where the package plays");
        dialog.built = "/missing/package.wpak";
        dialog.check();
        expect(dialog.signature.state === "invalid", "a signature of a missing package was not refused");
        dialog.destroy();
    }

    function keys() {
        const dialog = Qt.createComponent("SceneKeys.qml").createObject(page, {});
        dialog.refresh();
        const before = dialog.found.length;
        dialog.backend.create(dialog.where, "check-key", "", false);
        expect(!dialog.failed && dialog.made.endsWith("check-key.key"), "a key pair was not made: " + dialog.message);
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
        dialog.destroy();
    }

    function shelving() {
        const dialog = Qt.createComponent("SceneShelve.qml").createObject(page, {
            "page": page
        });
        dialog.chosen("/missing/repository");
        expect(dialog.failed && dialog.shown === null, "a folder that is no repository was opened");
        dialog.aboutToShow();
        expect(dialog.id !== "" && dialog.id === JSON.parse(page.editor.value("/manifest/id")), "the dialog does not name the package it adds");
        dialog.destroy();
    }

    function exercise() {
        review();
        keys();
        shelving();
    }
}
