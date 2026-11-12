import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: emission

    required property var tools
    required property int index
    readonly property var emitter: (tools.info, JSON.parse(tools.page.editor.value(SceneSwarmKnobs.pointer(tools.system, "/emitters/" + index)) || "null")) ?? ({})
    readonly property bool driven: emitter.rate !== undefined && typeof emitter.rate === "object"
    readonly property var cues: ["", "beat", "click"]

    function put(values, gesture) {
        tools.edit("emit", {
                "emitter": index,
                "values": values
            }, gesture);
    }

    function pace(most) {
        tools.edit("pace", {
                "emitter": index,
                "max_per_step": most
            });
    }

    function rate(value, replace, gesture) {
        tools.edit("settings", {
                "emitter": index,
                "rate": value,
                "replace": replace
            }, gesture);
    }

    RowLayout {
        QQC2.Label {
            text: emission.driven ? App.tr("Rate follows keyframes or a link") : App.tr("%1 a second", Math.round((emission.emitter.rate ?? 0) * 10) / 10)
        }

        SceneSlider {
            Layout.fillWidth: true
            enabled: !emission.driven
            from: 0
            to: Math.max(500, typeof emission.emitter.rate === "number" ? emission.emitter.rate : 0)
            value: typeof emission.emitter.rate === "number" ? emission.emitter.rate : 0
            onMoved: emission.rate(Math.round(value * 10) / 10, false, true)
        }

        QQC2.Button {
            visible: emission.driven
            text: App.tr("Fixed rate")
            onClicked: emission.rate(10, true)
        }
    }

    RowLayout {
        QQC2.CheckBox {
            text: App.tr("Most particles started in one step")
            checked: emission.emitter.max_per_step !== undefined
            onToggled: emission.pace(checked ? 1 : null)
        }

        QQC2.SpinBox {
            enabled: emission.emitter.max_per_step !== undefined
            from: 1
            to: 1000
            editable: true
            value: emission.emitter.max_per_step ?? 1
            onValueModified: emission.pace(value)
        }
    }

    SceneSwarmEmitLane {
        Layout.fillWidth: true
        emission: emission
    }

    SceneSwarmBursts {
        Layout.fillWidth: true
        emission: emission
    }

    RowLayout {
        QQC2.CheckBox {
            text: App.tr("Follow the pointer")
            checked: emission.emitter.follow === "cursor"
            onToggled: emission.put({
                    "follow": checked ? "cursor" : null
                })
        }

        QQC2.CheckBox {
            text: App.tr("Only while sound plays")
            checked: emission.emitter.audio_gate === true
            onToggled: emission.put({
                    "audio_gate": checked
                })
        }
    }

    RowLayout {
        QQC2.Label {
            text: App.tr("Speed taken from the emitter %1", Math.round((emission.emitter.inherit ?? 0) * 100) / 100)
        }

        SceneSlider {
            Layout.fillWidth: true
            from: 0
            to: 1
            value: emission.emitter.inherit ?? 0
            onMoved: emission.put({
                    "inherit": Math.round(value * 100) / 100
                }, true)
        }
    }
}
