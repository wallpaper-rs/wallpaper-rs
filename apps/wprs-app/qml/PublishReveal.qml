import QtQuick
import io.github.wallpaper_rs.app

Timer {
    id: reveal

    required property var publish
    required property var flick

    function seen(item) {
        const top = item.mapToItem(flick.contentItem, 0, 0).y;
        return top >= flick.contentY && top + item.height <= flick.contentY + flick.height;
    }

    function target() {
        if (publish.at === 1)
            return publish.review.going;
        if (publish.at !== 3)
            return null;
        return publish.fault !== null ? publish.where.trouble.button : publish.where.actions;
    }

    function run() {
        const item = target();
        if (item === null || !item.visible || seen(item))
            return;
        const bottom = item.mapToItem(flick.contentItem, 0, 0).y + item.height + Look.e4;
        flick.contentY = Math.max(0, Math.min(bottom - flick.height, flick.contentHeight - flick.height));
    }

    interval: 50
    onTriggered: run()
}
