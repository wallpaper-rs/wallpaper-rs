import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: broken

    required property var page
    readonly property var host: page.host
    readonly property var entry: page.entry ?? {}
    readonly property bool hurt: host.item.missing === true || entry.missing === true || String(entry.broken ?? "") !== ""
    readonly property string title: host.item.title ?? entry.title ?? ""
    readonly property string from: host.repo
    readonly property bool scene: !!host.pkg && host.pkg.kind === "scene"

    function reinstall() {
        const pkg = host.pkg;
        App.act("Install", [host.details.installed.repo_id + "/" + pkg.id + "@" + pkg.version, false, false], "Repos1");
    }

    visible: hurt
    spacing: Look.e3

    Fallo {
        Layout.fillWidth: true
        big: true
        title: broken.scene ? App.tr("Could not open this scene") : App.tr("Could not open this wallpaper")
        text: broken.from !== "" ? App.tr("The file of “%1” is damaged or incomplete. It came from “%2”: you can install it again from there. Your wallpaper did not change.", broken.title, broken.from) : App.tr("The file of “%1” is damaged or incomplete. Your wallpaper did not change.", broken.title)
        details: String(broken.entry.broken ?? "")
        action: broken.from !== "" && !!broken.host.pkg ? App.tr("Install it again") : ""
        onActed: broken.reinstall()
    }

    RowLayout {
        spacing: Look.e2

        QQC2.Button {
            icon.name: "folder-open"
            text: App.tr("Show in folder")
            onClicked: broken.page.tools.folder(broken.host.uid)
        }

        QQC2.Button {
            id: remove

            flat: true
            icon.name: "trash"
            text: App.tr("Remove from the library…")
            onClicked: confirm.open()
        }
    }

    PanelChico {
        id: confirm

        opener: remove
        title: App.tr("Remove “%1” from your library?", broken.title)
        text: broken.from !== "" ? App.tr("It leaves your library. If you want it later, you can install it again from “%1”.", broken.from) : App.tr("It leaves your library.")
        quiet: App.tr("Cancel")
        principal: App.tr("Remove")
        principalIcon: "trash"
        borra: true
        onAccepted: {
            App.act("Remove", [[broken.host.uid], false], "Library1");
            broken.page.routeBack();
        }
    }
}
