import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: touch

    required property var page
    required property var node
    readonly property var kinds: ["group", "image", "video", "text", "shape", "solid", "spectrum", "mesh"]
    readonly property var raw: node ? JSON.parse((page.view, page.editor.value(node.pointer + "/solid")) || "null") : null
    readonly property string takes: raw === true ? "all" : (raw !== null && typeof raw === "object" ? "alpha" : "off")
    readonly property real alpha: raw?.alpha ?? 0.5

    function put(takes, alpha) {
        page.run("layer", {
                "touch": {
                    "layer": node.id,
                    "takes": takes,
                    "alpha": alpha
                }
            });
    }

    visible: node !== null && kinds.includes(node.type)

    QQC2.Label {
        text: App.tr("Pointer")
        font.bold: true
    }

    Repeater {
        model: [["off", App.tr("Lets the pointer through")], ["all", App.tr("Takes the pointer over its whole box")], ["alpha", App.tr("Takes the pointer where it is opaque")]]

        delegate: QQC2.RadioButton {
            required property var modelData

            text: modelData[1]
            checked: touch.takes === modelData[0]
            onClicked: touch.put(modelData[0], modelData[0] === "alpha" ? touch.alpha : null)
        }
    }

    RowLayout {
        visible: touch.takes === "alpha"

        QQC2.Label {
            text: App.tr("Opaque from %1%", Math.round(touch.alpha * 100))
        }

        SceneSlider {
            Layout.fillWidth: true
            editor: touch.page.editor
            from: 0
            to: 1
            value: touch.alpha
            onMoved: touch.put("alpha", Math.round(value * 100) / 100)
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: touch.takes !== "off"
        wrapMode: Text.Wrap
        text: App.tr("Clicks and the pointer entering or leaving trigger timelines, sounds and scripts, with the pointer permission. The area shows over the canvas.")
    }
}
