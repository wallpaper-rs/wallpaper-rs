import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: host

    property var wallpaper
    property var catalog
    property var item: ({})
    readonly property var record: {
        if (!wallpaper || wallpaper.kind !== "scene" || !catalog)
            return null;
        const found = catalog.installed.filter(i => i.repo_id === wallpaper.repo_id && i.id === wallpaper.id && i.item);
        return found.length > 0 ? found[found.length - 1] : null;
    }
    readonly property var pkg: record && item.path ? wallpaper : null

    function close() {
    }

    onRecordChanged: {
        item = ({});
        if (!record)
            return;
        const uid = record.item;
        App.query("GetItem", [uid], "Library1", reply => host && host.record && host.record.item === uid && (host.item = reply.item ?? {}), () => {});
    }
}
