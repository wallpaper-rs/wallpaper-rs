import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

GridLayout {
    id: deep

    required property var tools
    required property var focal
    readonly property var init: {
        tools.info;
        return JSON.parse(tools.page.editor.value(SceneSwarmKnobs.pointer(tools.system, "/init")) || "null") ?? {};
    }
    readonly property var rows: [["depth", App.tr("Depth"), -2000, 4000, 1], ["depth_speed", App.tr("Depth speed"), -500, 500, 1]]

    columns: 2

    QQC2.CheckBox {
        Layout.columnSpan: 2
        text: App.tr("Perspective")
        checked: deep.focal !== null && deep.focal !== undefined
        onToggled: {
            if (checked) {
                deep.tools.edit("perspective", {
                    "focal": 800
                });
                deep.tools.edit("start", {
                    "ranges": {
                        "depth": [0, 400]
                    }
                });
            } else {
                deep.tools.edit("perspective", {
                    "off": true
                });
            }
        }
    }

    QQC2.Label {
        visible: !!deep.focal
        text: App.tr("Focal length")
    }

    SceneSlider {
        Layout.fillWidth: true
        visible: !!deep.focal
        from: 50
        to: 4000
        value: deep.focal?.focal ?? 800
        onMoved: deep.tools.edit("perspective", {
            "focal": Math.round(value)
        }, true)
    }

    Repeater {
        model: deep.focal ? deep.rows : []

        delegate: SceneSwarmSpread {
            required property var modelData

            objectName: "spread_" + modelData[0]
            Layout.columnSpan: 2
            Layout.fillWidth: true
            tools: deep.tools
            row: modelData
            value: deep.init[modelData[0]] ?? null
        }
    }
}
