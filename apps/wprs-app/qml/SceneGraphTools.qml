import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: tools

    required property var graph
    readonly property bool lone: graph.chosen.length === 1
    readonly property real first: graph.chosen.length ? Math.min.apply(null, graph.chosen.map(c => c.t)) : 0

    function stretch(text) {
        const factor = Number(String(text).replace(",", "."));
        if (isFinite(factor) && factor > 0 && factor !== 1)
            graph.ops.move(0, factor, first, 0, null);
    }

    RowLayout {
        QQC2.ToolButton {
            icon.name: "format-align-vertical-center"
            text: tools.graph.aligned ? App.tr("Aligned handles") : App.tr("Broken handles")
            checkable: true
            checked: tools.graph.aligned
            onToggled: tools.graph.aligned = checked
            QQC2.ToolTip.visible: hovered
            QQC2.ToolTip.text: App.tr("Aligned: dragging one handle of a key turns the other one with it. Broken: each handle moves alone.")
        }

        QQC2.ToolButton {
            icon.name: "link"
            text: tools.graph.linked ? App.tr("Linked parts") : App.tr("Separate parts")
            checkable: true
            checked: tools.graph.linked
            onToggled: {
                tools.graph.linked = checked;
                if (checked && tools.lone && tools.graph.ops.split(tools.graph.chosen[0].s, tools.graph.chosen[0].t))
                    tools.graph.ops.link();
            }
            QQC2.ToolTip.visible: hovered
            QQC2.ToolTip.text: App.tr("Linked: a handle dragged on one track of a position or a color bends every part of it. Separate: each track keeps handles of its own. Linking again gives the picked key the shape of its first part.")
        }

        QQC2.ToolButton {
            icon.name: "draw-line"
            text: App.tr("Straighten")
            enabled: tools.lone
            onClicked: tools.graph.ops.act({
                    "straighten": true
                })
        }

        QQC2.ToolButton {
            icon.name: "edit-copy"
            text: App.tr("Copy")
            display: QQC2.AbstractButton.IconOnly
            enabled: tools.graph.chosen.length > 0
            onClicked: tools.graph.ops.copy()
        }

        QQC2.ToolButton {
            icon.name: "edit-paste"
            text: App.tr("Paste at the playhead")
            display: QQC2.AbstractButton.IconOnly
            enabled: tools.graph.copied.length > 0
            onClicked: tools.graph.ops.paste()
        }

        QQC2.ToolButton {
            icon.name: "edit-delete"
            text: App.tr("Remove the picked keys")
            display: QQC2.AbstractButton.IconOnly
            enabled: tools.graph.chosen.length > 0
            onClicked: tools.graph.ops.act({
                    "remove": true
                })
        }

        QQC2.Label {
            text: App.tr("Stretch times ×")
            enabled: tools.graph.chosen.length > 1
        }

        QQC2.TextField {
            implicitWidth: Kirigami.Units.gridUnit * 3
            enabled: tools.graph.chosen.length > 1
            placeholderText: "2"
            onAccepted: {
                tools.stretch(text);
                text = "";
            }
        }

        Item {
            Layout.fillWidth: true
        }

        QQC2.Label {
            text: App.tr("%1 picked", tools.graph.chosen.length)
            color: Kirigami.Theme.disabledTextColor
        }
    }

    Flow {
        Layout.fillWidth: true
        spacing: Kirigami.Units.smallSpacing

        Repeater {
            model: tools.graph.shown.length

            delegate: Repeater {
                id: source

                required property int index
                readonly property var curve: tools.graph.shown[index]

                model: curve?.tracks ?? []

                delegate: QQC2.CheckBox {
                    required property var modelData

                    readonly property string track: source.index + ":" + modelData.part

                    text: (source.curve.label ?? source.curve.pointer) + " · " + tools.graph.named(modelData.name)
                    checked: !tools.graph.hidden[track]
                    onToggled: {
                        const next = Object.assign({}, tools.graph.hidden);
                        next[track] = !checked;
                        tools.graph.hidden = next;
                    }

                    contentItem: QQC2.Label {
                        leftPadding: parent.indicator.width + parent.spacing
                        text: parent.text
                        color: tools.graph.tint(source.index, modelData.part)
                    }
                }
            }
        }
    }

    Shortcut {
        sequences: [StandardKey.Copy]
        enabled: tools.graph.visible && tools.graph.chosen.length > 0
        onActivated: tools.graph.ops.copy()
    }

    Shortcut {
        sequences: [StandardKey.Paste]
        enabled: tools.graph.visible && tools.graph.copied.length > 0
        onActivated: tools.graph.ops.paste()
    }

    Shortcut {
        sequences: [StandardKey.Delete]
        enabled: tools.graph.visible && tools.graph.chosen.length > 0
        onActivated: tools.graph.ops.act({
                "remove": true
            })
    }
}
