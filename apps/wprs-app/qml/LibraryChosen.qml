import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Seleccion {
    id: line

    required property var page
    readonly property var tools: page.tools
    readonly property var items: page.library

    count: page.chosen.length

    QQC2.Button {
        id: rotate

        highlighted: true
        icon.name: "clock-clockwise"
        text: App.tr("Rotate these every…")
        onClicked: line.tools.chosen("rotator", rotate)
    }

    QQC2.Button {
        id: list

        flat: true
        text: App.tr("Playlist")
        onClicked: line.tools.chosen("playlist", list)
    }

    QQC2.Button {
        id: tags

        flat: true
        text: App.tr("Tags")
        onClicked: line.tools.chosen("tagger", tags)
    }

    QQC2.Button {
        id: more

        flat: true
        text: App.tr("More")
        icon.name: "caret-down"
        LayoutMirroring.enabled: true
        onClicked: menu.open()
    }

    QQC2.Button {
        flat: true
        text: App.tr("Stop choosing")
        onClicked: line.page.stop()
    }

    PiecesTecla {
        text: App.trc("key", "Esc")
    }

    ListaMenu {
        id: menu

        opener: more
        title: App.tr("More")
        options: [{
                "text": line.items.favorite ? App.tr("Unfavorite") : App.tr("Favorite"),
                "icon": line.items.favorite ? "heart" : "heart-fill"
            }, {
                "text": App.tr("Rate…"),
                "icon": "star"
            }, {
                "text": App.tr("Allow these wallpapers…"),
                "icon": "warning",
                "off": !line.items.flashing
            }, {
                "text": App.tr("Send files to my other computers"),
                "icon": "cloud-arrow-up"
            }, {
                "separator": true
            }, {
                "text": App.tr("Remove…"),
                "icon": "trash",
                "tone": "coral",
                "off": !line.items.removable
            }]
        onChosen: index => {
            if (index === 0)
                line.tools.favorite(!line.items.favorite);
            else if (index === 1)
                line.tools.chosen("rater", more);
            else if (index === 2)
                line.tools.chosen("allower", more);
            else if (index === 3)
                line.tools.share();
            else if (index === 5)
                line.tools.chosen("remover", more);
        }
    }
}
