import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Mandos {
    id: bar

    required property string screen
    required property var info
    required property string item
    property bool favorite: false
    readonly property bool paused: App.state.paused === true
    readonly property bool muted: App.state.muted === true
    readonly property var fits: App.schema.fits ?? ["fill", "fit", "stretch", "center", "tile"]
    readonly property alias pause: pause

    label: App.tr("What is on your desktop")

    Mando {
        id: pause

        first: true
        icon: bar.paused ? "play-fill" : "pause-fill"
        text: bar.paused ? App.tr("Resume") : App.tr("Pause")
        onClicked: App.act("SetPaused", [!bar.paused])
    }

    Mando {
        visible: (bar.info.sound ?? "") !== ""
        activeFocusOnTab: false
        text: App.tr("Volume")

        QQC2.ToolButton {
            icon.name: bar.muted ? "speaker-slash" : "speaker-high"
            text: App.tr("Mute everything")
            display: QQC2.AbstractButton.IconOnly
            Accessible.name: text
            QQC2.ToolTip.text: text
            QQC2.ToolTip.visible: hovered
            onClicked: App.act("SetMuted", [!bar.muted])
        }

        QQC2.Slider {
            id: volume

            width: Look.e8 + Look.e7
            from: 0
            to: 1
            value: App.state.volume ?? 0
            Accessible.name: App.tr("Overall volume")
            onMoved: debounce.restart()
        }
    }

    Mando {
        visible: bar.item !== ""
        icon: bar.favorite ? "heart-fill" : "heart"
        active: bar.favorite
        text: App.tr("Favorite")
        onClicked: App.act("SetFavorite", [[bar.item], !bar.favorite], "Library1", () => bar && (bar.favorite = !bar.favorite))
    }

    Mando {
        id: fit

        icon: "frame-corners"
        text: App.tr("Fit setting")
        onClicked: fitMenu.open()
    }

    Mando {
        id: filter

        icon: "sliders-horizontal"
        active: !!bar.info.filter
        text: App.tr("Filter")
        onClicked: filterPanel.open()
    }

    Timer {
        id: debounce

        interval: 200
        onTriggered: {
            App.act("SetVolume", [volume.value]);
            App.act("SetMuted", [false]);
        }
    }

    ListaMenu {
        id: fitMenu

        opener: fit
        title: App.tr("Fit setting")
        options: bar.fits.map(name => ({
                    "text": App.value(name)
                }))
        current: Math.max(0, bar.fits.indexOf(bar.info.fit ?? "fill"))
        onChosen: index => {
            close();
            App.act("SetWallpaper", ["", bar.screen, bar.fits[index]]);
        }
    }

    PanelChico {
        id: filterPanel

        opener: filter
        title: App.tr("Filter")

        OutputFilter {
            width: parent.width
            host: ({
                    "daemon": App,
                    "name": bar.screen,
                    "info": bar.info
                })
        }
    }
}
