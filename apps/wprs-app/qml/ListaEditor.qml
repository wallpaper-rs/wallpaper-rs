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
    property var items: []
    readonly property alias rows: rows
    readonly property alias interval: interval
    readonly property alias wallpapers: wallpapers
    readonly property list<QtObject> actions: [
        Kirigami.Action {
            visible: editor.running
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
            icon.name: "stop-fill"
            text: App.tr("Stop")
            onTriggered: editor.stop(false)
        }
    ]

    function load() {
        if (uid !== "")
            App.query("Get", [uid, 0, 200], "Playlists1", reply => editor && reply.playlist.uid === editor.uid && (editor.items = reply.items));
    }

    function stop(again) {
        const was = playlist;
        (was.outputs ?? []).forEach(output => App.act("Stop", [output], "Playlists1"));
        if (again)
            applicationWindow().showPassiveNotification(App.tr("The rotation of “%1” stopped.", was.name), "long", App.tr("Resume"), () => was.outputs.forEach(output => App.act("Start", [was.uid, output], "Playlists1")));
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
        anchors.leftMargin: Look.margen
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
                kind: "pliega"
                value: String(editor.playlist.count ?? 0)
                note: editor.playlist.query === null ? "" : App.tr("Chosen by its search")
                onOpened: open = !open
            }

            Sublista {
                width: parent.width
                visible: wallpapers.open
                addText: App.tr("Add wallpapers…")
                model: editor.items.map(item => ({
                            "name": item.title ?? item.uid
                        }))
                onRemoved: index => App.act("RemoveItems", [editor.uid, [editor.items[index].uid]], "Playlists1")
                onAdded: applicationWindow().showPage(0)
            }

            QQC2.Button {
                visible: wallpapers.open
                flat: true
                icon.name: "folder-plus"
                text: App.tr("Add a folder…")
                onClicked: folder.open()
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
        anchors.fill: parent
        anchors.leftMargin: Look.margen
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

        onAccepted: App.act("AddFolder", [editor.uid, App.path(selectedFolder), true], "Playlists1", reply => applicationWindow().showPassiveNotification(App.tr("The folder has %1 pictures and videos", reply[0])))
    }
}
