import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: more

    required property var page
    readonly property var host: page.host
    readonly property var pkg: host.pkg

    spacing: Look.e3

    QQC2.TextField {
        Layout.fillWidth: true
        text: more.host.item.title ?? ""
        onAccepted: App.act("SetTitle", [more.host.uid, text], "Library1")
    }

    Stars {
        rating: more.host.item.rating ?? 0
        onRated: stars => App.act("SetRating", [[more.host.uid], stars], "Library1")
    }

    ItemFacts {
        Layout.fillWidth: true
        host: more.host
    }

    QQC2.Switch {
        Layout.fillWidth: true
        visible: !!more.pkg && more.pkg.kind === "scene" && more.pkg.photosensitive !== true
        text: App.tr("Keep moving when the desktop asks for reduced motion")
        checked: !!more.pkg && (more.pkg.confirmed ?? []).includes("motion")
        onToggled: {
            App.act(checked ? "Confirm" : "Unconfirm", [[more.host.uid], "motion"], "Library1");
            checked = Qt.binding(() => !!more.pkg && (more.pkg.confirmed ?? []).includes("motion"));
        }
    }

    WebPrivacy {
        Layout.fillWidth: true
        uid: more.host.uid
        pkg: more.pkg
    }

    QQC2.Button {
        visible: !!more.pkg && more.pkg.kind === "web"
        enabled: !!more.pkg && !more.pkg.unshown
        text: App.tr("Try the page and its lights…")
        icon.name: "lightbulb"
        onClicked: lights.start(more.host.uid)
    }

    PageLights {
        id: lights
    }

    RowLayout {
        spacing: Look.e3

        QQC2.Button {
            text: App.tr("Edit…")
            icon.name: "pencil-simple"
            visible: more.host.item.kind === "image" && !more.host.item.missing
            onClicked: App.edit(more.host.item)
        }

        QQC2.Button {
            id: remove

            flat: true
            text: App.tr("Remove from the library…")
            icon.name: "trash"
            onClicked: confirm.open()
        }
    }

    PanelChico {
        id: confirm

        opener: remove
        title: App.tr("Remove “%1” from the library?", more.host.item.title ?? "")
        text: App.tr("The file stays where it is.")
        principal: App.tr("Remove")
        principalIcon: "trash"
        borra: true
        onAccepted: {
            App.act("Remove", [[more.host.uid], false], "Library1");
            more.page.routeBack();
            more.page.routeBack();
        }
    }
}
