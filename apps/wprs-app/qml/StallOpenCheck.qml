import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Timer {
    id: check

    required property var host
    property bool finished: !App.backend.claim("stall.open")
    property var opened: null

    function must(ok, text) {
        if (!ok)
            throw new Error("static repository page: " + text);
    }

    function begin() {
        if (finished)
            return;
        start();
        host.showPage(host.names.indexOf("repos"));
        host.pageStack.currentItem.upperBlock.more.pick(1);
        opened = host.pageStack.currentItem;
    }

    interval: 250
    onTriggered: {
        finished = true;
        must(host.pageStack.currentItem === opened && opened.stall !== undefined, "Explore did not open it");
        const admin = opened.stall.admin;
        const making = admin.children.find(child => child instanceof QQC2.Button && child.text === App.tr("New repository…"));
        must(making !== undefined, "it has no button for a new repository");
        making.clicked();
        must(admin.charter.opened || admin.charter.visible, "its button for a new repository opens nothing");
        admin.charter.close();
        opened.actions[0].trigger();
        must(host.current === host.names.indexOf("repos"), "its way back to Explore does nothing");
    }
}
