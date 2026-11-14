import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: dialogs

    required property var owner
    readonly property alias importer: importer
    readonly property alias playlist: playlist
    readonly property alias tagger: tagger
    readonly property alias remover: remover
    readonly property alias renamer: renamer
    readonly property alias rater: rater
    readonly property alias allower: allower
    readonly property alias folder: more.folder
    readonly property alias rotator: more.rotator

    Kirigami.PromptDialog {
        id: allower

        title: App.tr("Allow wallpapers that flash?")
        subtitle: App.tr("Wallpapers that flash or show fast patterns can trigger seizures in people with photosensitive epilepsy. Allow them only if that is safe for whoever sees your screens; every screen and playlist may then show them.")
        standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel
        onAccepted: App.act("SetOption", ["playback.allow_flashing", "true"])
    }

    Kirigami.Dialog {
        id: importer

        title: App.tr("Import files")
        standardButtons: Kirigami.Dialog.Cancel
        customFooterActions: [
            Kirigami.Action {
                text: App.tr("Choose files…")
                icon.name: "document-open"
                onTriggered: files.open()
            }
        ]

        ColumnLayout {
            Layout.preferredWidth: Kirigami.Units.gridUnit * 28

            QQC2.Label {
                Layout.fillWidth: true
                wrapMode: Text.Wrap
                text: App.tr("Type or paste the full paths of the files to add, one per line, or choose them with the button below.")
            }

            Area {
                id: typed

                Layout.fillWidth: true
                Layout.preferredHeight: Kirigami.Units.gridUnit * 6
                placeholderText: App.tr("One path per line")
                Accessible.name: App.tr("Paths to import")
            }

            QQC2.Button {
                icon.name: "document-import"
                text: App.tr("Import these paths")
                enabled: typed.text.trim() !== ""
                onClicked: {
                    App.act("Import", [typed.text.split("\n").map(line => line.trim()).filter(line => line !== "").map(line => line.startsWith("file://") ? decodeURIComponent(line.slice(7)) : line), copy.checked], "Library1");
                    typed.text = "";
                    importer.close();
                }
            }

            QQC2.Switch {
                id: copy

                text: App.tr("Copy into the library")
            }
        }
    }

    FileDialog {
        id: files

        fileMode: FileDialog.OpenFiles
        onAccepted: {
            App.act("Import", [Array.from(selectedFiles, file => App.path(file)), copy.checked], "Library1");
            importer.close();
        }
    }

    RotateDialogs {
        id: more

        owner: dialogs.owner
    }

    Kirigami.Dialog {
        id: playlist

        title: App.tr("Add to playlist")
        standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel
        onOpened: {
            more.loadLists();
            standardButton(Kirigami.Dialog.Ok).enabled = Qt.binding(() => chooser.ready);
        }
        onAccepted: chooser.resolve(uid => App.act("AddItems", [uid, dialogs.owner.uids], "Playlists1"))

        PlaylistChoice {
            id: chooser

            playlists: dialogs.owner.playlists
        }
    }

    Kirigami.Dialog {
        id: tagger

        title: App.tr("Tags")
        standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel
        onAccepted: App.act("SetTags", [dialogs.owner.uids, dialogs.owner.list(added.text), dialogs.owner.list(removed.text)], "Library1")

        Kirigami.FormLayout {
            QQC2.TextField {
                id: added

                Kirigami.FormData.label: App.tr("Add (comma separated):")
            }

            QQC2.TextField {
                id: removed

                Kirigami.FormData.label: App.tr("Remove:")
            }
        }
    }

    Kirigami.Dialog {
        id: remover

        title: App.tr("Remove from the library")
        standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel
        onAccepted: App.act("Remove", [dialogs.owner.uids, purge.visible && purge.checked], "Library1")

        ColumnLayout {
            QQC2.Label {
                text: App.tr("Items to remove: %1", dialogs.owner.uids.length)
            }

            QQC2.CheckBox {
                id: purge

                visible: dialogs.owner.items.imported
                text: App.tr("Also delete the imported copies")
            }
        }
    }

    Kirigami.Dialog {
        id: renamer

        property string from

        title: App.tr("Rename tag")
        standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel
        onAccepted: App.act("RenameTag", [from, name.text], "Library1")

        QQC2.TextField {
            id: name

            text: renamer.from
        }
    }

    Kirigami.Dialog {
        id: rater

        title: App.trc("rating", "Rate")
        standardButtons: Kirigami.Dialog.Close

        Stars {
            onRated: stars => {
                App.act("SetRating", [dialogs.owner.uids, stars], "Library1");
                rater.close();
            }
        }
    }
}
