import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: paper

    required property var host
    readonly property bool awake: host.visible && host.visibility !== Window.Minimized && host.visibility !== Window.Hidden
    readonly property var outputs: App.state.outputs ?? ({})
    readonly property string screenName: host.screen?.name ?? ""
    readonly property var shown: named(outputs[screenName]) ?? Object.values(outputs).map(named).find(found => found !== null) ?? null
    readonly property string uid: shown ?? ""
    readonly property bool none: uid === ""
    readonly property url example: "qrc:/wprs/pictures/example.jpg"
    property string loaded: "-"
    property string path: ""
    property var cache: ({})
    readonly property url picture: path !== "" ? App.url(path) : example
    readonly property bool held: path === ""

    function named(output) {
        if (output?.item)
            return output.item;
        return output?.kind === "image" && output?.image ? "file:" + output.image : null;
    }

    function keep(wanted, found) {
        const tones = JSON.parse(Look.bridge.derive(found));
        cache[wanted] = {
            "path": found,
            "tones": tones
        };
        use(found, tones);
    }

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
        if (wanted.startsWith("file:"))
            return keep(wanted, wanted.slice(5));
        App.query("Thumbnail", [wanted, 1024], "Library1", reply => {
                if (!paper || paper.uid !== wanted)
                    return;
                paper.keep(wanted, reply[0] ?? "");
            }, () => paper && paper.uid === wanted && paper.use("", JSON.parse(Look.bridge.derive(""))));
    }

    onUidChanged: refresh()
    onAwakeChanged: refresh()
    Component.onCompleted: refresh()
}
