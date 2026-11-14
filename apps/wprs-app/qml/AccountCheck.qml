import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var page

    function expect(ok, what) {
        if (!ok)
            throw new Error("account page: " + what);
    }

    function seat(state, extra) {
        page.level = "";
        page.account = state === null ? null : Object.assign({}, check.kept, {
            "state": state,
            "pairing": null,
            "last_error": null,
            "link_problem": null
        }, extra || {});
        return page.shown.item;
    }

    property var kept: null

    function signedOut() {
        const main = seat(null);
        expect(main.empty.visible, "without a server it offers to add one");
        main.empty.acted();
        expect(page.panels.adder.opened || page.panels.adder.visible, "«Add a server…» opens its panel");
        page.panels.adder.close();
    }

    function connected() {
        const main = seat("connected");
        expect(main.sync.current === 0 && !main.sync.off, "connected without syncing shows syncing off");
        expect(main.line.text === App.tr("Connected · not syncing"), "connected shows its state line");
    }

    function syncing() {
        const main = seat("ready");
        expect(main.sync.current === 1, "a syncing computer shows syncing on");
        expect(main.line.tone === "brote", "a syncing computer shows a good state line");
        expect(!main.waiting.visible && !main.problem.link.visible, "a syncing computer shows nothing waiting and no problem");
        page.open("more");
        expect(page.route.length === 2, "«More» is a step of the route");
        page.routeBack();
        expect(page.level === "", "Esc goes back from «More»");
    }

    function waiting() {
        const main = seat("ready", {
            "pairing": {
                "device": "0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b",
                "code": "123 456",
                "sharing": true
            }
        });
        expect(main.waiting.visible && main.waiting.value === "123 456", "another device waiting shows its number");
        main.waiting.opened();
        expect(page.codes.shown === "123 456", "the comparison shows the number of this computer");
        page.codes.sharer.close();
    }

    function failing() {
        let main = seat("error", {
            "last_error": "server unreachable"
        });
        expect(main.line.tone === "coral", "a failed sync shows a coral state line");
        page.failure = "keyring";
        expect(main.problem.link.visible, "a connection that failed for lack of a keyring shows its problem");
        page.filer.open();
        page.filer.close();
        page.failure = "";
        main = seat("signed_out");
        expect(main.problem.signedOut, "a computer the server no longer accepts shows its problem");
    }

    function ghosts() {
        seat("ready");
        page.level = "devices";
        const devices = page.shown.item;
        const ghost = {
            "id": "1",
            "name": "Device 3",
            "current": false,
            "seen_at": null,
            "created_at": 1
        };
        const seen = Object.assign({}, ghost, {
            "seen_at": 2
        });
        const acts = device => devices.offers(device).map(offer => offer.act);
        expect(!acts(ghost).includes("share") && acts(ghost).includes("revoke"), "a device that never connected offers no key, only removal");
        expect(acts(seen).includes("share"), "a device that connected can be let to sync");
        page.revoker.ask(ghost, page);
        page.revoker.close();
    }

    function code() {
        seat("ready");
        page.level = "server";
        const server = page.shown.item;
        expect(server.qr.modules.length > 0, "the server level draws a code for the phone");
        server.code.opened();
        server.qr.close();
    }

    function run() {
        kept = page.account;
        if (!kept)
            throw new Error("account page: the fixture account did not load");
        signedOut();
        connected();
        syncing();
        waiting();
        failing();
        ghosts();
        code();
        page.account = kept;
        page.level = "";
    }
}
