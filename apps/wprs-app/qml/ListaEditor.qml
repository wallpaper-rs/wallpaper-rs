import QtQuick
import QtQuick.Dialogs
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

FocusScope {
    id: editor

    required property var page
    required property var playlist
    readonly property string uid: playlist.uid ?? ""
    readonly property var presets: ["10m", "30m", "1h", "1d"]
    readonly property int preset: presets.indexOf(String(playlist.rules ? playlist.rules.interval : ""))
    readonly property var running: (playlist.outputs ?? []).length > 0
    readonly property bool manual: playlist.query === null || playlist.query === undefined
    readonly property var shownNow: (playlist.outputs ?? []).map(output => (editor.page.screens[output] ?? {}).item ?? "")
    property var items: []
    readonly property alias rows: rows
    readonly property alias interval: interval
    readonly property alias wallpapers: wallpapers
    readonly property alias list: list
    readonly property alias more: more
    readonly property var hints: [{
            "keys": ["up", "down"],
            "text": App.tr("Move around")
        }, {
            "keys": ["left", "right"],
            "text": App.tr("Change")
        }, {
            "keys": ["Esc"],
            "text": App.tr("Back")
        }]
    readonly property list<QtObject> actions: [
        Kirigami.Action {
            visible: editor.running
            icon.name: "clock"
            text: editor.page.shelf.note(editor.playlist)
            enabled: false
        },
        Kirigami.Action {
            visible: editor.running
            icon.name: "pause-fill"
            text: App.tr("Pause the list")
            onTriggered: editor.stop(true)
        },
        Kirigami.Action {
            visible: editor.running
            text: App.tr("Stop")
            onTriggered: editor.stop(false)
        }
    ]

    function load() {
        if (uid !== "")
            App.query("Get", [uid, 0, 500], "Playlists1", reply => editor && reply.playlist.uid === editor.uid && editor.fill(reply.items));
    }

    function fill(list) {
        items = list;
        const missing = list.filter(item => !item.thumb).map(item => item.uid);
        if (missing.length > 0)
            App.query("RequestThumbnails", [missing, 256], "Library1", reply => editor && (editor.items = editor.items.map(item => item.thumb || !reply[0][item.uid] ? item : Object.assign({}, item, {
                                "thumb": reply[0][item.uid]
                            }))));
    }

    function stop(again) {
        const was = playlist;
        (was.outputs ?? []).forEach(output => App.act("Stop", [output], "Playlists1"));
        if (again)
            applicationWindow().showPassiveNotification(App.tr("The rotation of “%1” stopped.", was.name), "long", App.tr("Resume"), () => was.outputs.forEach(output => App.act("Start", [was.uid, output], "Playlists1")));
    }

    function edit(member, args) {
        App.act(member, [uid].concat(args), "Playlists1", null, error => {
            if (!editor || error.name !== App.conflict)
                return;
            applicationWindow().showPassiveNotification(App.tr("The playlist changed meanwhile; it was reloaded."));
            editor.load();
        });
    }

    function rule(key, value) {
        App.act("SetRule", [uid, key, value], "Playlists1");
    }

    function enter() {
        rows.children[0].forceActiveFocus(Qt.TabFocusReason);
    }

    onUidChanged: load()
    onVisibleChanged: visible && Qt.callLater(() => editor && editor.enter())

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "playlists")
                editor.load();
        }
    }

    QQC2.ScrollView {
        anchors.fill: parent
        contentWidth: availableWidth
        visible: editor.page.level === ""

        Filas {
            id: rows

            Fila {
                text: App.tr("Name")
                kind: "campo"
                fieldText: editor.playlist.name ?? ""
                onEdited: text => text.trim() !== "" && App.act("Rename", [editor.uid, text.trim()], "Playlists1")
            }

            Fila {
                id: interval

                text: App.tr("Changes every")
                kind: "elige"
                options: [App.tr("10 min"), App.tr("30 min"), App.tr("1 h"), App.tr("Every day"), editor.preset < 0 ? editor.page.shelf.every(editor.playlist.rules ? editor.playlist.rules.interval : "") : App.tr("Other…")]
                current: editor.preset < 0 ? 4 : editor.preset
                explain: App.tr("How often it moves to the next wallpaper. “Other…”, at the end, takes seconds, minutes or hours.")
                onChosen: index => index < 4 ? editor.rule("interval", editor.presets[index]) : other.open()
            }

            Fila {
                visible: App.outputs.length > 1
                text: App.tr("Which screen")
                kind: "elige"
                options: App.outputs
                current: Math.max(0, App.outputs.indexOf((editor.playlist.outputs ?? [])[0] ?? editor.page.target))
                onChosen: index => App.act("Start", [editor.uid, App.outputs[index]], "Playlists1")
            }

            Fila {
                id: wallpapers

                text: App.tr("Wallpapers")
                kind: "despliega"
                note: String(editor.playlist.count ?? 0) + " · " + (editor.manual ? App.tr("Drag to change the order") : App.tr("Chosen by its search"))
            }

            Sublista {
                id: list

                width: parent.width
                visible: wallpapers.open
                movable: editor.manual
                removeText: editor.manual ? App.tr("Remove") : ""
                addText: editor.manual ? App.tr("Add wallpapers…") : ""
                otherText: editor.manual ? App.tr("Add a folder…") : ""
                otherIcon: "folder-add"
                model: editor.items.map(item => ({
                            "name": item.title ?? item.uid,
                            "picture": item.thumb ? "file://" + item.thumb : "",
                            "where": editor.shownNow.includes(item.uid) ? App.tr("on screen now") : ""
                        }))
                onRemoved: index => editor.edit("RemoveEntries", [[index], [editor.items[index].uid]])
                onReordered: (from, to) => editor.edit("MoveItem", [from, to, editor.items[from].uid])
                onAdded: applicationWindow().showPage(0)
                onOtherAdded: folder.open()
            }

            Fila {
                text: App.tr("More")
                kind: "pliega"
                note: App.tr("History, search, rules and removing the list")
                onOpened: editor.page.level = "more"
            }
        }
    }

    ListaMas {
        id: more

        anchors.fill: parent
        visible: editor.page.level === "more"
        page: editor.page
        playlist: editor.playlist
    }

    ListaOtro {
        id: other

        editor: editor
    }

    FolderDialog {
        id: folder

        onAccepted: App.act("AddFolder", [editor.uid, App.path(selectedFolder), false], "Playlists1", reply => applicationWindow().showPassiveNotification(App.tr("The folder has %1 pictures and videos", reply[0])))
    }
}
