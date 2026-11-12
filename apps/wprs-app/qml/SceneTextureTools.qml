import QtQuick
import QtQuick.Layouts

ColumnLayout {
    id: tools

    required property var input
    readonly property var kinds: input.field.sample?.tools ?? []

    Loader {
        Layout.fillWidth: true
        active: tools.kinds.includes("ramp")
        visible: active
        sourceComponent: SceneRampTool {
            input: tools.input
            format: "png"
        }
    }

    Loader {
        Layout.fillWidth: true
        active: tools.kinds.includes("table")
        visible: active
        sourceComponent: SceneRampTool {
            input: tools.input
            format: "cube"
        }
    }

    Loader {
        Layout.fillWidth: true
        active: tools.kinds.includes("gray")
        visible: active
        sourceComponent: SceneTextureBrush {
            input: tools.input
            mode: "gray"
        }
    }

    Loader {
        Layout.fillWidth: true
        active: tools.kinds.includes("flow")
        visible: active
        sourceComponent: SceneTextureBrush {
            input: tools.input
            mode: "flow"
        }
    }

    Loader {
        Layout.fillWidth: true
        active: tools.kinds.includes("depth")
        visible: active
        sourceComponent: SceneDepthTool {
            input: tools.input
        }
    }
}
