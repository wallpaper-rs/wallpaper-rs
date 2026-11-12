import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.FormLayout {
    id: depth

    required property var page
    required property var node
    required property string pointer
    readonly property var stored: JSON.parse(page.editor.value(pointer + "/parallax") || "0")
    readonly property bool bound: stored !== null && typeof stored === "object" && !Array.isArray(stored)
    readonly property var pair: Array.isArray(stored) ? stored : [bound ? 0 : stored || 0, bound ? 0 : stored || 0]
    readonly property var reaches: [1, 3, 10]
    readonly property real reach: Math.max(reaches[spans.currentIndex], reaches.find(r => r >= Math.max(Math.abs(pair[0]), Math.abs(pair[1]))) ?? 10)
    property bool linked: pair[0] === pair[1]

    function slid(axis, value) {
        const v = Math.round(value * 1000) / 1000;
        const next = linked ? [v, v] : axis === 0 ? [v, pair[1]] : [pair[0], v];
        if (next[0] !== pair[0] || next[1] !== pair[1])
            page.run("layer", {
                "depth": {
                    "layer": node.id,
                    "depth": next
                }
            }, true);
    }

    visible: node !== null && !["sound", "light"].includes(node.type ?? "") && pointer.startsWith("/scene/layers/")

    QQC2.Label {
        Kirigami.FormData.label: App.tr("Parallax depth:")
        visible: depth.bound
        text: App.tr("Follows a binding or keyframes; change it from its bindings")
        wrapMode: Text.Wrap
    }

    QQC2.ComboBox {
        id: spans

        visible: !depth.bound
        Kirigami.FormData.label: App.tr("Parallax range:")
        model: depth.reaches.map(r => "±" + r)
    }

    QQC2.CheckBox {
        visible: !depth.bound
        text: App.tr("Same depth across and up and down")
        checked: depth.linked
        onToggled: depth.linked = checked
    }

    Repeater {
        model: depth.bound ? [] : depth.linked ? [[0, App.tr("Parallax depth:")]] : [[0, App.tr("Depth across:")], [1, App.tr("Depth up and down:")]]

        delegate: SceneSlider {
            required property var modelData

            Kirigami.FormData.label: modelData[1]
            editor: depth.page.editor
            from: -depth.reach
            to: depth.reach
            stepSize: depth.reach / 100
            value: depth.pair[modelData[0]]
            onMoved: depth.slid(modelData[0], value)
        }
    }
}
