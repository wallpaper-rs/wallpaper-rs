import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: still

    property var told: ({})
    readonly property var outputs: App.state.outputs ?? {}
    readonly property string heavy: App.outputs.find(name => (still.outputs[name] ?? {}).quality_auto === "still" && !!(still.outputs[name] ?? {}).item) ?? ""

    function tell(output) {
        const shown = outputs[output];
        const mark = output + "/" + shown.item;
        if (told[mark])
            return;
        told[mark] = true;
        App.query("GetItem", [shown.item], "Library1", reply => {
            if (!still)
                return;
            const title = reply.item ? reply.item.title ?? "" : "";
            applicationWindow().showPassiveNotification({
                "text": App.tr("“%1” stays still on its poster. It is too heavy for this computer with the automatic quality. If you play it anyway it uses the lowest quality: it may drop frames and use more battery.", title),
                "tone": "ambar"
            }, 15000, App.tr("Play anyway"), () => App.act("SetOption", ["playback.quality." + output, "min-q1"]));
        });
    }

    onHeavyChanged: heavy !== "" && tell(heavy)
}
