import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: sounds

    required property var page
    required property var node
    required property string pointer
    readonly property bool root: pointer === "/scene/canvas"
    readonly property bool shown: root || node?.type === "sound"
    readonly property var found: shown ? ask(page.view) : ({})
    readonly property var layers: (found.layers ?? []).filter(layer => root || layer.id === node?.id)
    readonly property var spare: root ? (found.assets ?? []).filter(asset => !(found.layers ?? []).some(layer => layer.sounds.includes(asset.name))) : []
    property var waves: ({})
    property var heard: null

    function ask(view) {
        return JSON.parse(page.editor.rig("", JSON.stringify({
                    "sounds": {}
                })) || "{}");
    }

    function key(asset) {
        const bytes = (found.assets ?? []).find(a => a.name === asset)?.bytes ?? 0;
        return asset + ":" + bytes;
    }

    function wave(asset) {
        return waves[key(asset)] ?? ({});
    }

    function fetch() {
        const next = Object.assign({}, waves);
        let asked = false;
        for (const asset of found.assets ?? []) {
            const at = key(asset.name);
            if (next[at] === undefined) {
                next[at] = {};
                asked = true;
                page.editor.wave(asset.name, 256);
            }
        }
        if (asked)
            waves = next;
    }

    function same(a, b) {
        return JSON.stringify(a) === JSON.stringify(b);
    }

    function listen(what) {
        heard = same(heard, what) ? null : what;
        SceneLook.hear(page, heard);
    }

    function trigger(layer) {
        return layer.play_on ? App.tr("Starts with %1", layer.play_on) : App.tr("Starts with the scene");
    }

    visible: shown && (layers.length > 0 || spare.length > 0)
    onFoundChanged: fetch()
    Component.onCompleted: fetch()
    Component.onDestruction: {
        if (heard !== null && page)
            SceneLook.hear(page, null);
    }

    Connections {
        function onWaved(asset, drawn) {
            const next = Object.assign({}, sounds.waves);
            next[sounds.key(asset)] = JSON.parse(drawn);
            sounds.waves = next;
        }

        target: sounds.page.editor
    }

    Kirigami.Heading {
        level: 4
        text: sounds.root ? App.tr("Sounds of the scene") : App.tr("Sound")
    }

    Repeater {
        model: sounds.layers

        delegate: SceneSoundLayer {
            owner: sounds
        }
    }

    Kirigami.Heading {
        visible: sounds.spare.length > 0
        level: 5
        text: App.tr("Sounds no layer plays")
    }

    Repeater {
        model: sounds.spare

        delegate: SceneSoundSpare {
            owner: sounds
        }
    }
}
