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
    readonly property alias bar: bar
    readonly property alias host: host
    readonly property var entry: covers.view.view?.currentItem?.model ?? null
    readonly property string skeleton: opened !== "" ? "options" : ""
    readonly property bool full: all && opened === ""
    readonly property bool tellsService: opened === "" && state.failed
    readonly property real bandLow: opened !== "" ? 0 : state.alone ? state.low : all ? 0 : notes.room + bar.height + Look.e4 + covers.implicitHeight
    readonly property real bandHigh: bandLow
    readonly property alias notes: notes
    readonly property real bandFrom: state.alone ? lower.y : lower.y + bar.y
    property rect band
    property rect upper
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
            return tools.folder(entry.uid);
        tools.put(entry.uid);
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
    background: null
    readonly property var keyHints: opened !== "" ? undefined : state.starting ? state.start.hints : covers.hints
    actions: opened !== "" ? panel.actions : []
    readonly property Component badge: opened !== "" ? panel.badge : null
    readonly property Component head: opened !== "" ? panel.head : null
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
            Qt.callLater(covers.keep);
            Qt.callLater(covers.look);
        }
    }

    Item {
        id: lower

        visible: page.opened === ""
        readonly property bool placed: page.bandLow > 0 && page.band.width > 0

        x: placed ? page.band.x : 0
        y: placed ? page.band.y : 0
        width: placed ? page.band.width : page.width
        height: placed ? page.band.height : page.height

        LibraryNotes {
            id: notes

            page: page
        }

        LibraryBar {
            id: bar

            visible: !state.alone
            y: page.all ? 0 : Math.max(notes.room, covers.y - height - Look.e4)
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
            y: page.all ? bar.height + Look.e5 : Math.max(notes.room + bar.height + Look.e4, lower.height - height)
            width: parent.width
        }

        LibraryState {
            id: state

            page: page
            bar: bar
            y: alone ? 0 : bar.y + bar.height + Look.e6
            width: parent.width
            height: Math.max(0, parent.height - y)
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
        tools: tools
    }

    Actions {
        id: tools

        items: library
        page: page
        target: page.target
    }
}
