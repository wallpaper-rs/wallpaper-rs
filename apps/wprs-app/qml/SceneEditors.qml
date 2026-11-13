import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: host

    required property var input
    readonly property Component number: numberEditor
    readonly property Component check: checkEditor
    readonly property Component pair: pairEditor
    readonly property Component list: listEditor
    readonly property Component curve: curveEditor
    readonly property Component raw: rawEditor
    readonly property Component mask: maskEditor
    readonly property Component rig: rigEditor
    readonly property Component refs: refsEditor

    visible: false

    Component {
        id: numberEditor

        RowLayout {
            id: amount

            readonly property bool named: input.field.options.includes(input.value)

            SceneSlider {
                Layout.fillWidth: true
                visible: Array.isArray(input.field.range) && !amount.named
                from: (input.field.range ?? [0, 1])[0]
                to: (input.field.range ?? [0, 1])[1]
                stepSize: input.field.widget === "integer" ? 1 : 0
                value: Number(input.value ?? 0)
                onMoved: input.commit(Math.round(value * 1000) / 1000, true)
            }

            SceneDial {
                visible: input.field.dial
                implicitWidth: Kirigami.Units.gridUnit * 2
                implicitHeight: implicitWidth
                from: -180
                to: 180
                value: Number(input.value ?? 0)
                onMoved: input.commit(Math.round(value), true)
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: App.tr("Turn %1°", Math.round(value))
            }

            QQC2.TextField {
                Layout.preferredWidth: Kirigami.Units.gridUnit * 5
                Layout.fillWidth: !Array.isArray(input.field.range)
                visible: !amount.named
                property bool typed: false

                text: typeof input.value === "number" ? String(Number(input.value.toFixed(4))) : ""
                onTextEdited: typed = true
                onEditingFinished: {
                    if (typed)
                        input.commit(input.parse(text, input.value));
                    typed = false;
                }
            }

            Repeater {
                model: input.field.options

                delegate: QQC2.CheckBox {
                    required property var modelData

                    Layout.fillWidth: amount.named
                    text: SceneWords.value(modelData)
                    checked: input.value === modelData
                    onToggled: input.commit(checked ? modelData : typeof input.field.default === "number" ? input.field.default : 0)
                }
            }
        }
    }

    Component {
        id: checkEditor

        RowLayout {
            id: toggle

            readonly property bool unset: input.field.required && !input.field.set

            QQC2.Switch {
                visible: !toggle.unset
                checked: input.value === true
                onToggled: input.commit(checked)
            }

            QQC2.Label {
                visible: toggle.unset
                text: App.tr("Not set")
                color: Look.coral
            }

            Repeater {
                model: toggle.unset ? [false, true] : []

                delegate: QQC2.Button {
                    required property bool modelData

                    text: modelData ? App.tr("Yes") : App.tr("No")
                    onClicked: input.commit(modelData)
                }
            }
        }
    }

    Component {
        id: pairEditor

        RowLayout {
            Repeater {
                model: 2

                delegate: QQC2.TextField {
                    required property int index

                    property bool typed: false

                    Layout.fillWidth: true
                    text: String(Number(Number(input.both[index]).toFixed(4)))
                    onTextEdited: typed = true
                    onEditingFinished: {
                        if (!typed)
                            return;
                        typed = false;
                        const next = [input.both[0], input.both[1]];
                        next[index] = input.parse(text, next[index]);
                        input.commit(next);
                    }
                }
            }
        }
    }

    Component {
        id: listEditor
        SceneListEditor { input: host.input }
    }

    Component {
        id: curveEditor
        SceneCurve { input: host.input }
    }

    Component {
        id: maskEditor
        SceneMask { input: host.input }
    }

    Component {
        id: refsEditor
        SceneRefs { input: host.input }
    }

    Component {
        id: rigEditor
        SceneRig { input: host.input }
    }

    Component {
        id: rawEditor

        QQC2.TextField {
            readonly property bool plain: ["text", "scalar"].includes(input.field.widget) || input.field.widget.startsWith("ref:")
            property bool typed: false

            text: plain ? String(input.value ?? "") : JSON.stringify(input.value ?? null)
            onTextEdited: typed = true
            onEditingFinished: {
                if (!typed)
                    return;
                typed = false;
                const number = Number(text.trim());
                if (input.field.widget === "scalar")
                    return input.commit(["true", "false"].includes(text.trim()) ? text.trim() === "true" : text.trim() !== "" && Number.isFinite(number) ? number : text);
                if (input.field.widget === "text")
                    return input.commit(text);
                try {
                    input.commit(JSON.parse(text));
                } catch (error) {
                    input.page.editor.error = App.tr("%1 is not valid JSON", text);
                }
            }
        }
    }
}
