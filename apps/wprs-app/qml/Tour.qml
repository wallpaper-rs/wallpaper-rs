import QtQuick
import io.github.wallpaper_rs.app

Timer {
    id: tour

    required property var host
    property int step: 0
    property var opened: null
    property bool edited: false
    property bool imaged: false

    function owns(at) {
        const name = host.names[at - 1];
        return name === "scenes" || App.backend.claim("page." + name);
    }

    function keeps(page) {
        const shown = host.pageStack.currentItem;
        host.activate(page);
        if (host.pageStack.currentItem !== shown)
            throw new Error("a request for the " + page + " page replaced the page shown");
    }

    readonly property KeyWalk walking: KeyWalk {
        host: tour.host
    }
    readonly property KeyProbe probing: KeyProbe {
        host: tour.host
        walk: tour.walking
    }

    interval: 300
    repeat: true
    running: App.checking
    onTriggered: {
        if (Object.keys(App.pending).length > 0 || probing.running)
            return;
        if (!probing.finished)
            return probing.begin();
        const at = step++;
        if (at > 0 && at <= host.pages.length && owns(at)) {
            walking.check(host.names[at - 1]);
            host.pageStack.currentItem.keys?.();
            host.pageStack.currentItem.exercise?.();
        }
        if (at < host.pages.length) {
            host.showPage(at);
            keeps(host.names[at]);
        } else if (at === host.pages.length) {
            if (App.backend.claim("look"))
                Qt.createComponent("LookCheck.qml").createObject(host, {
                    "host": host
                }).exercise();
            if (App.backend.claim("shell"))
                Qt.createComponent("ShellCheck.qml").createObject(host, {
                    "host": host
                }).exercise();
            if (App.backend.claim("pieces.choose"))
                Qt.createComponent("PiecesChooseCheck.qml").createObject(host, {
                    "host": host
                }).exercise();
            if (App.backend.claim("pieces.band"))
                Qt.createComponent("PiecesBandCheck.qml").createObject(host, {
                    "host": host
                }).exercise();
            if (App.backend.claim("pieces.adjust"))
                Qt.createComponent("PiecesAdjustCheck.qml").createObject(host, {
                    "host": host
                }).exercise();
            if (App.backend.claim("create"))
                Qt.createComponent("CreateCheck.qml").createObject(host, {
                    "host": host
                }).exercise();
            if (App.backend.claim("repo"))
                Qt.createComponent("RepoCheck.qml").createObject(host, {
                    "host": host
                }).start();
            return;
        } else if (at === host.pages.length + 1) {
            imaged = App.backend.claim("image");
            if (!imaged)
                return;
            return App.query("GetItem", [""], "Library1", reply => {
                if (!host)
                    return;
                App.edit(reply.item);
                opened = host.pageStack.currentItem;
            });
        } else if (imaged && (opened?.editor?.ready !== true || host.pageStack.currentItem !== opened)) {
            return;
        } else if (imaged && !edited) {
            edited = true;
        } else if (host.waiting > 0) {
            return;
        } else {
            stop();
            Qt.quit();
            if (!imaged)
                return;
            if (opened.editor.dirty !== true)
                throw new Error("the editor has no unsaved edits to keep");
            return keeps("settings");
        }
        if (imaged && at > host.pages.length + 1)
            host.pageStack.currentItem.exercise?.();
    }
}
