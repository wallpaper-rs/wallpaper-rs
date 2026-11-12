import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: tool

    required property var input
    required property string format
    readonly property var page: input.page
    readonly property string own: typeof input.value === "string" ? input.value : ""
    readonly property var kept: own === "" ? null : (page.view, JSON.parse(page.editor.value("/scene/ext/editor/ramps/" + own.replace(/~/g, "~0").replace(/\//g, "~1")) || "null"))
    readonly property var stops: kept ? kept.map(s => [s.at, s.color]) : [[0, "#000000"], [1, "#ffffff"]]

    function make(list) {
        const args = {
            "stops": list.map(s => ({
                            "at": s[0],
                            "color": s[1]
                        })),
            "pointer": input.field.pointer,
            "format": format
        };
        if (kept)
            args.asset = own;
        page.run("ramp", args);
    }

    SceneStops {
        Layout.fillWidth: true
        stops: tool.stops
        onCommitted: list => tool.make(list)
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        text: tool.kept ? (tool.format === "cube" ? App.tr("The stops make a color table: dark tones take the first stops and light tones the last.") : App.tr("The stops make the picture of this gradient; change them to make it again.")) : App.tr("Change the stops to make a gradient picture and use it here.")
    }
}
