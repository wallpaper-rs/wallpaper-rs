import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function expect(ok, what) {
        if (!ok)
            throw new Error("try: " + what);
    }

    function value(panel, key) {
        return panel.fields.find(f => f.key === key)?.value;
    }

    function exercise() {
        page.change("/manifest/properties/try_speed", {
            "type": "slider",
            "min": 0,
            "max": 10,
            "step": 1,
            "default": 2,
            "label": "Speed"
        });
        page.change("/manifest/presets", [{
                "id": "fast",
                "label": "Fast",
                "values": {
                    "try_speed": 9
                }
            }]);
        const panel = Qt.createComponent("SceneTry.qml").createObject(page, {
            "page": page
        });
        expect(value(panel, "try_speed") === 2 && panel.presets.length === 1, "the panel does not list the property and its preset");
        SceneLook.tryPreset(page, "fast");
        expect(value(panel, "try_speed") === 9 && JSON.parse(page.editor.look).preset === "fast", "a preset was not tried");
        SceneLook.tryValue(page, "try_speed", 4);
        expect(value(panel, "try_speed") === 4 && JSON.parse(page.editor.look).values.try_speed === 4, "a value was not tried");
        expect(JSON.parse(page.editor.value("/manifest/properties/try_speed")).default === 2, "trying a value changed the package");
        SceneLook.untry(page);
        const look = JSON.parse(page.editor.look);
        expect(value(panel, "try_speed") === 2 && look.values === undefined && look.preset === undefined, "the defaults did not come back");
        panel.destroy();
        page.run("undo");
        page.run("undo");
    }
}
