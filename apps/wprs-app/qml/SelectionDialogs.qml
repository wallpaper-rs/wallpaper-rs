import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Item {
    id: dialogs

    required property var owner
    readonly property alias importer: importer
    readonly property alias playlist: more.playlist
    readonly property alias tagger: tagger
    readonly property alias remover: remover
    readonly property alias renamer: renamer
    readonly property alias rater: rater
    readonly property alias allower: allower
    readonly property alias rotator: more.rotator
    readonly property int inner: Look.panelChico - 2 * Look.e4

    LibraryAdd {
        id: importer

        owner: dialogs.owner
    }

    RotateDialogs {
        id: more

        owner: dialogs.owner
    }

    PanelChico {
        id: allower

        title: App.tr("Allow wallpapers that flash?")
        text: App.tr("Wallpapers that flash or show fast patterns can trigger seizures in people with photosensitive epilepsy. Allow them only if that is safe for whoever sees your screens; every screen and playlist may then show them.")
        quiet: App.tr("Cancel")
        principal: App.tr("Allow")
        onAccepted: App.act("SetOption", ["playback.allow_flashing", "true"])
    }

    PanelChico {
        id: remover

        readonly property int count: dialogs.owner.uids.length

        title: count === 1 ? App.tr("Remove 1 wallpaper from the library?") : App.tr("Remove %1 wallpapers from the library?", count)
        text: App.tr("The files stay where they are.")
        quiet: App.tr("Cancel")
        principal: App.tr("Remove")
        principalIcon: "trash"
        borra: true
        onAboutToShow: purge.checked = false
        onAccepted: App.act("Remove", [dialogs.owner.uids, purge.visible && purge.checked], "Library1")

        QQC2.CheckBox {
            id: purge

            width: dialogs.inner
            visible: dialogs.owner.items.imported
            text: App.tr("Also delete the imported copies")
        }
    }

    PanelChico {
        id: renamer

        property string from

        title: App.tr("Rename tag")
        quiet: App.tr("Cancel")
        principal: App.tr("Rename")
        first: name
        onAboutToShow: name.text = from
        onAccepted: name.text.trim() !== "" && App.act("RenameTag", [from, name.text.trim()], "Library1")

        QQC2.TextField {
            id: name

            width: dialogs.inner
            Accessible.name: renamer.title
            onAccepted: {
                renamer.accepted();
                renamer.close();
            }
        }
    }

    PanelChico {
        id: rater

        title: App.trc("rating", "Rate")
        quiet: App.tr("Close")
        first: stars

        Stars {
            id: stars

            onRated: count => {
                App.act("SetRating", [dialogs.owner.uids, count], "Library1");
                rater.close();
            }
        }
    }

    PanelChico {
        id: tagger

        title: App.tr("Tags")
        quiet: App.tr("Cancel")
        principal: App.tr("Save")
        first: added
        onAboutToShow: added.text = removed.text = ""
        onAccepted: App.act("SetTags", [dialogs.owner.uids, dialogs.owner.list(added.text), dialogs.owner.list(removed.text)], "Library1")

        QQC2.TextField {
            id: added

            width: dialogs.inner
            placeholderText: App.tr("Add (comma separated):")
            Accessible.name: placeholderText
        }

        QQC2.TextField {
            id: removed

            width: dialogs.inner
            placeholderText: App.tr("Remove:")
            Accessible.name: placeholderText
        }
    }
}
