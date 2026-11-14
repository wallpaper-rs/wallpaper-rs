import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

QQC2.ScrollView {
    id: panel

    required property var page
    readonly property var host: page.host
    readonly property string levelName: page.level === "more" ? App.tr("More") : ""
    readonly property list<QtObject> actions: [
        Kirigami.Action {
            text: panel.host.item.favorite ? App.tr("Unfavorite") : App.tr("Favorite")
            icon.name: panel.host.item.favorite ? "heart-fill" : "heart"
            onTriggered: App.act("SetFavorite", [[panel.page.opened], !panel.host.item.favorite], "Library1")
        },
        Kirigami.Action {
            text: App.outputs.length > 1 ? App.tr("Set as wallpaper on %1", panel.page.target) : App.tr("Set as wallpaper")
            icon.name: "image"
            visible: !!panel.host.pkg ? panel.host.pkg.kind !== "assets" : true
            onTriggered: panel.page.tools.put(panel.page.opened)
        }
    ]

    contentWidth: availableWidth

    ColumnLayout {
        width: Math.min(panel.availableWidth, Look.tapaFoco * 2)
        spacing: Look.e3

        Foto {
            visible: panel.page.level === "" && panel.host.preview !== "" && !moving.visible
            Layout.preferredWidth: Look.tapaFoco
            Layout.preferredHeight: Math.round(Look.tapaFoco * 9 / 16)
            source: App.url(panel.host.preview)
            cache: false
        }

        AnimatedImage {
            id: moving

            Layout.preferredWidth: Look.tapaFoco
            Layout.preferredHeight: Math.round(Look.tapaFoco * 9 / 16)
            visible: panel.page.level === "" && panel.host.animated !== "" && status === AnimatedImage.Ready
            source: panel.page.level === "" ? panel.host.animated : ""
            playing: visible && !(App.state.on_battery ?? false)
            fillMode: Image.PreserveAspectCrop
            cache: false
        }

        PackageAbout {
            Layout.fillWidth: true
            visible: panel.page.level === "" && !!panel.host.details.about
            info: panel.host.details.about ?? {}
            uid: panel.host.uid
            pkg: panel.host.pkg
        }

        Seccion {
            visible: panel.page.level === ""
            text: App.tr("From its author")
        }

        ItemKin {
            Layout.fillWidth: true
            visible: panel.page.level === ""
            host: panel.host
        }

        Seccion {
            visible: panel.page.level === ""
            text: App.tr("Yours")
        }

        ItemExtras {
            Layout.fillWidth: true
            visible: panel.page.level === ""
            host: panel.host
        }

        Fila {
            id: more

            Layout.fillWidth: true
            visible: panel.page.level === ""
            kind: "pliega"
            text: App.tr("More")
            note: App.tr("Tags, playlists, rating, credits, details, remove")
            onOpened: panel.page.level = "more"
        }

        LibraryMore {
            Layout.fillWidth: true
            visible: panel.page.level === "more"
            page: panel.page
        }
    }
}
