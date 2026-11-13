import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: tools

    required property ItemModel items
    property var uids: []
    property var playlists: []
    readonly property bool applying: apply.visible
    readonly property list<QtObject> actions: [
        Kirigami.Action {
            text: App.tr("Import…")
            icon.name: "document-import"
            onTriggered: dialogs.importer.open()
        },
        Kirigami.Action {
            text: App.tr("Rescan all")
            icon.name: "view-refresh"
            onTriggered: App.act("Rescan", [""], "Library1")
        },
        Kirigami.Action {
            text: App.tr("Details")
            icon.name: "documentinfo"
            enabled: tools.items.selected === 1
            onTriggered: tools.inspect(tools.take()[0])
        },
        Kirigami.Action {
            text: App.tr("Apply to…")
            icon.name: "preferences-desktop-wallpaper"
            enabled: tools.items.selected === 1
            onTriggered: apply.show(tools.take()[0])
        },
        Kirigami.Action {
            text: App.tr("Add to playlist…")
            icon.name: "view-media-playlist"
            enabled: tools.items.selected > 0
            onTriggered: {
                tools.take();
                dialogs.playlist.open();
            }
        },
        Kirigami.Action {
            text: App.tr("Rotate every…")
            icon.name: "media-playlist-repeat"
            onTriggered: {
                tools.take();
                dialogs.rotator.open();
            }
        },
        Kirigami.Action {
            text: App.tr("Add a folder to a playlist…")
            icon.name: "folder-add"
            onTriggered: dialogs.folder.open()
        },
        Kirigami.Action {
            text: App.tr("Tags…")
            icon.name: "tag"
            enabled: tools.items.selected > 0
            onTriggered: {
                tools.take();
                dialogs.tagger.open();
            }
        },
        Kirigami.Action {
            text: tools.items.favorite ? App.tr("Unfavorite") : App.tr("Favorite")
            icon.name: "emblem-favorite"
            enabled: tools.items.selected > 0
            onTriggered: App.act("SetFavorite", [tools.take(), !tools.items.favorite], "Library1")
        },
        Kirigami.Action {
            text: App.tr("Rate…")
            icon.name: "rating"
            enabled: tools.items.selected > 0
            onTriggered: {
                tools.take();
                dialogs.rater.open();
            }
        },
        Kirigami.Action {
            text: App.tr("Allow these wallpapers…")
            icon.name: "dialog-ok-apply"
            visible: tools.items.flashing
            onTriggered: {
                tools.take();
                dialogs.allower.open();
            }
        },
        Kirigami.Action {
            text: App.tr("Send files to my other computers")
            icon.name: "folder-cloud"
            enabled: tools.items.selected > 0
            onTriggered: App.act("ShareFiles", [tools.take(), true], "Sync1", () => applicationWindow().showPassiveNotification(App.tr("Sending the files, encrypted, to your server. Your other computers can then download them from the Library or from Account.")), () => applicationWindow().showPassiveNotification(App.tr("The files were not sent. Link this computer to your server with end-to-end encryption in Account first.")))
        },
        Kirigami.Action {
            text: App.tr("Remove…")
            icon.name: "edit-delete"
            enabled: tools.items.selected > 0 && tools.items.removable
            onTriggered: {
                tools.take();
                dialogs.remover.open();
            }
        }
    ]

    function addToPlaylist() {
        take();
        dialogs.playlist.open();
    }

    function rotate() {
        take();
        dialogs.rotator.open();
    }

    function take() {
        uids = JSON.parse(items.selection());
        return uids;
    }

    function inspect(uid) {
        if (uid)
            drawer.show(uid);
    }

    function exercise(uid) {
        if (!uid)
            throw new Error("the library shows no item to open");
        inspect(uid);
        if (!drawer.visible)
            throw new Error("the item drawer did not open");
        let hid = false;
        const note = () => hid = true;
        drawer.aboutToHide.connect(note);
        App.backend.press(Qt.Key_Escape, 0);
        drawer.aboutToHide.disconnect(note);
        if (!hid)
            throw new Error("the Escape key does not close the details panel");
        apply.show(uid);
        shown();
    }

    function shown() {
        const make = (id, kind) => Qt.createComponent("ItemSettings.qml").createObject(tools, {
                "host": {
                    "uid": id,
                    "pkg": {
                        "kind": kind
                    }
                }
            });
        const on = make("01J9Z0X9Z6Q7X4F3K5E1V0R1SC", "scene");
        const off = make("01J9Z0X9Z6Q7X4F3K5E1V0R1ZZ", "scene");
        const image = make("01J9Z0X9Z6Q7X4F3K5E1V0R1AB", "image");
        const found = [on.screens.join(), off.screens.length, on.visible, image.visible];
        [on, off, image].forEach(item => item.destroy());
        if (found.join() !== "HDMI-A-1,0,true,false")
            throw new Error("the details panel does not offer the settings of a wallpaper on a screen: " + found.join());
    }

    function applyTo(uid) {
        if (uid)
            apply.show(uid);
    }

    function renameTag(name) {
        dialogs.renamer.from = name;
        dialogs.renamer.open();
    }

    function list(text) {
        return text.split(",").map(tag => tag.trim()).filter(tag => tag);
    }

    ItemDrawer {
        id: drawer

        parent: tools.QQC2.Overlay.overlay
        removable: tools.items.selected === 1 && tools.items.removable
        onApply: uid => apply.show(uid)
        onRemove: uid => {
            tools.uids = [uid];
            dialogs.remover.open();
        }
    }

    ApplyDialog {
        id: apply
    }

    SelectionDialogs {
        id: dialogs

        owner: tools
    }
}
