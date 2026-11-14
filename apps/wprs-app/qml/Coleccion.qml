import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    property var model
    property string label
    property string pictureRole: "picture"
    property string picturesRole: "pictures"
    property string nameRole: "name"
    property string captionRole: "caption"
    property string noteRole: "note"
    property string noteIconRole: "noteIcon"
    property string noteToneRole: "noteTone"
    property string inUseRole: "inUse"
    property string usedOnRole: "usedOn"
    property string offRole: "off"
    property bool choosing
    property var chosen: []
    property var movingOf: null
    readonly property string currentName: view.currentItem?.name ?? ""
    readonly property string currentCaption: view.currentItem?.caption ?? ""
    property Item view
    property int from: -1

    signal activated(int index)
    signal toggled(int index)
    signal extended(int from, int to)
    signal wanted(int index)

    function role(entry, name) {
        const found = entry[name];
        return found !== undefined ? found : entry.modelData ? entry.modelData[name] : undefined;
    }

    function tap(index, modifiers) {
        const before = view.currentIndex;
        view.currentIndex = index;
        view.forceActiveFocus();
        if (modifiers & Qt.ShiftModifier)
            extended(before, index);
        else if (choosing || modifiers & Qt.ControlModifier)
            toggled(index);
    }

    function press(event) {
        const moves = [Qt.Key_Left, Qt.Key_Right, Qt.Key_Up, Qt.Key_Down, Qt.Key_Home, Qt.Key_End, Qt.Key_PageUp, Qt.Key_PageDown];
        from = event.modifiers & Qt.ShiftModifier && moves.includes(event.key) ? view.currentIndex : -1;
    }

    Connections {
        target: view

        function onCurrentIndexChanged() {
            const start = from;
            from = -1;
            if (start >= 0 && view.currentIndex >= 0 && start !== view.currentIndex)
                extended(start, view.currentIndex);
        }
    }

    Keys.onReturnPressed: activated(view.currentIndex)
    Keys.onEnterPressed: activated(view.currentIndex)
    Keys.onSpacePressed: event => {
        event.accepted = choosing;
        if (choosing)
            toggled(view.currentIndex);
    }
}
