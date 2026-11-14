import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Page {
    id: page

    property var providers: []
    property var playlists: []
    property var seen: ({})
    property string view: "list"
    property string kind
    property string store: "keyring"
    readonly property real bandLow: view === "list" && providers.length > 0 ? list.lowest : 0
    readonly property real bandHigh: list.tallest
    readonly property real upperHeight: notice.visible ? notice.implicitHeight : 0
    property rect band
    property rect upper
    property string current
    property string level
    readonly property var chosen: providers.find(p => p.id === current) || ({})
    readonly property string skeleton: view !== "list" ? "options" : ""
    readonly property string leaf: view === "pick" ? App.tr("Add a source") : view === "add" ? App.tr("Add %1", SourceKinds.of(kind).name) : !!chosen.id ? SourceKinds.heading(chosen) : ""
    readonly property Item shown: body.shown
    readonly property alias cards: list
    readonly property var route: view === "list" ? [] : view === "more" ? [App.tr("Sources"), leaf, App.tr("More options")].concat(level ? [SourceMoreWords.level(level)] : []) : [App.tr("Sources"), leaf]
    readonly property var keyHints: [{
            "keys": ["up", "down"],
            "text": App.tr("Move around")
        }, {
            "keys": ["Intro"],
            "text": App.tr("Choose")
        }, {
            "keys": ["Esc"],
            "text": view === "list" ? App.tr("Menu") : App.tr("Back")
        }]
    readonly property Component now: SourcesNow {
        sources: page.providers.length
        photos: page.providers.reduce((sum, p) => sum + p.items, 0)
    }

    function routeBack() {
        if (view === "more")
            return level === "" ? open("detail") : deeper(SourceMoreWords.parent(level));
        view = view === "add" && providers.length === 0 ? "list" : view === "add" ? "pick" : "list";
    }

    function deeper(next) {
        level = next;
        open("more");
    }

    function apply(next) {
        acts.apply(next);
    }

    function load() {
        App.query("List", [], "Providers1", reply => {
            if (!page)
                return;
            page.store = reply.store || "keyring";
            page.apply(reply.providers);
        });
        App.query("List", [], "Playlists1", reply => page && (page.playlists = reply.playlists));
    }

    function open(next) {
        view = next;
        Qt.callLater(() => page && page.focusView());
    }

    function focusView() {
        const item = view === "list" ? (providers.length === 0 ? kinds : list) : view === "pick" ? picker : body.shown;
        if (item)
            item.enter();
    }

    function exercise() {
        current = providers.length > 0 ? providers[0].id : "";
        kind = "wallhaven";
        ["pick", "add", "detail", "list"].forEach(v => page.open(v));
        Qt.createComponent("SourcesCheck.qml").createObject(page, {
            "page": page
        }).exercise();
    }

    title: App.tr("Sources")
    padding: 0
    Component.onCompleted: load()
    onProvidersChanged: if (view === "list")
        Qt.callLater(() => page && page.focusView())

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "providers" || topic === "playlists")
                page.load();
        }
    }

    SourceList {
        id: list

        x: page.bandLow > 0 ? page.band.x : 0
        y: page.bandLow > 0 ? page.band.y : 0
        width: page.bandLow > 0 ? page.band.width : page.width
        height: page.bandLow > 0 ? page.band.height : page.height
        wide: page.bandLow > 0
        visible: page.view === "list" && page.providers.length > 0
        providers: page.providers
        playlists: page.playlists
        onRotate: provider => acts.rotate(provider)
        onDetails: provider => {
            page.current = provider.id;
            page.open("detail");
        }
        onFix: provider => provider.state === "needs_secret" ? dialogs.keys.ask(provider.id) : App.act("Sync", [provider.id], "Providers1")
        onAddAsked: page.open("pick")
        onRefreshAsked: id => App.act("Sync", [id], "Providers1")
    }

    SourceKindCards {
        id: kinds

        anchors.bottom: parent.bottom
        width: Math.min(parent.width, Look.tiposAncho)
        visible: page.view === "list" && page.providers.length === 0
        onPicked: kind => acts.start(kind)
    }

    SourceKindCards {
        id: picker

        anchors.bottom: parent.bottom
        width: Math.min(parent.width, Look.tiposAncho)
        visible: page.view === "pick"
        onPicked: kind => acts.start(kind)
        onClosed: page.routeBack()
    }

    SourceBody {
        id: body

        anchors.fill: parent
        host: page
        dialogs: dialogs
    }

    SourceActions {
        id: acts

        host: page
    }

    SecretNotice {
        id: notice

        x: page.bandLow > 0 ? page.upper.x : 0
        y: page.bandLow > 0 ? page.upper.y : 0
        width: page.bandLow > 0 ? page.upper.width : page.width
        visible: page.view === "list"
        filer: dialogs.filer
    }

    Binding {
        target: page.background
        property: "visible"
        value: page.bandLow === 0
    }

    SourceDialogs {
        id: dialogs

        host: page
    }
}
