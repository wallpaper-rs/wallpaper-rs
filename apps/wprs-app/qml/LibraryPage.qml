import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Page {
    id: page

    property string query
    property string anchor
    readonly property int cell: Kirigami.Units.gridUnit * 10

    function search() {
        const columns = Math.max(1, Math.floor(grid.width / page.cell));
        anchor = library.uid(Math.max(0, Math.floor(grid.contentY / page.cell)) * columns);
        library.search(filters.text, filters.kind, filters.favorite, filters.tag);
    }

    function keep() {
        const row = anchor ? library.row(anchor) : -1;
        if (row >= 0)
            grid.positionViewAtIndex(row, GridView.Beginning);
        else if (anchor)
            grid.positionViewAtBeginning();
        anchor = "";
    }

    function look() {
        const columns = Math.max(1, Math.floor(grid.width / page.cell));
        const top = Math.max(0, Math.floor(grid.contentY / page.cell));
        const bottom = Math.ceil((grid.contentY + grid.height) / page.cell);
        library.visible(top * columns, Math.min(grid.count, bottom * columns) - 1);
    }

    function applyCurrent() {
        const uid = grid.currentIndex >= 0 ? library.uid(grid.currentIndex) : "";
        if (!uid)
            return;
        if (!grid.currentItem.selected)
            grid.pick(true);
        tools.applyTo(uid);
    }

    function keys() {
        if (grid.count === 0)
            throw new Error("the library shows no item to choose with the keyboard");
        grid.forceActiveFocus(Qt.TabFocusReason);
        App.backend.press(Qt.Key_Space, 0);
        if (library.selected !== 1 || !grid.currentItem?.current)
            throw new Error("the Space key does not choose the focused item of the library");
        App.backend.press(Qt.Key_Return, 0);
        if (!tools.applying)
            throw new Error("the Enter key does not offer to apply the focused item");
        App.backend.press(Qt.Key_Escape, 0);
    }

    function exercise() {
        tools.exercise(library.uid(0));
    }

    title: App.tr("Library")
    padding: 0
    Component.onCompleted: filters.start(query)
    actions: tools.actions

    footer: Column {
        SyncedFiles {
            width: parent.width
        }

        AwayNote {
            width: parent.width
        }
    }

    header: Filters {
        id: filters

        onSearch: page.search()
        onRename: name => tools.renameTag(name)
    }

    QQC2.ScrollView {
        anchors.fill: parent

        GridView {
            id: grid

            function pick(only) {
                if (currentIndex >= 0)
                    library.toggle(currentIndex, only);
            }

            function choose(modifiers) {
                if (currentIndex >= 0)
                    pick(!(modifiers & Qt.ControlModifier) && !currentItem.selected);
            }

            cellWidth: page.cell
            cellHeight: page.cell
            activeFocusOnTab: true
            keyNavigationEnabled: true
            currentIndex: -1
            reuseItems: true
            cacheBuffer: page.cell * 2
            model: ItemModel {
                id: library

                size: grid.Screen.devicePixelRatio > 1 ? 512 : 256
                onReloaded: page.keep()
            }
            delegate: Tile {
                width: page.cell
                height: page.cell
                onPicked: only => library.toggle(index, only)
                onOpened: tools.inspect(uid)
            }
            Keys.onSpacePressed: event => choose(event.modifiers)
            Keys.onReturnPressed: page.applyCurrent()
            Keys.onEnterPressed: page.applyCurrent()
            onActiveFocusChanged: {
                if (activeFocus && currentIndex < 0 && count > 0)
                    currentIndex = 0;
            }
            onContentYChanged: page.look()
            onHeightChanged: page.look()
            onWidthChanged: page.look()
            onCountChanged: page.look()

            Kirigami.PlaceholderMessage {
                anchors.centerIn: parent
                width: parent.width - Kirigami.Units.gridUnit * 4
                visible: grid.count === 0 && !library.loading
                icon.name: library.error ? "data-error" : "folder-pictures"
                text: library.error || App.tr("Nothing here yet")
                explanation: library.error ? "" : App.tr("Add folders in the settings, import files or drop them here.")
            }
        }
    }

    DropArea {
        anchors.fill: parent
        keys: ["text/uri-list"]
        onDropped: drop => {
            const paths = drop.urls.map(url => App.path(url)).filter(p => p);
            if (paths.length > 0)
                App.act("Import", [paths, false], "Library1");
        }
    }

    Actions {
        id: tools

        items: library
    }
}
