import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: brush

    required property var input
    required property string mode
    readonly property var page: input.page
    readonly property bool flow: mode === "flow"
    readonly property bool painting: page.brush?.pointer === input.field.pointer && (page.brush.action === "comb_flow") === flow

    function pick() {
        const args = {
            "pointer": input.field.pointer,
            "channel": null,
            "radius": Math.round(size.value * 1000) / 1000,
            "hardness": 0.5,
            "value": erase.checked ? 0 : Math.round(level.value * 100) / 100,
            "action": flow ? "comb_flow" : "paint_mask"
        };
        page.brush = args;
    }

    RowLayout {
        QQC2.Button {
            text: brush.flow ? App.tr("Paint the direction") : App.tr("Paint in grey")
            icon.name: "draw-brush"
            checkable: true
            checked: brush.painting
            onToggled: {
                if (checked)
                    brush.pick();
                else
                    brush.page.brush = null;
            }
        }

        QQC2.CheckBox {
            id: erase

            text: brush.flow ? App.tr("Flatten") : App.tr("Erase")
            onToggled: {
                if (brush.painting)
                    brush.pick();
            }
        }
    }

    GridLayout {
        visible: brush.painting
        columns: 2

        QQC2.Label {
            text: App.tr("Brush size")
        }

        SceneSlider {
            id: size

            Layout.fillWidth: true
            from: 0.005
            to: 0.25
            value: 0.05
            onMoved: brush.pick()
        }

        QQC2.Label {
            text: brush.flow ? App.tr("Strength") : App.tr("Gray")
        }

        SceneSlider {
            id: level

            Layout.fillWidth: true
            value: 1
            onMoved: brush.pick()
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: brush.painting
        wrapMode: Text.Wrap
        text: brush.flow ? App.tr("Drag on the layer in the preview: each stroke writes the way it went, and the rest stays still. Each stroke is one step to undo.") : App.tr("Paint on the layer in the preview; each stroke is one step to undo.")
    }
}
