import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

PanelChico {
    id: creator

    required property var page

    title: App.tr("New playlist")
    text: App.tr("Leave the search empty to choose its wallpapers by hand.")
    principal: App.tr("Create")
    principalIcon: "plus"
    first: name
    onAboutToShow: {
        name.text = "";
        search.text = creator.page.query;
    }
    onAccepted: name.text.trim() !== "" && App.act("Create", [name.text.trim(), search.text.trim()], "Playlists1", made => creator && (creator.page.opened = made[0]))
    foot: [
        Column {
            spacing: Look.e2

            QQC2.TextField {
                id: name

                width: Look.panelChico - 2 * Look.e4
                placeholderText: App.tr("Name")
            }

            QQC2.TextField {
                id: search

                width: Look.panelChico - 2 * Look.e4
                placeholderText: App.tr("Search, for example fav:yes")
            }
        }
    ]
}
