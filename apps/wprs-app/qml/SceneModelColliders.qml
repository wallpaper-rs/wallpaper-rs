import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: hulls

    required property var motion
    readonly property var list: motion.info?.colliders ?? []
    readonly property var shapes: ["capsule", "sphere"]

    function run(args, gesture) {
        motion.page.run("collider", Object.assign({
                    "layer": motion.layerId
                }, args), gesture);
    }

    visible: motion.bones.length > 0 && (motion.info?.model ?? false)

    QQC2.Label {
        text: App.tr("Colliders")
        font.bold: true
    }

    QQC2.Button {
        icon.name: "list-add"
        text: motion.bone !== "" ? App.tr("Add a collider on %1", hulls.motion.bone) : App.tr("Pick a bone to add a collider")
        enabled: hulls.motion.bone !== "" && hulls.list.length < 8
        onClicked: hulls.run({
                "bone": hulls.motion.bone,
                "radius": 0.1
            })
    }

    SceneRepeater {
        id: listed

        rows: hulls.list

        delegate: RowLayout {
            id: hull

            required property int index
            readonly property var modelData: listed.rows[index]

            Layout.fillWidth: true

            QQC2.Label {
                text: hull.modelData.bone
                font.italic: true
            }

            QQC2.ComboBox {
                model: [App.tr("Capsule along the bone"), App.tr("Sphere at its head")]
                currentIndex: Math.max(0, hulls.shapes.indexOf(hull.modelData.shape ?? "capsule"))
                onActivated: index => hulls.run({
                        "index": hull.index,
                        "shape": hulls.shapes[index]
                    })
            }

            SceneSlider {
                Layout.fillWidth: true
                from: 0
                to: 2
                value: hull.modelData.radius ?? 0.1
                onMoved: hulls.run({
                        "index": hull.index,
                        "radius": Math.round(value * 1000) / 1000
                    }, true)
            }

            QQC2.ToolButton {
                icon.name: "edit-delete"
                text: App.tr("Remove")
                display: QQC2.AbstractButton.IconOnly
                onClicked: hulls.run({
                        "index": hull.index,
                        "remove": true
                    })
            }
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: hulls.list.length > 0
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
        text: App.tr("Swinging bones stay out of the colliders: all of them at the two higher quality levels, the first two at the second lowest and none at the lowest, where bones keep their animation.")
    }
}
