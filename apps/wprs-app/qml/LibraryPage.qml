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
        if (grid.count > 1) {
            App.backend.press(Qt.Key_Right, Qt.ShiftModifier);
            if (library.selected !== 2)
                throw new Error("Shift with an arrow key does not choose the items between");
            App.backend.press(Qt.Key_A, Qt.ControlModifier);
            if (library.selected !== grid.count)
                throw new Error("Ctrl+A does not choose every item of the library");
            App.backend.press(Qt.Key_Escape, 0);
            if (library.selected !== 0)
                throw new Error("Escape does not clear the choice of the library");
            App.backend.press(Qt.Key_Space, 0);
        }
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
        SelectionBar {
            width: parent.width
            items: library
            tools: tools
            visible: grid.count > 0
        }

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
                if (currentIndex < 0)
                    return;
                if (modifiers & Qt.ShiftModifier)
                    library.range(currentIndex, !!(modifiers & Qt.ControlModifier));
                else
                    pick(!(modifiers & Qt.ControlModifier) && !currentItem.selected);
            }

            function pickWith(row, modifiers) {
                currentIndex = row;
                if (modifiers & Qt.ShiftModifier)
                    library.range(row, !!(modifiers & Qt.ControlModifier));
                else
                    library.toggle(row, !(modifiers & Qt.ControlModifier));
            }

            property bool extending

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
                onPicked: modifiers => grid.pickWith(index, modifiers)
                onOpened: tools.inspect(uid)
            }
            Keys.onSpacePressed: event => choose(event.modifiers)
            Keys.onEscapePressed: library.everything(false)
            Keys.onPressed: event => {
                const moves = [Qt.Key_Left, Qt.Key_Right, Qt.Key_Up, Qt.Key_Down, Qt.Key_Home, Qt.Key_End, Qt.Key_PageUp, Qt.Key_PageDown];
                if (event.key === Qt.Key_A && (event.modifiers & Qt.ControlModifier)) {
                    library.everything(true);
                    event.accepted = true;
                } else {
                    extending = !!(event.modifiers & Qt.ShiftModifier) && moves.includes(event.key);
                }
            }
            onCurrentIndexChanged: {
                if (extending && currentIndex >= 0)
                    library.range(currentIndex, false);
                extending = false;
            }
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

    LibraryDrop {
        anchors.fill: parent
    }

    Actions {
        id: tools

        items: library
    }
}
