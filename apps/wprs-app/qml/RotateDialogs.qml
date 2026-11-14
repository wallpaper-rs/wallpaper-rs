import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Item {
    id: dialogs

    required property var owner
    readonly property alias rotator: rotator
    readonly property alias playlist: playlist
    readonly property int inner: Look.panelChico - 2 * Look.e4
    property string folder

    function loadLists() {
        App.query("List", [], "Playlists1", reply => dialogs && (dialogs.owner.playlists = reply.playlists.filter(p => p.query === null)));
    }

    function tell(text) {
        applicationWindow().showPassiveNotification(text);
    }

    function rotate(every, said) {
        const output = owner.several ? owner.target : "";
        App.act("Rotate", ["", folder !== "" ? [] : owner.uids, folder, folder !== "" && follow.checked, every, output], "Playlists1", () => dialogs && dialogs.tell(App.tr("The wallpaper changes every %1 from now on.", said)));
    }

    function add(uid) {
        App.act("AddItems", [uid, owner.uids], "Playlists1", () => dialogs && dialogs.tell(App.tr("Added to the playlist.")));
    }

    ListaMenu {
        id: rotator

        readonly property var every: ["10m", "30m", "1h", "1d"]

        title: dialogs.owner.uids.length === 1 ? App.tr("Rotate 1 wallpaper") : App.tr("Rotate %1 wallpapers", dialogs.owner.uids.length)
        onAboutToShow: dialogs.folder = ""
        options: [App.tr("10 min"), App.tr("30 min"), App.tr("1 h"), App.tr("Every day")].map(text => ({
                    "text": text,
                    "icon": "clock"
                })).concat([{
                    "separator": true
                }, {
                    "text": App.tr("Other…"),
                    "icon": "dots-three",
                    "note": App.tr("seconds, minutes or hours")
                }, {
                    "text": App.tr("Use a folder instead…"),
                    "icon": "folder-simple"
                }])
        onChosen: index => {
            if (index < every.length)
                return dialogs.rotate(every[index], [App.tr("10 min"), App.tr("30 min"), App.tr("1 h"), App.tr("day")][index]);
            other.opener = rotator.opener;
            if (index === 5)
                return other.open();
            chooser.open();
        }
    }

    PanelChico {
        id: other

        title: dialogs.folder !== "" ? App.tr("Folder: %1", dialogs.folder) : App.tr("Rotate these every…")
        quiet: App.tr("Cancel")
        principal: App.tr("Rotate")
        principalIcon: "clock-clockwise"
        first: amount
        onAccepted: dialogs.rotate(amount.value + unit.currentValue, amount.value + " " + unit.currentText)

        RowLayout {
            width: dialogs.inner
            spacing: Look.e2

            QQC2.SpinBox {
                id: amount

                from: 1
                to: 999
                value: 10
                editable: true
                Accessible.name: App.tr("Change the wallpaper every")
            }

            QQC2.ComboBox {
                id: unit

                Layout.fillWidth: true
                textRole: "text"
                valueRole: "value"
                currentIndex: 1
                model: [
                    {
                        "text": App.tr("seconds"),
                        "value": "s"
                    },
                    {
                        "text": App.tr("minutes"),
                        "value": "m"
                    },
                    {
                        "text": App.tr("hours"),
                        "value": "h"
                    }
                ]
            }
        }

        QQC2.Switch {
            id: follow

            width: dialogs.inner
            visible: dialogs.folder !== ""
            text: App.tr("Keep following the folder: new files join the rotation")
        }
    }

    FolderDialog {
        id: chooser

        onAccepted: {
            dialogs.folder = App.path(selectedFolder);
            follow.checked = false;
            other.open();
        }
    }

    ListaMenu {
        id: playlist

        title: App.tr("Add to playlist")
        onAboutToShow: dialogs.loadLists()
        options: dialogs.owner.playlists.map(list => ({
                    "text": list.name,
                    "icon": "playlist"
                })).concat(dialogs.owner.playlists.length > 0 ? [{
                    "separator": true
                }] : []).concat([{
                    "text": App.tr("A new playlist…"),
                    "icon": "plus"
                }])
        onChosen: index => {
            if (index < dialogs.owner.playlists.length)
                return dialogs.add(dialogs.owner.playlists[index].uid);
            fresh.opener = playlist.opener;
            fresh.open();
        }
    }

    PanelChico {
        id: fresh

        title: App.tr("New playlist")
        quiet: App.tr("Cancel")
        principal: App.tr("Create")
        principalIcon: "plus"
        first: name
        onAboutToShow: name.text = ""
        onAccepted: name.text.trim() !== "" && App.act("Create", [name.text.trim(), ""], "Playlists1", reply => dialogs && dialogs.add(reply[0]))

        QQC2.TextField {
            id: name

            width: dialogs.inner
            placeholderText: App.tr("Name of the new playlist")
            onAccepted: {
                fresh.accepted();
                fresh.close();
            }
        }
    }
}
