import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: dialogs

    required property var owner
    readonly property alias folder: folder
    readonly property alias rotator: rotator
    property string path

    function loadLists() {
        App.query("List", [], "Playlists1", reply => dialogs && (dialogs.owner.playlists = reply.playlists.filter(p => p.query === null)));
    }

    function tell(text) {
        applicationWindow().showPassiveNotification(text);
    }

    Kirigami.Dialog {
        id: folder

        title: App.tr("Add a folder to a playlist")
        standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel
        onOpened: {
            dialogs.path = "";
            keep.checked = false;
            dialogs.loadLists();
            standardButton(Kirigami.Dialog.Ok).enabled = Qt.binding(() => dialogs.path !== "" && target.ready);
        }
        onAccepted: target.resolve(uid => App.act("AddFolder", [uid, dialogs.path, keep.checked], "Playlists1", reply => dialogs && dialogs.tell(App.tr("The folder has %1 pictures and videos", reply[0]))))

        ColumnLayout {
            PlaylistChoice {
                id: target

                Layout.preferredWidth: Kirigami.Units.gridUnit * 22
                playlists: dialogs.owner.playlists
            }

            RowLayout {
                QQC2.Button {
                    text: App.tr("Choose the folder…")
                    icon.name: "folder-open"
                    onClicked: {
                        chooser.open();
                    }
                }

                QQC2.Label {
                    Layout.fillWidth: true
                    elide: Text.ElideMiddle
                    text: dialogs.path || App.tr("No folder chosen")
                }
            }

            QQC2.Switch {
                id: keep

                text: App.tr("Keep following the folder: new files join the playlist")
            }
        }
    }

    Kirigami.Dialog {
        id: rotator

        title: App.tr("Rotate every…")
        standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel
        onOpened: {
            dialogs.path = "";
            follow.checked = false;
            standardButton(Kirigami.Dialog.Ok).enabled = Qt.binding(() => dialogs.owner.uids.length > 0 || dialogs.path !== "");
        }
        onAccepted: App.act("Rotate", [label.text.trim(), dialogs.path ? [] : dialogs.owner.uids, dialogs.path, follow.checked, amount.value + unit.currentValue, ""], "Playlists1", () => dialogs && dialogs.tell(App.tr("The wallpaper changes every %1 %2 from now on.", amount.value, unit.currentText)))

        ColumnLayout {
            QQC2.Label {
                Layout.preferredWidth: Kirigami.Units.gridUnit * 22
                wrapMode: Text.Wrap
                text: dialogs.path ? App.tr("Folder: %1", dialogs.path) : App.tr("Pictures chosen: %1", dialogs.owner.uids.length)
            }

            QQC2.Button {
                text: App.tr("Use a folder instead…")
                icon.name: "folder-open"
                onClicked: {
                    chooser.open();
                }
            }

            QQC2.Switch {
                id: follow

                enabled: dialogs.path !== ""
                text: App.tr("Keep following the folder: new files join the rotation")
            }

            RowLayout {
                QQC2.Label {
                    text: App.tr("Change the wallpaper every")
                }

                QQC2.SpinBox {
                    id: amount

                    from: 1
                    to: 999
                    value: 10
                    editable: true
                }

                QQC2.ComboBox {
                    id: unit

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

            QQC2.TextField {
                id: label

                Layout.fillWidth: true
                placeholderText: App.tr("Name of the playlist (optional)")
            }
        }
    }

    FolderDialog {
        id: chooser

        onAccepted: dialogs.path = App.path(selectedFolder)
    }
}
