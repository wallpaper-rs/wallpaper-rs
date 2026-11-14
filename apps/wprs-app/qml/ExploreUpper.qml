import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Column {
    id: upper

    required property var catalog
    required property Installer installer
    readonly property alias repositories: repositories
    readonly property alias refresh: refresh
    readonly property alias updateAll: updateAll
    readonly property alias more: more
    readonly property alias hidden: hidden

    spacing: Look.e2

    Text {
        anchors.right: parent.right
        text: App.tr("Explore")
        font.family: Look.letraTitulo
        font.pixelSize: Look.tPortada
        font.weight: Font.Bold
        font.letterSpacing: -0.03 * Look.tPortada
        color: Look.escarcha
        Accessible.role: Accessible.Heading
        Accessible.name: text
    }

    Text {
        anchors.right: parent.right
        visible: text !== ""
        text: upper.catalog.repos.map(r => upper.catalog.repoName(r)).join(" · ")
        textFormat: Text.PlainText
        font.family: Look.letraTexto
        font.pixelSize: Look.tPie
        font.weight: Font.Medium
        color: Look.escarcha
    }

    Estado {
        anchors.right: parent.right
        visible: upper.catalog.total > upper.catalog.packages.length
        icon: "info"
        text: App.tr("Showing %1 of %2 wallpapers; refine the search to see the others.", upper.catalog.packages.length, upper.catalog.total)
    }

    QQC2.Button {
        id: hidden

        anchors.right: parent.right
        visible: upper.catalog.hidden > 0
        flat: true
        text: upper.catalog.hidden === 1 ? App.tr("1 hidden by your content filter") : App.tr("%1 hidden by your content filter", upper.catalog.hidden)
        Accessible.description: App.tr("Opens the settings, where the content filter is")
        onClicked: applicationWindow().showPage(5)
    }

    Row {
        anchors.right: parent.right
        topPadding: Look.e4
        spacing: Look.e2

        QQC2.Button {
            id: repositories

            text: App.tr("Repositories…")
            icon.name: "books"
            onClicked: applicationWindow().showPage(8)
        }

        QQC2.Button {
            id: refresh

            text: App.tr("Check for news")
            icon.name: "arrows-clockwise"
            enabled: upper.catalog.repos.length > 0
            onClicked: App.act("Refresh", [""], "Repos1")
        }

        QQC2.Button {
            id: updateAll

            text: App.tr("Update my wallpapers")
            icon.name: "arrow-circle-up"
            enabled: upper.catalog.plain.length > 0
            onClicked: upper.catalog.plain.forEach(p => upper.installer.update(p))
        }

        ExploreMore {
            id: more
        }
    }
}
