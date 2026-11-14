import QtQuick
import io.github.wallpaper_rs.app

Timer {
    id: check

    required property var host
    readonly property var frame: host.shell.titleFrame

    function must(ok, text) {
        if (!ok)
            throw new Error("now block: " + text);
    }

    function start() {
        host.waiting++;
        host.held = host.held.concat([check]);
        host.showPage(0);
        host.showPassiveNotification("check");
        running = true;
    }

    function empty() {
        const block = Qt.createComponent("Ahora.qml").createObject(frame, {
            "none": true
        });
        must(block.title === App.tr("You have no wallpapers yet"), "with no wallpaper the block does not say so");
        must(!block.controls.visible, "with no wallpaper the block still offers the controls");
        block.destroy();
    }

    function notice() {
        const notices = host.shell.notices;
        must(notices.text === "check" && notices.height > 0, "a notice does not show");
        if (frame.banded) {
            must(notices.y + notices.height <= frame.band.y, "a notice covers the low band");
            must(notices.x >= frame.band.x, "a notice starts left of the band");
        } else {
            must(Math.abs(notices.x - frame.noticeAt.x) < 1, "a notice left its place in the column");
        }
        host.hidePassiveNotification();
    }

    interval: Look.paso * 6
    onTriggered: {
        try {
            must(frame.bandLow > 0, "the library does not use the low band");
            const tasks = frame.ahora.tasks;
            must(tasks.shown.length === 0 || tasks.height > 0, "the job line is hidden on a page with the band");
            empty();
            notice();
        } finally {
            host.waiting--;
            host.held = host.held.filter(c => c !== check);
        }
    }
}
