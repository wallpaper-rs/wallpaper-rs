import QtQuick
import QtQuick.Dialogs
import io.github.wallpaper_rs.app

FocusScope {
    id: part

    required property var page
    property var chosen: null
    property Item opener: null

    function rotate(source) {
        const query = "source:" + source.uid;
        const start = uid => App.act("Start", [uid, ""], "Playlists1");
        const have = page.playlists.find(list => (list.query || "").includes(query));
        if (have)
            return start(have.uid);
        App.act("Create", [SettingsFolder.name(source), query], "Playlists1", reply => part && start(reply[0]));
    }

    function choose(source, row) {
        chosen = source;
        opener = row;
        menu.opener = row;
        menu.open();
    }

    implicitHeight: filas.height

    Filas {
        id: filas

        Repeater {
            model: part.page.sources

            Fila {
                id: folder

                required property var modelData

                objectName: "dynamic"
                width: parent.width
                text: SettingsFolder.name(modelData)
                note: SettingsFolder.note(modelData)
                kind: "pliega"
                value: modelData.error ? App.tr("Not found") : App.tr("Up to date")
                status: modelData.error ? App.tr("This folder cannot be found") : ""
                statusTone: "coral"
                explain: modelData.error ? App.tr("It may have been moved or be on a disk that is not connected now. Its %1 wallpapers stay in your library until you decide.", modelData.items ?? 0) : String(modelData.path ?? "")
                onOpened: part.choose(modelData, folder)
            }
        }

        Fila {
            text: App.tr("Add a folder…")
            kind: "pliega"
            onOpened: dialog.open()
        }

        Fila {
            id: recursive

            text: App.tr("Include subfolders")
            note: App.tr("For the next folder you add")
            kind: "llave"
            checked: true
            value: checked ? App.tr("Yes") : App.tr("No")
        }
    }

    ListaMenu {
        id: menu

        options: [
            {
                "text": App.tr("Check it again"),
                "icon": "arrows-clockwise"
            },
            {
                "text": App.tr("Rotate these photos"),
                "icon": "play"
            },
            {
                "text": App.tr("Remove it"),
                "icon": "trash",
                "tone": "coral",
                "off": !part.chosen || !!part.chosen.provider
            }
        ]
        onChosen: index => {
            const source = part.chosen;
            if (!source)
                return;
            if (index === 0)
                App.act("Rescan", [source.uid], "Library1");
            else if (index === 1)
                part.rotate(source);
            else
                Qt.callLater(() => confirm.open());
        }
    }

    PanelChico {
        id: confirm

        opener: part.opener
        title: part.chosen ? App.tr("Remove “%1” from the library?", SettingsFolder.name(part.chosen)) : ""
        text: App.tr("Its wallpapers leave your library with their ratings and tags. The files in the folder are not deleted.")
        quiet: App.tr("Cancel")
        principal: App.tr("Remove")
        principalIcon: "trash"
        borra: true
        onAccepted: App.act("RemoveSource", [part.chosen.uid], "Library1")
    }

    FolderDialog {
        id: dialog

        onAccepted: App.act("AddSource", [App.path(selectedFolder), recursive.checked], "Library1")
    }
}
