import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var page

    function same(found, wanted, what) {
        if (found !== wanted)
            throw new Error("fields: " + what + ": " + JSON.stringify(found) + " instead of " + JSON.stringify(wanted));
    }

    function color() {
        const picked = [];
        const swatch = Qt.createComponent("SceneColor.qml").createObject(page, {
                "hex": "#112233",
                "width": 300,
                "height": 40,
                "z": 1000
            });
        swatch.picked.connect(hex => picked.push(hex));
        same(swatch.normalized("F80"), "#ff8800", "a short hex color");
        same(swatch.normalized("#12345"), "", "a malformed hex color");
        same(swatch.normalized("#11223344"), "#11223344", "a hex color with transparency");
        same(swatch.normalized("#112233ff"), "#112233", "an opaque hex color");
        swatch.field.forceActiveFocus();
        swatch.field.text = "#ff8800";
        App.backend.press(Qt.Key_Return, 0);
        same(picked.join(), "#ff8800", "the Enter key in the hex field");
        swatch.hex = "#ff8800";
        swatch.field.text = "00ff00";
        swatch.field.focus = false;
        same(picked.join(), "#ff8800,#00ff00", "leaving the hex field");
        swatch.field.text = "nonsense";
        swatch.field.editingFinished();
        same(picked.length, 2, "a malformed hex color is not applied");
        same(swatch.field.text, "#ff8800", "a malformed hex color is put back");
        swatch.destroy();
    }

    function lines() {
        const area = Qt.createComponent("Area.qml").createObject(page, {
                "enterApplies": true,
                "text": "one",
                "width": 300,
                "height": 80,
                "z": 1000
            });
        const sent = [];
        area.editingFinished.connect(() => sent.push(area.text));
        area.forceActiveFocus();
        area.cursorPosition = 3;
        App.backend.press(Qt.Key_Return, 0);
        same(area.text, "one", "the Enter key in a text field adds no line break");
        same(sent.join(), "one", "the Enter key in a text field applies it");
        App.backend.press(Qt.Key_Return, Qt.ShiftModifier);
        same(area.text, "one\n", "Shift+Enter starts a new line");
        area.text = "one";
        area.cursorPosition = 3;
        App.backend.press(Qt.Key_Tab, 0);
        same(area.text, "one", "the Tab key in a text field adds no tab character");
        area.destroy();
    }

    function effects() {
        page.run("add_layer", {
                "kind": "solid",
                "id": "tinted"
            });
        const layer = page.view.layers.find(item => item.id === "tinted");
        const count = () => {
            const list = JSON.parse(page.editor.value(layer.pointer + "/effects") || "null");
            return Array.isArray(list) ? list.length : 0;
        };
        page.editor.select(layer.pointer);
        const field = page.fields.find(item => item.key === "effects");
        if (!field)
            throw new Error("fields: a layer shows no list of effects");
        const input = Qt.createComponent("SceneInput.qml").createObject(page, {
                "page": page,
                "field": field,
                "width": 400,
                "height": 100,
                "z": 1000
            });
        const gather = (item, out) => {
            out.push(item);
            item.children.forEach(child => gather(child, out));
            return out;
        };
        const add = gather(input, []).find(item => item.icon !== undefined && item.icon.name === "list-add" && item.text === App.tr("Add"));
        if (!add)
            throw new Error("fields: the list of effects has no Add button");
        add.clicked();
        same(count(), 0, "adding an effect from the list creates one before asking which");
        same(page.editor.error, "", "the error line after adding an effect from the list");
        input.destroy();
        page.run("remove_layer", {
                "id": "tinted"
            });
    }

    function pairs() {
        page.run("add_layer", {
                "kind": "solid",
                "id": "placed"
            });
        const layer = page.view.layers.find(item => item.id === "placed");
        page.editor.select(layer.pointer);
        const field = page.fields.find(item => item.key === "position" && item.widget === "vec2");
        if (!field)
            throw new Error("fields: a layer shows no position");
        const before = page.editor.value(field.pointer);
        const input = Qt.createComponent("SceneInput.qml").createObject(page, {
                "page": page,
                "field": field,
                "width": 400,
                "height": 100,
                "z": 1000
            });
        const gather = (item, out) => {
            out.push(item);
            item.children.forEach(child => gather(child, out));
            return out;
        };
        const boxes = gather(input, []).filter(item => item.typed !== undefined && item.textEdited !== undefined);
        same(boxes.length, 2, "the text boxes of a position");
        boxes[0].forceActiveFocus();
        boxes[0].editingFinished();
        same(page.editor.value(field.pointer), before, "a click into a position that changes nothing");
        boxes[0].text = "12";
        boxes[0].textEdited();
        boxes[0].editingFinished();
        const after = JSON.parse(page.editor.value(field.pointer) || "null");
        same(Array.isArray(after) ? after[0] : after, 12, "a position typed into its text box");
        input.destroy();
        page.run("remove_layer", {
                "id": "placed"
            });
    }

    function exercise() {
        color();
        lines();
        effects();
        pairs();
    }
}
