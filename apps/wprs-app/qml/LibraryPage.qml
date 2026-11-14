import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Page {
    id: page

    property bool all: false
    property bool choosing: false
    property var chosen: []
    property string opened
    property string level
    property string target: App.outputs.length > 0 ? App.outputs[0] : ""
    readonly property alias library: library
    readonly property alias tools: tools
    readonly property alias covers: covers
    readonly property alias host: host
    readonly property var entry: {
        const current = covers.view.view ? covers.view.view.currentItem : null;
        return current && current.model ? current.model : null;
    }
    readonly property string skeleton: opened !== "" ? "options" : ""
    readonly property bool full: all && opened === ""
    readonly property var route: opened !== "" ? [App.tr("Library"), host.item.title ?? ""].concat(level !== "" ? [panel.levelName] : []) : []

    function search() {
        library.search(bar.text, bar.kind !== "" ? bar.kind : bar.kinds[bar.word], bar.favorite, bar.tag);
    }

    function sync() {
        chosen = JSON.parse(library.chosen());
        if (chosen.length === 0 && library.selected === 0)
            choosing = false;
    }

    function toggle(index) {
        choosing = true;
        library.toggle(index, false);
        sync();
    }

    function stop() {
        library.everything(false);
        choosing = false;
        sync();
    }

    function activate() {
        if (!entry)
            return;
        if (entry.missing === true || String(entry.broken ?? "") !== "")
            return folder(entry.uid);
        tools.put(entry.uid);
    }

    function folder(uid) {
        App.query("GetItem", [uid], "Library1", reply => {
            const path = String(reply.item ? reply.item.path ?? "" : "");
            if (page && path !== "")
                Qt.openUrlExternally("file://" + path.substring(0, path.lastIndexOf("/")));
        });
    }

    function open(uid) {
        level = "";
        host.show(uid);
    }

    function routeBack() {
        if (level !== "")
            return level = "";
        host.show("");
        Qt.callLater(() => page && covers.view.forceActiveFocus());
    }

    function keys() {
        check.keys();
    }

    function exercise() {
        check.exercise();
    }

    function shown() {
        const outputs = screens;
        const map = {};
        Object.keys(outputs).forEach(name => map[name] = outputs[name].item ?? "");
        library.showing(JSON.stringify(map));
    }

    title: opened !== "" ? host.item.title ?? "" : App.tr("Library")
    padding: 0
    readonly property var keyHints: opened !== "" ? undefined : choosing ? [{
            "keys": ["Esc"],
            "text": App.tr("Stop choosing")
        }] : all ? [{
            "keys": ["left", "right", "up", "down"],
            "text": App.tr("Move")
        }, {
            "keys": ["Esc"],
            "text": App.tr("Show shelf")
        }] : [{
            "keys": ["left", "right"],
            "text": App.tr("Move")
        }, {
            "keys": ["down"],
            "text": App.tr("See all")
        }, {
            "keys": ["Esc"],
            "text": App.tr("Menu")
        }]
    actions: opened !== "" ? panel.actions : []
    readonly property var screens: App.state.outputs ?? {}
    onScreensChanged: shown()
    Component.onCompleted: {
        shown();
        search();
    }

    ItemModel {
        id: library

        size: page.Screen.devicePixelRatio > 1 ? 512 : 256
        onReloaded: {
            page.sync();
            if (library.total > 0 && covers.view.currentIndex < 0)
                covers.view.currentIndex = 0;
            Qt.callLater(covers.look);
        }
    }

    Item {
        id: band

        visible: page.opened === ""
        anchors.fill: parent

        Column {
            y: Math.max(0, bar.y - height - Look.e3)
            width: Math.min(parent.width, Look.tapaFoco * 2)
            visible: !page.all

            SyncedFiles {
                width: parent.width
            }

            AwayNote {
                width: parent.width
            }
        }

        LibraryBar {
            id: bar

            y: page.all ? 0 : Math.max(0, covers.y - height - Look.e5)
            tools: tools
            total: library.total
            all: page.all
            onChanged: page.search()
            onFlipped: page.all = !page.all
        }

        LibraryCovers {
            id: covers

            visible: library.total > 0
            page: page
            y: page.all ? bar.height + Look.e5 : Math.max(bar.height + Look.e5, band.height - height)
            width: parent.width
        }

        LibraryState {
            page: page
            bar: bar
            anchors.fill: parent
        }
    }

    LibraryPanel {
        id: panel

        visible: page.opened !== ""
        anchors.fill: parent
        page: page
    }

    ItemHost {
        id: host

        onUidChanged: page.opened = uid
        onApply: uid => tools.put(uid)
        onClosed: page.level = ""
    }

    LibraryCheck {
        id: check

        page: page
    }

    LibraryDrop {
        anchors.fill: parent
    }

    Actions {
        id: tools

        items: library
        target: page.target
    }
}
