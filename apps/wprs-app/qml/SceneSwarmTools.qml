import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: tools

    required property var page
    required property string system
    property var info: null
    property string renderLayer: ""
    readonly property var layered: ["cutout", "orient", "shader", "strand", "texture", "lit", "refract", "over", "render"]

    function load() {
        const q = {
            "particles": {
                "layer": renderLayer || null,
                "time": page.editor.time,
                "sound": page.heard
            }
        };
        info = system ? JSON.parse(page.editor.rig(system, JSON.stringify(q)) || "null") : null;
    }

    function edit(what, args, gesture) {
        page.run("particles", {
                [what]: Object.assign({
                        "system": system
                    }, renderLayer && layered.includes(what) ? {
                        "layer": renderLayer
                    } : {}, args)
            }, gesture === true);
    }

    visible: info !== null
    onSystemChanged: {
        renderLayer = "";
        load();
    }
    onRenderLayerChanged: load()
    Component.onCompleted: load()

    Connections {
        function onViewChanged() {
            tools.load();
        }

        target: tools.page.editor
    }

    SceneSwarmSystems {
        Layout.fillWidth: true
        page: tools.page
        system: tools.system
    }

    SceneSwarmSettings {
        Layout.fillWidth: true
        tools: tools
    }

    Kirigami.Heading {
        level: 4
        text: App.tr("Emitters")
    }

    SceneRepeater {
        id: listed

        readonly property var host: tools

        rows: tools.info?.emitters ?? []

        delegate: SceneSwarmEmitter {
            required property int index
            readonly property var modelData: listed.rows[index]

            Layout.fillWidth: true
            tools: listed.host
            emitter: modelData
        }
    }

    QQC2.Button {
        objectName: "addEmitter"
        icon.name: "list-add"
        text: App.tr("Add an emitter")
        enabled: (tools.info?.emitters?.length ?? 0) < 64
        onClicked: tools.edit("shape", {
                "emitter": tools.info?.emitters?.length ?? 0,
                "shape": {
                    "point": [960, 540]
                }
            })
    }

    SceneSwarmPoints {
        Layout.fillWidth: true
        tools: tools
    }

    SceneSwarmStart {
        Layout.fillWidth: true
        tools: tools
    }

    SceneSwarmFields {
        Layout.fillWidth: true
        tools: tools
    }

    SceneSwarmRemaps {
        Layout.fillWidth: true
        tools: tools
    }

    SceneSwarmRemaps {
        Layout.fillWidth: true
        tools: tools
        steady: true
    }

    SceneSwarmSteer {
        Layout.fillWidth: true
        tools: tools
    }

    SceneSwarmWindows {
        Layout.fillWidth: true
        tools: tools
    }

    SceneSwarmColliders {
        Layout.fillWidth: true
        tools: tools
    }

    SceneSwarmScope {
        Layout.fillWidth: true
        tools: tools
    }

    SceneSwarmStrands {
        Layout.fillWidth: true
        tools: tools
    }

    SceneSwarmBudget {
        Layout.fillWidth: true
        tools: tools
    }

    SceneSwarmStack {
        Layout.fillWidth: true
        tools: tools
    }

    SceneSwarmLook {
        Layout.fillWidth: true
        tools: tools
    }

    SceneSwarmOverLife {
        Layout.fillWidth: true
        tools: tools
    }

    SceneSwarmLight {
        Layout.fillWidth: true
        tools: tools
    }

    SceneSwarmKin {
        Layout.fillWidth: true
        tools: tools
    }

    Kirigami.Separator {
        Layout.fillWidth: true
    }
}
