import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

QQC2.ScrollView {
    id: panel

    required property var page
    readonly property var host: page.host
    readonly property string levelName: page.level === "more" ? App.tr("More") : page.level === "presets" ? App.tr("My presets") : ""
    readonly property bool first: page.level === "" && !broken.hurt
    readonly property list<QtObject> actions: [
        Kirigami.Action {
            text: panel.host.item.favorite ? App.tr("Unfavorite") : App.tr("Favorite")
            icon.name: panel.host.item.favorite ? "heart-fill" : "heart"
            onTriggered: App.act("SetFavorite", [[panel.page.opened], !panel.host.item.favorite], "Library1")
        }
    ]

    readonly property bool puts: !!host.pkg ? host.pkg.kind !== "assets" : true
    readonly property Component badge: Component {
        Item {
            implicitWidth: Math.round(Look.tapa * 3 / 4)
            implicitHeight: Math.round(implicitWidth * 9 / 16)

            Foto {
                anchors.fill: parent
                visible: !moving.visible
                source: panel.host.preview !== "" ? App.url(panel.host.preview) : ""
                cache: false
            }

            AnimatedImage {
                id: moving

                anchors.fill: parent
                visible: panel.host.animated !== "" && status === AnimatedImage.Ready
                source: panel.host.animated
                playing: visible && !(App.state.on_battery ?? false)
                fillMode: Image.PreserveAspectCrop
                cache: false
            }
        }
    }
    readonly property Component head: Component {
        Row {
            visible: panel.puts
            spacing: Look.e3

            QQC2.Button {
                highlighted: true
                text: App.outputs.length > 1 ? App.tr("Set as wallpaper on %1", App.screen(panel.page.target)) : App.tr("Set as wallpaper")
                icon.name: "image"
                onClicked: panel.put()
            }

            PiecesTecla {
                anchors.verticalCenter: parent.verticalCenter
                text: App.trc("key", "Enter")
            }
        }
    }

    function put() {
        if (puts)
            page.tools.put(page.opened);
    }

    contentWidth: availableWidth
    onVisibleChanged: visible && Qt.callLater(() => panel && panel.forceActiveFocus())
    Keys.onReturnPressed: put()
    Keys.onEnterPressed: put()

    ColumnLayout {
        width: Math.min(panel.availableWidth, Look.tapaFoco * 2)
        spacing: Look.e3

        SceneBroken {
            id: broken

            Layout.fillWidth: true
            page: panel.page
            visible: hurt && panel.page.level === ""
        }

        SceneStill {
            Layout.fillWidth: true
            host: panel.host
            visible: reason !== "" && panel.first
        }

        ItemSettings {
            Layout.fillWidth: true
            host: panel.host
            visible: tunable && panel.first
        }

        Seccion {
            visible: panel.first
            text: App.tr("Yours")
        }

        Fila {
            Layout.fillWidth: true
            visible: panel.first && !!panel.host.pkg && panel.host.pkg.kind !== "assets"
            kind: "pliega"
            text: App.tr("My presets")
            note: App.tr("They work for every scene")
            number: String(presets.all.length)
            onOpened: panel.page.level = "presets"
        }

        Fila {
            id: more

            Layout.fillWidth: true
            visible: panel.first
            kind: "pliega"
            text: App.tr("More")
            note: App.tr("Tags, playlists, rating, credits, details, remove")
            onOpened: panel.page.level = "more"
        }

        ItemPresets {
            id: presets

            Layout.fillWidth: true
            visible: panel.page.level === "presets"
            page: panel.page
        }

        LibraryMore {
            Layout.fillWidth: true
            visible: panel.page.level === "more"
            page: panel.page
        }
    }
}
