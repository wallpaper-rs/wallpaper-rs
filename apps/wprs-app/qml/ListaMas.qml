import QtQuick
import QtQuick.Dialogs
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.ScrollView {
    id: more

    required property var page
    required property var playlist
    readonly property string uid: playlist.uid ?? ""
    readonly property bool manual: playlist.query === null || playlist.query === undefined
    readonly property var query: playlist.query ?? null
    property int matches: -1
    readonly property alias rows: rows
    readonly property alias rules: rules

    function count() {
        if (manual)
            return;
        const query = more.query;
        App.query("Search", [query, 0, 0], "Library1", reply => more && more.query === query && (more.matches = reply.total), () => more && (more.matches = -1));
    }

    function enter() {
        rows.children[0].forceActiveFocus(Qt.TabFocusReason);
    }

    onVisibleChanged: visible && Qt.callLater(() => more && more.enter())
    onQueryChanged: count()
    contentWidth: availableWidth
    Component.onCompleted: count()

    Filas {
        id: rows

        Fila {
            text: App.tr("Search")
            kind: "campo"
            fieldText: more.playlist.query ?? ""
            placeholder: App.tr("Manual playlist")
            note: !more.manual && more.matches >= 0 ? App.tr("Matches: %1", more.matches) : ""
            onEdited: text => App.act("SetQuery", [more.uid, text.trim()], "Playlists1")
        }

        Fila {
            visible: !more.manual
            text: App.tr("Make manual")
            kind: "pliega"
            onOpened: App.act("SetQuery", [more.uid, ""], "Playlists1")
        }

        Fila {
            text: App.tr("Follow a folder…")
            note: App.tr("Keep following it: new files join the playlist")
            kind: "pliega"
            onOpened: follow.open()
        }

        ListaReglas {
            id: rules

            width: parent.width
            playlist: more.playlist
        }

        ListaHistorial {
            width: parent.width
            active: more.visible
        }

        AwayNote {
            width: parent.width
            playlist: more.uid
        }

        Fila {
            text: App.tr("Remove the list")
            note: App.tr("You can undo it right after")
            kind: "pliega"
            onOpened: more.page.remove(more.playlist)
        }
    }

    FolderDialog {
        id: follow

        onAccepted: App.act("AddFolder", [more.uid, App.path(selectedFolder), true], "Playlists1", reply => applicationWindow().showPassiveNotification(App.tr("The folder has %1 pictures and videos", reply[0])))
    }
}
