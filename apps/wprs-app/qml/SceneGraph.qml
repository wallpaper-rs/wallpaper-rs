import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: graph

    required property var page
    property var sources: []
    property real now: 0
    property var shown: []
    property var hidden: ({})
    property var chosen: []
    readonly property var copied: SceneGraphSources.copied
    property bool aligned: true
    property bool linked: true
    readonly property SceneGraphKeys ops: SceneGraphKeys {
        graph: graph
    }
    readonly property var watch: page.view
    readonly property var span: {
        const all = shown.map(c => c.range ?? [0, 1]);
        return all.length ? [Math.min.apply(null, all.map(r => r[0])), Math.max.apply(null, all.map(r => r[1]))] : [0, 1];
    }

    function named(part) {
        const names = {
            "value": App.tr("Value"),
            "x": "X",
            "y": "Y",
            "z": "Z",
            "red": App.tr("Red"),
            "green": App.tr("Green"),
            "blue": App.tr("Blue"),
            "alpha": App.tr("Alpha")
        };
        return names[part] ?? part;
    }

    function tint(s, part) {
        const colors = [Kirigami.Theme.negativeTextColor, Kirigami.Theme.positiveTextColor, Kirigami.Theme.linkColor, Kirigami.Theme.neutralTextColor];
        return (shown[s]?.tracks.length ?? 0) > 1 ? colors[part % 4] : Kirigami.Theme.highlightColor;
    }

    function load() {
        shown = sources.map(s => JSON.parse(page.editor.rig("", JSON.stringify({
                "curves": Object.assign({
                    "samples": 160
                }, s)
            })) || "{}")).map((c, i) => Object.assign(c, {
                    "label": sources[i].label
                })).filter(c => c.list !== undefined);
        chosen = chosen.filter(c => shown[c.s]?.keys.some(k => Math.abs(k.t - c.t) < 1e-3));
    }

    function picked(s, t) {
        return chosen.some(c => c.s === s && Math.abs(c.t - t) < 1e-3);
    }

    function choose(s, t, adding) {
        const rest = adding ? chosen.filter(c => !(c.s === s && Math.abs(c.t - t) < 1e-3)) : [];
        chosen = adding && rest.length < chosen.length ? rest : rest.concat([{
                "s": s,
                "t": t
            }]);
    }

    title: App.tr("Curve editor")
    standardButtons: Kirigami.Dialog.Close
    preferredWidth: Kirigami.Units.gridUnit * 40
    onOpened: load()
    onWatchChanged: visible && load()
    onSourcesChanged: {
        chosen = [];
        if (visible)
            load();
    }

    ColumnLayout {
        spacing: Kirigami.Units.smallSpacing

        SceneGraphTools {
            Layout.fillWidth: true
            graph: graph
        }

        SceneGraphPlot {
            id: plot

            Layout.fillWidth: true
            Layout.preferredHeight: Kirigami.Units.gridUnit * 18
            graph: graph
        }

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            color: Kirigami.Theme.disabledTextColor
            text: graph.shown.length === 0 ? App.tr("No keys to show: add keyframes to a value first.") : App.tr("Click a key to pick it, Ctrl+click to add it, drag around keys to pick several. Drag picked keys to move them in time and value, the grip at the right of the selection to stretch their times, and the round handles to bend the curve. Each track is drawn on its own scale.")
        }
    }
}
