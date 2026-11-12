import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: tool

    required property var page
    readonly property var parallax: page.values.parallax ?? {}
    readonly property bool moving: parallax.enabled ?? false
    readonly property var sources: ["auto", "pointer", "accelerometer"]
    readonly property var steps: [1, 8, 16, 32]
    readonly property var missing: JSON.parse(maps.assets || "[]").filter(asset => !asset.installed && !asset.inpaint)
    readonly property string sizes: missing.map(asset => asset.size).join(" + ")

    function send(key, value) {
        const args = {};
        args[key] = value;
        tool.page.run("parallax", args);
    }

    DepthMaps {
        id: maps

        Component.onCompleted: open(tool.page.item.path ?? "")
        onMapChanged: tool.page.editor.refresh()
    }

    QQC2.Switch {
        text: App.tr("Move the picture in depth")
        checked: tool.moving
        onToggled: tool.page.run("parallax", {
            "off": !checked
        })
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        type: Kirigami.MessageType.Warning
        visible: tool.moving && maps.map === "" && !maps.busy
        text: maps.ready ? App.tr("The picture stays still: it has no depth map yet. Make one in Depth above.") : App.tr("The picture stays still: the depth model is not installed. It is downloaded only if you ask (%1), then the map is made on this computer.", tool.sizes)
        actions: [
            Kirigami.Action {
                text: maps.ready ? App.tr("Generate the map") : App.tr("Download the depth model…")
                icon.name: maps.ready ? "view-preview" : "download"
                onTriggered: maps.ready ? maps.generate(tool.page.values.depth?.input ?? 518) : consent.open()
            }
        ]
    }

    RowLayout {
        visible: maps.busy

        QQC2.ProgressBar {
            Layout.fillWidth: true
            value: Math.max(0, maps.progress)
            indeterminate: maps.progress < 0
        }

        QQC2.ToolButton {
            icon.name: "process-stop"
            text: App.tr("Cancel")
            onClicked: maps.cancel()
        }
    }

    QQC2.ComboBox {
        Layout.fillWidth: true
        enabled: tool.moving
        model: [App.tr("Slow orbit"), App.tr("Pointer"), App.tr("Accelerometer")]
        currentIndex: Math.max(0, tool.sources.indexOf(tool.parallax.source))
        onActivated: index => tool.send("source", tool.sources[index])
    }

    QQC2.ComboBox {
        Layout.fillWidth: true
        enabled: tool.moving
        model: tool.steps.map(n => n === 1 ? App.tr("Fastest: shift only") : App.tr("%1 steps", n))
        currentIndex: Math.max(0, tool.steps.indexOf(tool.parallax.steps))
        onActivated: index => tool.send("steps", tool.steps[index])
    }

    Repeater {
        model: [["strength", App.tr("Intensity"), 0, 0.03, 0.001, 3], ["focus", App.tr("Focus"), 0, 1, 0.01, 2], ["smoothing_ms", App.tr("Smoothing (ms)"), 0, 2000, 10, 0], ["overscan", App.tr("Zoom to hide the edges"), 1, 1.1, 0.005, 3]]

        delegate: EditSlider {
            required property var modelData

            enabled: tool.moving
            text: modelData[1]
            from: modelData[2]
            to: modelData[3]
            stepSize: modelData[4]
            decimals: modelData[5]
            value: tool.parallax[modelData[0]] ?? 0
            onMoved: value => tool.send(modelData[0] === "smoothing_ms" ? "smoothing" : modelData[0], value)
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        text: App.tr("Turn on Show on the desktop to try it. The parallax needs a depth map, pauses like videos and stays still when the system asks for reduced motion.")
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
    }

    DepthDialog {
        id: consent

        maps: maps
    }
}
