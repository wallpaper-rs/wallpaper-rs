import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Page {
    id: page

    property string query
    property var playlists: []
    property var pictures: ({})
    property string error
    property bool loaded: false
    property int kind: 0
    property string opened
    property string level
    property string target: App.outputs.length > 0 ? App.outputs[0] : ""
    readonly property var screens: App.state.outputs ?? {}
    readonly property var running: playlists.filter(p => p.outputs.length > 0)
    readonly property var shown: kind === 1 ? running : playlists
    readonly property var playlist: playlists.find(p => p.uid === opened) ?? null
    readonly property string skeleton: playlist ? "options" : ""
    readonly property var route: playlist ? [App.tr("Playlists"), playlist.name].concat(level !== "" ? [App.tr("More")] : []) : []
    readonly property real bandLow: playlist ? 0 : shelf.low
    readonly property real bandHigh: bandLow
    property rect band
    property rect upper
    readonly property var keyHints: playlist ? undefined : shelf.hints
    readonly property alias shelf: shelf
    readonly property alias creator: creator

    function load() {
        App.query("List", [], "Playlists1", reply => {
            if (!page)
                return;
            page.playlists = reply.playlists;
            page.error = "";
            page.loaded = true;
            reply.playlists.forEach(p => page.cover(p.uid));
        }, error => page && (page.error = error.message));
    }

    function cover(uid) {
        App.query("Get", [uid, 0, 4], "Playlists1", reply => {
            if (!page)
                return;
            const next = Object.assign({}, page.pictures);
            next[uid] = reply.items.filter(item => !!item.thumb).map(item => "file://" + item.thumb);
            page.pictures = next;
        });
    }

    function next(p) {
        const on = (p.outputs ?? []).map(name => screens[name] && screens[name].playlist ? screens[name].playlist.next_at : null).filter(at => !!at);
        return on.length > 0 ? Math.min(...on) : 0;
    }

    function start(p) {
        const output = App.outputs.length > 1 ? target : "";
        App.act("Start", [p.uid, output], "Playlists1");
    }

    function remove(p) {
        App.query("Get", [p.uid, 0, 100000], "Playlists1", reply => {
            const uids = reply.items.map(item => item.uid);
            App.act("Delete", [p.uid], "Playlists1", () => {
                if (page)
                    page.routeBack();
                applicationWindow().showPassiveNotification(App.tr("The list “%1” was removed.", p.name), "long", App.tr("Undo"), () => App.act("Create", [p.name, p.query ?? ""], "Playlists1", made => {
                    if (p.query === null && uids.length > 0)
                        App.act("AddItems", [made[0], uids], "Playlists1");
                    ["order", "interval", "synced", "resume", "advance_while_paused", "skip_videos_on_battery"].forEach(key => App.act("SetRule", [made[0], key, String(p.rules[key])], "Playlists1"));
                }));
            });
        });
    }

    function keys() {
        check.keys();
    }

    function exercise() {
        check.exercise();
    }

    function routeBack() {
        if (level !== "")
            return level = "";
        opened = "";
        Qt.callLater(() => page && shelf.forceActiveFocus());
    }

    title: playlist ? playlist.name : App.tr("Playlists")
    padding: 0
    background: null
    actions: playlist ? editor.actions : []
    Component.onCompleted: {
        load();
        if (query)
            creator.open();
    }

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "playlists")
                page.load();
        }
    }

    ListasEstante {
        id: shelf

        visible: !page.playlist
        x: page.bandLow > 0 ? page.band.x : 0
        y: page.bandLow > 0 ? page.band.y : 0
        width: page.bandLow > 0 ? page.band.width : page.width
        height: page.bandLow > 0 ? page.band.height : page.height
        page: page
        focus: !page.playlist
    }

    ListaEditor {
        id: editor

        visible: !!page.playlist
        anchors.fill: parent
        page: page
        playlist: page.playlist ?? ({})
    }

    ListaNueva {
        id: creator

        page: page
    }

    ListasCheck {
        id: check

        page: page
    }
}
