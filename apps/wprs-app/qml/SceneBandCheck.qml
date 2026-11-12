import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function exercise() {
        const picker = Qt.createComponent("SceneBandPicker.qml").createObject(page, {
                "page": page
            });
        if (picker.table.bands.length !== 16)
            throw new Error("bands: the strip has " + picker.table.bands.length + " bands");
        picker.mark(2, false);
        picker.mark(5, true);
        if (JSON.stringify(picker.reply.source) !== "{\"audio\":[2,5],\"of\":16}")
            throw new Error("bands: a range gave " + JSON.stringify(picker.reply));
        picker.reduce = "max";
        picker.channel = "left";
        const kept = picker.reply.source;
        if (kept.reduce !== "max" || kept.channel !== "left")
            throw new Error("bands: the reduction and channel were lost: " + JSON.stringify(kept));
        picker.of = 32;
        picker.from = picker.to = 7;
        if (picker.reply.source.audio !== 7 || picker.reply.source.of !== 32)
            throw new Error("bands: a single band of 32 gave " + JSON.stringify(picker.reply));
        picker.level = "treble";
        if (JSON.stringify(picker.reply.source) !== "{\"audio\":\"treble\"}")
            throw new Error("bands: a named band gave " + JSON.stringify(picker.reply));
        picker.base = {
            "audio": "bass",
            "scale": 3
        };
        if (picker.ask({
                "pick": "mid",
                "base": picker.base
            }).source.scale !== 3)
            throw new Error("bands: the scale of the binding was lost");
        picker.destroy();
    }
}
