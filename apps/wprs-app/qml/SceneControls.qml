import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

ColumnLayout {
    id: scene

    required property var daemon
    required property string output
    property var props: ({})
    property var inputs: ({})
    property var folded: ({})
    property string problem: ""
    property bool tools: true
    readonly property string packageId: props.package ?? ""
    readonly property var uses: (inputs.shown ?? {})[output] ?? {}
    readonly property var fields: (props.fields ?? []).filter(field => field.visible && !scene.closed(field.group))

    function load() {
        daemon.query("GetSceneProperties", [output], "Daemon1", reply => {
            if (!scene)
                return;
            const fresh = Object.assign({}, scene.folded);
            (reply.fields ?? []).filter(f => f.kind === "group" && !((reply.package + "/" + f.key) in fresh)).forEach(f => fresh[reply.package + "/" + f.key] = f.collapsed);
            scene.folded = fresh;
            scene.props = reply;
        }, () => scene && (scene.props = ({})));
        daemon.query("GetInputs", [], "Inputs1", reply => scene && (scene.inputs = reply));
    }

    function closed(group) {
        return group !== "" && folded[packageId + "/" + group] === true;
    }

    function set(key, value) {
        daemon.act("SetSceneProperty", [output, key, JSON.stringify(value)]);
    }

    Component.onCompleted: load()

    Connections {
        target: scene.daemon

        function onUpdated(topic) {
            if (["state", "inputs", "profiles"].includes(topic))
                reload.restart();
        }
    }

    Timer {
        id: reload

        interval: 300
        onTriggered: scene.load()
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        visible: scene.problem !== ""
        type: Kirigami.MessageType.Error
        text: scene.problem
    }

    QQC2.Switch {
        id: screenClicks

        visible: scene.uses.clicks === true
        text: scene.daemon.tr("Let clicks and taps on this screen reach its wallpaper")
        checked: (scene.inputs.clicks ?? []).includes(scene.output)
        onToggled: scene.daemon.act("SetClicks", [scene.output, checked], "Inputs1", () => scene && scene.load(), error => {
            if (!scene)
                return;
            scene.problem = error.message;
            scene.load();
        })

        Connections {
            target: scene

            function onInputsChanged() {
                screenClicks.checked = Qt.binding(() => (scene.inputs.clicks ?? []).includes(scene.output));
            }
        }
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        visible: scene.uses.clicks === true && (scene.inputs.unreached ?? []).includes(scene.output)
        type: Kirigami.MessageType.Warning
        text: scene.daemon.tr("Clicks and taps cannot reach the wallpaper of this screen here, so this wallpaper does not react to them.")
    }

    ScenePresets {
        Layout.fillWidth: true
        visible: scene.packageId !== ""
        daemon: scene.daemon
        output: scene.output
        props: scene.props
        tools: scene.tools
        onReset: scene.load()
    }

    SceneRepeater {
        id: listed

        rows: scene.fields

        delegate: SceneField {
            required property int index
            modelData: listed.rows[index]

            Layout.fillWidth: true
            daemon: scene.daemon
            output: scene.output
            folded: scene.closed(modelData.key)
            onEdited: (key, value) => scene.set(key, value)
            onFold: (key, on) => {
                const next = Object.assign({}, scene.folded);
                next[scene.packageId + "/" + key] = on;
                scene.folded = next;
            }
        }
    }
}
