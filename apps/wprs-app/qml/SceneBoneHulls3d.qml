import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: hulls

    required property var bones
    readonly property var page: bones.stage.page
    readonly property var list: bones.model.colliders ?? []

    function ring(c) {
        const r = c.radius * c.per;
        const around = (p, k) => [p[0] + Math.cos(k * Math.PI / 12) * r, p[1] + Math.sin(k * Math.PI / 12) * r];
        const out = [];
        for (let k = 0; k < 24; k++)
            out.push([around(c.at, k), around(c.at, k + 1)]);
        if (c.shape === "capsule")
            for (let k = 0; k < 24; k++)
                out.push([around(c.to, k), around(c.to, k + 1)]);
        return out.concat([[c.at, c.to]]);
    }

    visible: list.length > 0 && !page.editor.playing

    SceneRepeater {
        id: listed

        rows: hulls.list

        delegate: Item {
            id: entry

            required property int index
            readonly property var modelData: listed.rows[index]

            anchors.fill: parent

            SceneLightLines {
                anchors.fill: parent
                ratio: hulls.bones.stage.ratio
                tint: Look.coral
                dashed: true
                segments: hulls.ring(entry.modelData)
            }

            SceneKnob {
                at: entry.modelData.rim
                ratio: hulls.bones.stage.ratio
                tint: Look.coral
                label: App.tr("Collider on %1", entry.modelData.bone)
                tip: App.tr("Drag to set how wide the collider is")
                onMoved: (to, done) => {
                    const d = Math.hypot(to[0] - entry.modelData.at[0], to[1] - entry.modelData.at[1]);
                    hulls.page.run("collider", {
                            "layer": hulls.bones.model.id,
                            "index": entry.modelData.index,
                            "radius": Math.round(Math.min(100, d / Math.max(entry.modelData.per, 1e-3)) * 1000) / 1000
                        }, !done);
                }
            }
        }
    }
}
