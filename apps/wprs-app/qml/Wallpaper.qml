import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: paper

    required property var host
    readonly property bool awake: host.visible && host.visibility !== Window.Minimized && host.visibility !== Window.Hidden
    readonly property var outputs: App.state.outputs ?? ({})
    readonly property string screenName: host.screen?.name ?? ""
    readonly property var shown: outputs[screenName]?.item ? outputs[screenName] : (Object.values(outputs).find(output => !!output?.item) ?? null)
    readonly property string uid: shown?.item ?? ""
    readonly property bool none: uid === ""
    readonly property url example: "qrc:/wprs/pictures/example.jpg"
    property string loaded: "-"
    property string path: ""
    property var cache: ({})
    readonly property url picture: path !== "" ? App.url(path) : example
    readonly property bool held: path === ""

    function use(found, tones) {
        path = found;
        Look.tone(tones);
    }

    function refresh() {
        if (!awake || loaded === uid)
            return;
        const wanted = uid;
        loaded = wanted;
        if (wanted === "")
            return use("", JSON.parse(Look.bridge.derive("")));
        const kept = cache[wanted];
        if (kept)
            return use(kept.path, kept.tones);
        App.query("Thumbnail", [wanted, 1024], "Library1", reply => {
                if (!paper || paper.uid !== wanted)
                    return;
                const found = reply[0] ?? "";
                const tones = JSON.parse(Look.bridge.derive(found));
                paper.cache[wanted] = {
                    "path": found,
                    "tones": tones
                };
                paper.use(found, tones);
            }, () => paper && paper.uid === wanted && paper.use("", JSON.parse(Look.bridge.derive(""))));
    }

    onUidChanged: refresh()
    onAwakeChanged: refresh()
    Component.onCompleted: refresh()
}
