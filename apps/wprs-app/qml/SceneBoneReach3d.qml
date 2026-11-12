import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: reach

    required property var bones
    readonly property var page: bones.stage.page
    readonly property var list: bones.model.reach ?? []

    function aim(entry, to, done) {
        const a = entry.frame[0];
        const b = entry.frame[1];
        const w = Math.abs(b[0] - a[0]) < 1e-3 ? 1 : b[0] - a[0];
        const h = Math.abs(b[1] - a[1]) < 1e-3 ? 1 : b[1] - a[1];
        const point = [(to[0] - a[0]) / w * entry.size[0], (to[1] - a[1]) / h * entry.size[1]];
        page.run("ik", {
                "layer": bones.model.id,
                "index": entry.index,
                "target": point.map(v => Math.round(v * 10) / 10)
            }, !done);
    }

    visible: list.length > 0 && !page.editor.playing

    SceneRepeater {
        id: listed

        rows: reach.list

        delegate: SceneKnob {
            required property int index
            readonly property var modelData: listed.rows[index]

            at: modelData.at
            ratio: reach.bones.stage.ratio
            tint: Kirigami.Theme.positiveTextColor
            label: App.tr("Target of %1", modelData.chain.join(" → "))
            tip: App.tr("Drag the point the chain reaches for")
            onMoved: (to, done) => reach.aim(modelData, to, done)
        }
    }
}
