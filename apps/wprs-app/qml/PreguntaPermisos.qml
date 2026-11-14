import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

PanelChico {
    id: question

    property string scene
    property string key
    property string name
    property var inputs: []
    property var kept: ({})
    readonly property var allowed: inputs.filter(input => question.kept[input] !== false)
    readonly property var left: inputs.filter(input => question.kept[input] === false)

    signal answered(var allow, var deny)

    function ask(scene, key, name, inputs) {
        question.scene = scene;
        question.key = key;
        question.name = name;
        question.inputs = inputs;
        question.kept = {};
        open();
    }

    function answer(allow, deny) {
        App.act("Decide", [scene, key, allow, deny], "Inputs1");
        answered(allow, deny);
    }

    title: App.tr("“%1” asks for permission to use", name)
    text: App.tr("It is already set and works without this. We remember it for next time.")
    quiet: App.tr("Without this")
    principal: App.tr("Allow")
    onAccepted: answer(allowed, left)
    onRejected: answer([], inputs)

    Repeater {
        model: question.inputs

        delegate: RowLayout {
            id: row

            required property string modelData
            readonly property bool on: question.kept[modelData] !== false

            width: parent.width - 2 * Look.e4
            spacing: Look.e3

            Pide {
                Layout.fillWidth: true
                icon: Permisos.icon(row.modelData)
                text: Permisos.title(row.modelData)
                reason: Permisos.reason(row.modelData)
            }

            QQC2.Button {
                Layout.alignment: Qt.AlignTop
                flat: !row.on
                checkable: true
                checked: row.on
                text: row.on ? App.tr("Allow") : App.tr("Without this")
                Accessible.name: Permisos.title(row.modelData) + ": " + text
                onToggled: {
                    const next = Object.assign({}, question.kept);
                    next[row.modelData] = checked;
                    question.kept = next;
                }
            }
        }
    }
}
