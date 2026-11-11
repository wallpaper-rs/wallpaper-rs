import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: rules

    required property var owner
    readonly property var page: owner.host.input.page
    readonly property string layerId: owner.host.layerId
    property string newBone: ""
    property string newParent: ""

    function run(index, args, gesture) {
        const at = index === null ? {} : {
            "index": index
        };
        rules.page.run("bone_rule", Object.assign({
            "layer": rules.layerId
        }, at, args), gesture);
    }

    QQC2.Label {
        text: App.tr("Rules that hand a bone to another parent")
        font.bold: true
    }

    SceneRepeater {
        id: listed

        rows: rules.owner.rules

        delegate: ColumnLayout {
            id: rule

            required property int index
            readonly property var modelData: listed.rows[index]

            Layout.fillWidth: true

            RowLayout {
                QQC2.Label {
                    Layout.fillWidth: true
                    text: App.tr("%1 to %2", rule.modelData.bone, rule.modelData.parent)
                    font.italic: true
                }

                QQC2.ToolButton {
                    icon.name: "edit-delete"
                    text: App.tr("Remove")
                    display: QQC2.AbstractButton.IconOnly
                    onClicked: rules.run(rule.index, {
                        "remove": true
                    })
                }
            }

            SceneBonePick {
                Layout.fillWidth: true
                label: App.tr("New parent")
                names: rules.owner.names.filter(n => n !== rule.modelData.bone)
                chosen: [rule.modelData.parent]
                single: true
                onPicked: list => rules.run(rule.index, {
                    "parent": list[0] ?? rule.modelData.parent
                })
            }

            RowLayout {
                QQC2.Label {
                    text: App.tr("Weight")
                }

                SceneSlider {
                    Layout.fillWidth: true
                    from: 0
                    to: 1
                    value: rule.modelData.weight ?? 1
                    onMoved: rules.run(rule.index, {
                        "weight": Math.round(value * 100) / 100
                    }, true)
                }
            }
        }
    }

    QQC2.Label {
        text: App.tr("New rule")
    }

    SceneBonePick {
        Layout.fillWidth: true
        label: App.tr("Bone")
        names: rules.owner.names
        chosen: rules.newBone !== "" ? [rules.newBone] : []
        single: true
        onPicked: list => rules.newBone = list[0] ?? ""
    }

    SceneBonePick {
        Layout.fillWidth: true
        label: App.tr("New parent")
        names: rules.owner.names.filter(n => n !== rules.newBone)
        chosen: rules.newParent !== "" ? [rules.newParent] : []
        single: true
        onPicked: list => rules.newParent = list[0] ?? ""
    }

    QQC2.Button {
        text: App.tr("Add a rule")
        icon.name: "list-add"
        enabled: rules.newBone !== "" && rules.newParent !== "" && rules.newBone !== rules.newParent
        onClicked: {
            rules.run(null, {
                "bone": rules.newBone,
                "parent": rules.newParent
            });
            rules.newBone = "";
            rules.newParent = "";
        }
    }
}
