import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Flow {
    id: tracks

    required property var graph
    required property var plot

    spacing: Kirigami.Units.smallSpacing

    Repeater {
        model: tracks.graph.shown.length

        delegate: Repeater {
            id: source

            required property int index
            readonly property var curve: tracks.graph.shown[index]

            model: curve?.tracks ?? []

            delegate: QQC2.CheckBox {
                required property var modelData

                readonly property string track: source.index + ":" + modelData.part

                text: (source.curve.label ?? source.curve.pointer) + " · " + tracks.graph.named(modelData.name)
                checked: !tracks.graph.hidden[track]
                onToggled: {
                    const next = Object.assign({}, tracks.graph.hidden);
                    next[track] = !checked;
                    tracks.graph.hidden = next;
                }

                contentItem: QQC2.Label {
                    leftPadding: parent.indicator.width + parent.spacing
                    text: parent.text
                    color: tracks.graph.tint(source.index, modelData.part)
                }
            }
        }
    }

    Shortcut {
        sequences: [StandardKey.Copy]
        enabled: tracks.graph.visible && tracks.graph.chosen.length > 0
        onActivated: tracks.graph.ops.copy()
    }

    Shortcut {
        sequences: [StandardKey.Paste]
        enabled: tracks.graph.visible && tracks.graph.copied.length > 0
        onActivated: tracks.graph.ops.paste()
    }

    Shortcut {
        sequences: [StandardKey.Delete]
        enabled: tracks.graph.visible && tracks.graph.chosen.length > 0
        onActivated: tracks.graph.ops.act({
                "remove": true
            })
    }

    Shortcut {
        sequences: ["F"]
        enabled: tracks.graph.visible && tracks.graph.chosen.length > 0
        onActivated: tracks.plot.view.fitPicked()
    }

    Shortcut {
        sequences: ["Home"]
        enabled: tracks.graph.visible
        onActivated: tracks.plot.view.fitAll()
    }
}
