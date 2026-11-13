import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: lamps

    required property var stage

    visible: stage.info?.through === false

    SceneRepeater {
        id: listed

        rows: lamps.visible ? (lamps.stage.info?.lights ?? []) : []

        delegate: Item {
            id: one

            required property int index
            readonly property var modelData: listed.rows[index]

            function drag(aim, from, to, held) {
                lamps.stage.grab({
                    "light": {
                        "layer": modelData.id,
                        "aim": aim
                    }
                }, held & Qt.ControlModifier ? "depth" : "plane", from, to, (aim ? modelData.aim_held : modelData.held) === "bound" && (held & Qt.ShiftModifier));
            }

            anchors.fill: parent

            SceneLightLines {
                anchors.fill: parent
                ratio: lamps.stage.ratio
                dashed: true
                tint: Look.ambar
                segments: one.modelData.at && one.modelData.aim ? [[one.modelData.at, one.modelData.aim]] : []
            }

            SceneKnob {
                at: one.modelData.at
                ratio: lamps.stage.ratio
                label: one.modelData.id
                tint: one.modelData.space ? Look.ambar : Kirigami.Theme.disabledTextColor
                tip: one.modelData.space ? App.tr("Drag to move this light among the models in the plane of the view, with Ctrl along the line of sight") : App.tr("This light follows its layer on the plane; drag it to place it in 3D")
                onMoved: (to, done) => {
                    if (done)
                        one.drag(false, one.modelData.at, to, held);
                }
            }

            SceneKnob {
                at: one.modelData.aim
                ratio: lamps.stage.ratio
                square: true
                label: App.tr("Aim")
                tint: Look.ambar
                tip: App.tr("Drag to move the point this light points at, with Ctrl along the line of sight")
                onMoved: (to, done) => {
                    if (done)
                        one.drag(true, one.modelData.aim, to, held);
                }
            }
        }
    }
}
