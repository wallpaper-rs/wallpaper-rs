pragma Singleton
import QtQuick

QtObject {
    id: store

    readonly property string marker: "wprs-layers:"
    readonly property bool pasteable: clip !== null || board.canPaste
    property var clip: null
    readonly property TextEdit board: TextEdit {
        visible: false
        textFormat: TextEdit.PlainText
    }

    function recognised(text) {
        if (!text.startsWith(marker))
            return null;
        try {
            const found = JSON.parse(text.slice(marker.length));
            return Array.isArray(found?.layers) && found.layers.length > 0 && typeof found.from === "string" ? found : null;
        } catch (e) {
            return null;
        }
    }

    function share(found) {
        clip = found;
        board.text = marker + JSON.stringify(found);
        board.selectAll();
        board.copy();
    }

    function take() {
        board.text = "";
        board.paste();
        return recognised(board.text) ?? clip;
    }
}
