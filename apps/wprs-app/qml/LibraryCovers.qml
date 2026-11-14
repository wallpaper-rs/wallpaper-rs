import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: covers

    required property var page
    readonly property bool all: page.all
    readonly property Coleccion view: all ? grid : shelf
    readonly property alias shelf: shelf
    readonly property alias grid: grid
    readonly property var hints: page.choosing ? [{
            "keys": ["Esc"],
            "text": App.tr("Stop choosing")
        }] : covers.all ? [{
            "keys": ["left", "right", "up", "down"],
            "text": App.tr("Move around")
        }, {
            "keys": ["Esc"],
            "text": App.tr("Show shelf")
        }] : [{
            "keys": ["left", "right"],
            "text": App.tr("Move around")
        }, {
            "keys": ["down"],
            "text": App.tr("See all")
        }, {
            "keys": ["Esc"],
            "text": App.tr("Menu")
        }]

    function look() {
        const count = covers.page.library.total;
        if (count === 0)
            return;
        if (all) {
            const columns = grid.columns;
            const cell = Math.max(1, grid.view.cellHeight);
            const top = Math.max(0, Math.floor(grid.view.contentY / cell));
            const bottom = Math.ceil((grid.view.contentY + grid.view.height) / cell);
            covers.page.library.visible(top * columns, Math.min(count, bottom * columns) - 1);
        } else {
            const at = shelf.view.indexAt(shelf.view.contentX + Look.tapa / 2, shelf.view.height - Look.e5);
            const first = Math.max(0, (at >= 0 ? at : shelf.currentIndex) - 2);
            covers.page.library.visible(first, Math.min(count - 1, first + Math.ceil(shelf.width / Look.tapa) + 4));
        }
    }

    function extend(from, to) {
        if (covers.page.chosen.length === 0)
            covers.page.library.toggle(from, true);
        covers.page.choosing = true;
        covers.page.library.range(to, false);
        covers.page.sync();
    }

    implicitHeight: all ? grid.height + fixed.implicitHeight + Look.e5 : shelf.implicitHeight
    onAllChanged: {
        if (all && grid.currentIndex < 0)
            grid.currentIndex = Math.max(0, shelf.currentIndex);
        Qt.callLater(look);
        Qt.callLater(() => covers && covers.view.forceActiveFocus());
    }
    Keys.onDownPressed: event => {
        event.accepted = !covers.page.all;
        covers.page.all = true;
    }
    Keys.onEscapePressed: event => {
        event.accepted = covers.page.choosing || covers.page.all;
        if (covers.page.choosing)
            covers.page.stop();
        else if (covers.page.all)
            covers.page.all = false;
    }
    Keys.onPressed: event => {
        if (event.key === Qt.Key_A && event.modifiers & Qt.ControlModifier) {
            covers.page.choosing = true;
            covers.page.library.everything(true);
            covers.page.sync();
            event.accepted = true;
        }
    }

    Estante {
        id: shelf

        visible: !covers.all
        width: parent.width
        height: implicitHeight
        focus: !covers.all
        label: App.tr("Your library")
        model: covers.page.library
        pictureRole: "thumb"
        nameRole: "title"
        choosing: covers.page.choosing
        chosen: covers.page.chosen
        actions: LibraryCard {
            page: covers.page
        }
        selection: LibraryChosen {
            page: covers.page
        }
        onActivated: covers.page.activate()
        onToggled: index => covers.page.toggle(index)
        onExtended: (from, to) => covers.extend(from, to)
        onCurrentIndexChanged: Qt.callLater(covers.look)
    }

    Connections {
        target: shelf.view

        function onContentXChanged() {
            covers.look();
        }
    }

    Rejilla {
        id: grid

        visible: covers.all
        width: parent.width
        height: Math.max(Look.tapaRejilla, covers.page.height * 0.62)
        focus: covers.all
        label: App.tr("All your wallpapers")
        model: covers.page.library
        pictureRole: "thumb"
        nameRole: "title"
        choosing: covers.page.choosing
        chosen: covers.page.chosen
        onActivated: covers.page.activate()
        onToggled: index => covers.page.toggle(index)
        onExtended: (from, to) => covers.extend(from, to)
    }

    Connections {
        target: grid.view

        function onContentYChanged() {
            covers.look();
        }

        function onHeightChanged() {
            covers.look();
        }
    }

    Loader {
        id: fixed

        visible: covers.all
        active: covers.all
        y: grid.height + Look.e5
        width: parent.width
        sourceComponent: covers.page.choosing && covers.page.chosen.length > 0 ? chosenLine : litCard
    }

    Component {
        id: chosenLine

        FichaFija {
            LibraryChosen {
                page: covers.page
            }
        }
    }

    Component {
        id: litCard

        FichaFija {
            title: grid.currentName
            caption: grid.currentCaption

            LibraryCard {
                page: covers.page
            }
        }
    }
}
