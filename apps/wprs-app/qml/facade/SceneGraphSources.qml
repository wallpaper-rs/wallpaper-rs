pragma Singleton
import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    property var copied: []
    readonly property int most: 12
    readonly property bool pasteable: copied.length > 0 || board.canPaste
    readonly property TextEdit board: TextEdit {
        visible: false
        textFormat: TextEdit.PlainText
    }

    function share(found) {
        copied = found.picks;
        board.text = found.text;
        board.selectAll();
        board.copy();
    }

    function held() {
        board.text = "";
        board.paste();
        return board.text;
    }

    function tracks(list, chosen) {
        const own = chosen ? (list ?? []).find(t => t.pointer === chosen.pointer) : null;
        const shown = (list ?? []).filter(t => !own || t.layer === own.layer);
        return shown.slice(0, most).map(t => ({
                    "pointer": t.pointer,
                    "label": (t.layer ? t.layer + " · " : "") + SceneWords.label(t.key, t.pointer)
                }));
    }

    function path(row) {
        if (row.kind === "bone")
            return ["bones", row.name, row.channel];
        return [row.kind === "shape" ? "shapes" : "order", row.name];
    }

    function rows(list, mesh, chosen) {
        const keyed = (list ?? []).filter(r => (r.keys ?? []).length > 0);
        const own = chosen?.row ?? null;
        const shown = own ? keyed.filter(r => r.kind === own.kind && r.name === own.name) : keyed;
        return shown.slice(0, most).map(r => ({
                    "layer": mesh.layer,
                    "clip": mesh.clip,
                    "row": path(r),
                    "label": r.channel ? r.name + " · " + SceneWords.label(r.channel) : r.name
                }));
    }
}
