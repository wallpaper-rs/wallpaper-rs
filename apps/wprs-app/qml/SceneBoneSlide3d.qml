import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: slide

    required property var bones
    readonly property var page: bones.stage.page
    readonly property var list: (bones.model.slides ?? []).filter(s => s.at && s.unit)

    function bound(entry, end, to, done) {
        const v = [entry.unit[0] - entry.at[0], entry.unit[1] - entry.at[1]];
        const length = v[0] * v[0] + v[1] * v[1];
        if (length < 1e-6)
            return;
        const amount = ((to[0] - entry.at[0]) * v[0] + (to[1] - entry.at[1]) * v[1]) / length * 100;
        const t = entry.translation.slice();
        t[end] = Math.round(Math.max(-10000, Math.min(10000, amount)));
        page.run("physics", {
                "layer": bones.model.id,
                "bones": [entry.bone],
                "translation": [Math.min(t[0], t[1]), Math.max(t[0], t[1])]
            }, !done);
    }

    visible: list.length > 0 && !page.editor.playing

    SceneRepeater {
        id: listed

        rows: slide.list

        delegate: Item {
            id: entry

            required property int index
            readonly property var modelData: listed.rows[index]
            readonly property bool chosen: slide.page.mesh.bone === modelData.bone

            anchors.fill: parent

            SceneLightLines {
                anchors.fill: parent
                ratio: slide.bones.stage.ratio
                tint: entry.chosen ? Look.brote : Look.ambar
                dashed: !entry.chosen
                segments: entry.modelData.lines
            }

            SceneRepeater {
                id: ends

                rows: entry.chosen ? [0, 1] : []

                delegate: SceneKnob {
                    required property int index
                    readonly property var modelData: ends.rows[index]

                    at: entry.modelData.ends[modelData]
                    square: true
                    ratio: slide.bones.stage.ratio
                    tint: Look.ambar
                    label: modelData === 0 ? App.tr("Least") : App.tr("Most")
                    tip: App.tr("Drag to bound how far the head of the bone slides each way")
                    onMoved: (to, done) => slide.bound(entry.modelData, modelData, to, done)
                }
            }
        }
    }
}
