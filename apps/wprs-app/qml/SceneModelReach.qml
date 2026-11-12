import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: reach

    required property var motion
    readonly property var chains: motion.info?.ik ?? []
    readonly property var parents: (motion.info?.bones ?? []).reduce((all, b) => Object.assign(all, Object.fromEntries([[b.name, b.parent]])), {})
    readonly property string above: parents[motion.bone] ?? ""
    readonly property var layers: (motion.page.view.layers ?? []).map(l => l.id).filter(id => id !== motion.layerId)
    readonly property var choices: [null, "cursor"].concat(motion.bones.map(b => "bone:" + b), layers.map(id => "layer:" + id))

    function label(t) {
        if (t === null || Array.isArray(t))
            return App.tr("A point");
        if (t === "cursor")
            return App.tr("The pointer");
        return t.startsWith("layer:") ? App.tr("Layer %1", t.slice(6)) : App.tr("Bone %1", t.slice(5));
    }

    function run(args, gesture) {
        motion.page.run("ik", Object.assign({
                    "layer": motion.layerId
                }, args), gesture);
    }

    visible: motion.bones.length > 1 && (motion.info?.model ?? false)

    QQC2.Label {
        text: App.tr("Chains that reach")
        font.bold: true
    }

    QQC2.Button {
        icon.name: "list-add"
        text: reach.above !== "" ? App.tr("New chain from %1 to %2", reach.above, reach.motion.bone) : App.tr("Pick a bone that has a parent")
        enabled: reach.above !== "" && reach.chains.length < 16
        onClicked: reach.run({
                "root": reach.above,
                "tip": reach.motion.bone
            })
    }

    SceneRepeater {
        id: listed

        rows: reach.chains

        delegate: ColumnLayout {
            id: chain

            required property int index
            readonly property var modelData: listed.rows[index]
            readonly property var target: Array.isArray(modelData.target) ? null : modelData.target

            Layout.fillWidth: true

            RowLayout {
                QQC2.Label {
                    Layout.fillWidth: true
                    text: (chain.modelData.chain ?? []).join(" → ")
                    elide: Text.ElideRight
                }

                QQC2.ComboBox {
                    model: reach.choices.map(c => reach.label(c))
                    currentIndex: Math.max(0, reach.choices.indexOf(chain.target))
                    onActivated: index => reach.run({
                            "index": chain.index,
                            "target": reach.choices[index] ?? [100, 100]
                        })
                }

                QQC2.ToolButton {
                    icon.name: "edit-delete"
                    text: App.tr("Remove")
                    display: QQC2.AbstractButton.IconOnly
                    onClicked: reach.run({
                            "index": chain.index,
                            "remove": true
                        })
                }
            }

            RowLayout {
                QQC2.Label {
                    text: App.tr("Weight")
                }

                SceneSlider {
                    Layout.fillWidth: true
                    from: 0
                    to: 1
                    value: typeof chain.modelData.weight === "number" ? chain.modelData.weight : 1
                    onMoved: reach.run({
                            "index": chain.index,
                            "weight": Math.round(value * 100) / 100
                        }, true)
                }
            }
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: reach.chains.length > 0
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
        text: App.tr("A point, a layer or the pointer is seen through the camera; drag the point on the view.")
    }
}
