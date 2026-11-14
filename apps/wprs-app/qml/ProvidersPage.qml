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
    property string current
    readonly property var chosen: providers.find(p => p.id === current) || ({})
    readonly property bool options: view !== "list"
    readonly property string leaf: view === "pick" ? App.tr("Add a source") : view === "add" ? App.tr("Add %1", SourceKinds.of(kind).name) : view === "detail" && !!chosen.id ? SourceKinds.heading(chosen) : ""
    readonly property var route: view === "list" ? [] : [App.tr("Sources"), leaf]
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
        view = view === "add" && providers.length === 0 ? "list" : view === "add" ? "pick" : "list";
    }

    function apply(next) {
        acts.apply(next);
    }

    function load() {
        App.query("List", [], "Providers1", reply => page && page.apply(reply.providers));
        App.query("List", [], "Playlists1", reply => page && (page.playlists = reply.playlists));
    }



    function open(next) {
        view = next;
        Qt.callLater(() => page && page.focusView());
    }

    function focusView() {
        const item = view === "list" ? (providers.length === 0 ? kinds : list) : view === "pick" ? picker : body.item;
        if (item)
            item.enter();
    }

    function exercise() {
        current = providers.length > 0 ? providers[0].id : "";
        kind = "wallhaven";
        ["pick", "add", "detail", "list"].forEach(v => page.open(v));
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

        anchors.fill: parent
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

    Flickable {
        id: scroll

        function reveal(item) {
            if (!item || !body.item || !body.item.contains(item.mapToItem(body.item, 0, 0)))
                return;
            const y = item.mapToItem(contentItem, 0, 0).y;
            if (y < contentY)
                contentY = Math.max(0, y - Look.e4);
            else if (y + item.height > contentY + height)
                contentY = y + item.height - height + Look.e4;
        }

        anchors.fill: parent
        visible: page.view === "add" || page.view === "detail"
        contentHeight: Math.max(height, body.implicitHeight)
        interactive: contentHeight > height
        clip: true

        Connections {
            target: page.Window.window

            function onActiveFocusItemChanged() {
                scroll.reveal(page.Window.window.activeFocusItem);
            }
        }

        Loader {
            id: body

            anchors.bottom: parent.bottom
            width: Math.min(scroll.width, Look.filasAncho)
            active: scroll.visible && (page.view === "add" || !!page.chosen.id)
            sourceComponent: page.view === "add" ? addForm : detailForm
        }
    }

    Component {
        id: addForm

        SourceAdd {
            kind: page.kind
            taken: page.providers.map(p => p.id)
            onDone: page.open("list")
            onCancelled: page.routeBack()
            onKeyFiler: dialogs.filer.open()
        }
    }

    Component {
        id: detailForm

        SourceDetail {
            provider: page.chosen
            onClosed: page.open("list")
            onRemoved: page.open("list")
            onKeyAsked: dialogs.keys.ask(page.chosen.id)
            onMore: dialogs.wallhaven.open()
            onListed: App.act("Create", [SourceKinds.heading(page.chosen), "source:" + page.chosen.source], "Playlists1", () => App.navigate(1, ""))
        }
    }

    SourceActions {
        id: acts

        host: page
    }

    SecretNotice {
        anchors.left: parent.left
        anchors.top: parent.top
        filer: dialogs.filer
    }


    SourceDialogs {
        id: dialogs

        host: page
    }
}
