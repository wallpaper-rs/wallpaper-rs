import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Page {
    id: page

    readonly property bool trouble: catalog.troubled !== "" || installer.failure !== null
    readonly property real bandLow: bar.height + lower.spacing + (trouble ? fallo.implicitHeight : none.visible ? none.implicitHeight : shelf.implicitHeight)
    readonly property real bandHigh: bandLow
    readonly property real upperHeight: upperBlock.visible ? upperBlock.implicitHeight : 0
    readonly property real bandFrom: upperBlock.visible ? Math.min(upper.y, band.y) : band.y
    readonly property Component now: Item {}
    property rect band
    property rect upper
    property bool full
    property bool open
    property int at
    readonly property var pkg: catalog.chosen[at] ?? null
    readonly property alias catalog: catalog
    readonly property alias installer: installer
    readonly property alias upperBlock: upperBlock
    readonly property alias bar: bar
    readonly property alias shelf: shelf
    readonly property alias grid: grid
    readonly property alias fallo: fallo
    readonly property var keyHints: full ? [{
            "keys": ["left", "right", "up", "down"],
            "text": App.tr("Move around")
        }, {
            "keys": ["Esc"],
            "text": App.tr("See the shelf")
        }] : [{
            "keys": ["left", "right"],
            "text": App.tr("Move around")
        }, {
            "keys": ["down"],
            "text": App.tr("See all")
        }]

    function exercise() {
        Qt.createComponent("ExploreCheck.qml").createObject(page, {
            "page": page
        }).exercise();
    }

    function flip() {
        full = !full;
        open = false;
        Qt.callLater(() => page && (page.full ? grid : shelf).forceActiveFocus());
    }

    title: App.tr("Explore")
    padding: 0
    background: null
    actions: uploader.shown ? uploader.actions : []
    onAtChanged: open = false
    onOpenChanged: open && !full && shelf.forceActiveFocus()
    Keys.onEscapePressed: event => {
        event.accepted = full || open;
        if (full)
            flip();
        open = false;
    }

    Browse {
        id: catalog
    }

    Installer {
        id: installer
    }

    ExploreUpper {
        id: upperBlock

        x: page.upper.x + Math.max(0, page.upper.width - width)
        y: page.upper.y
        visible: !page.full && !page.catalog.fresh
        catalog: page.catalog
        installer: page.installer
        publisher: uploader
    }

    Column {
        id: lower

        x: page.full ? 0 : page.band.x
        y: page.full ? 0 : page.band.y
        width: page.full ? page.width : page.band.width
        spacing: Look.e4

        ExploreBar {
            id: bar

            catalog: page.catalog
            full: page.full
            roomy: lower.width >= Look.filasAncho * 2
            onFlipped: page.flip()
        }

        BrowseTrouble {
            id: fallo

            width: Math.min(parent.width, Look.errorGrandeAncho)
            visible: page.trouble
            catalog: page.catalog
            installer: page.installer
            onRepositories: applicationWindow().showPage(8)
        }

        Vacio {
            id: none

            visible: page.catalog.loaded && page.catalog.repos.length === 0
            title: App.tr("No repositories")
            text: App.tr("The bundled examples were removed. Add them again, or add a repository with its link.")
            action: App.tr("Add the examples again")
            actionIcon: "arrow-counter-clockwise"
            onActed: App.act("AddExamples", [], "Repos1")
        }

        Estante {
            id: shelf

            width: parent.width + Look.margen
            visible: !page.trouble && !page.full && !none.visible
            label: App.tr("Explore")
            model: catalog.covers
            movingOf: index => catalog.motion(catalog.chosen[index])
            onWanted: index => catalog.animate(catalog.chosen[index])
            currentIndex: page.at
            onCurrentIndexChanged: page.at = currentIndex
            onActivated: page.pkg && fixed.act()
            Keys.onDownPressed: page.flip()
            actions: Item {
                readonly property bool shown: page.open && page.pkg !== null

                implicitWidth: shown ? Math.max(row.implicitWidth, Look.tapaFoco + Look.e5 + aside.implicitWidth) : row.implicitWidth
                implicitHeight: row.implicitHeight

                BrowseRow {
                    id: row

                    pkg: page.pkg
                    catalog: page.catalog
                    installer: page.installer
                    open: page.open
                    onDetails: page.open = !page.open
                }

                BrowseDetails {
                    id: aside

                    x: Look.tapaFoco + Look.e5
                    visible: parent.shown
                    pkg: page.pkg
                    catalog: page.catalog
                }
            }
        }

        Rejilla {
            id: grid

            width: parent.width
            height: page.height - bar.height - fija.implicitHeight - 2 * lower.spacing
            visible: page.full && !page.trouble
            label: App.tr("All the wallpapers")
            model: catalog.covers
            movingOf: index => catalog.motion(catalog.chosen[index])
            onWanted: index => catalog.animate(catalog.chosen[index])
            currentIndex: page.at
            onCurrentIndexChanged: page.at = currentIndex
            onActivated: page.pkg && fixed.act()
        }

        FichaFija {
            id: fija

            width: parent.width
            visible: grid.visible
            title: page.pkg ? page.pkg.title : ""
            caption: page.pkg ? catalog.cover(page.pkg).caption : ""

            BrowseRow {
                id: fixed

                pkg: page.pkg
                catalog: page.catalog
                installer: page.installer
                folds: false
            }
        }
    }

    Publish {
        id: uploader
    }
}
