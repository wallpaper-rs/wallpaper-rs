import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

RowLayout {
    id: row

    required property var form
    required property var param
    required property int index
    readonly property var held: (form.report.handles ?? []).find(h => h.param === param.id) ?? null
    readonly property var handles: ({
            "point": App.tr("a point on the canvas"),
            "corner": App.tr("a corner on the canvas"),
            "vector": App.tr("an arrow on the canvas"),
            "angle": App.tr("a dial on the canvas")
        })

    function number(text, key) {
        const value = parseFloat(text);
        if (text.trim() === "")
            form.param(param.id, {
                    "clear": [key]
                });
        else if (!isNaN(value)) {
            const set = {};
            set[key] = value;
            form.param(param.id, {
                    "set": set
                });
        }
    }

    function given(text) {
        try {
            return JSON.parse(text);
        } catch (e) {
            return text;
        }
    }

    QQC2.Label {
        Layout.preferredWidth: Kirigami.Units.gridUnit * 5
        text: row.param.id
        elide: Text.ElideRight
    }

    QQC2.ComboBox {
        model: row.form.kinds
        currentIndex: Math.max(0, row.form.kinds.indexOf(row.param.type))
        onActivated: index => row.form.param(row.param.id, {
                    "set": {
                        "type": row.form.kinds[index]
                    }
                })
    }

    QQC2.TextField {
        Layout.preferredWidth: Kirigami.Units.gridUnit * 5
        placeholderText: App.tr("Default")
        text: JSON.stringify(row.param.default ?? null)
        onEditingFinished: row.form.param(row.param.id, {
                "set": {
                    "default": row.given(text)
                }
            })
    }

    QQC2.TextField {
        Layout.preferredWidth: Kirigami.Units.gridUnit * 3
        placeholderText: App.tr("Min")
        text: row.param.min === undefined ? "" : String(row.param.min)
        onEditingFinished: row.number(text, "min")
    }

    QQC2.TextField {
        Layout.preferredWidth: Kirigami.Units.gridUnit * 3
        placeholderText: App.tr("Max")
        text: row.param.max === undefined ? "" : String(row.param.max)
        onEditingFinished: row.number(text, "max")
    }

    QQC2.CheckBox {
        text: App.tr("Degrees")
        visible: row.param.type === "float"
        checked: row.param.unit === "degrees"
        onToggled: row.form.param(row.param.id, checked ? {
                "set": {
                    "unit": "degrees"
                }
            } : {
                "clear": ["unit"]
            })
    }

    QQC2.ComboBox {
        readonly property var choices: [[], ["ramp"], ["gray"], ["gray", "flow"], ["gray", "depth"]]

        visible: row.param.type === "texture" || row.param.type === "mask"
        model: [App.tr("Tools by its id: %1", (row.form.source.tools?.[row.param.id] ?? []).join(", ") || App.tr("none")), App.tr("Gradient stops"), App.tr("Grey brush"), App.tr("Grey and direction brushes"), App.tr("Grey brush and depth map")]
        currentIndex: Math.max(0, choices.findIndex(c => JSON.stringify(c) === JSON.stringify(row.param.tools ?? [])))
        onActivated: index => row.form.param(row.param.id, index === 0 ? {
                "clear": ["tools"]
            } : {
                "set": {
                    "tools": choices[index]
                }
            })
    }

    QQC2.Label {
        Layout.fillWidth: true
        font: Kirigami.Theme.smallFont
        elide: Text.ElideRight
        text: row.held ? (row.held.partner ? App.tr("%1, paired with %2", row.handles[row.held.handle] ?? row.held.handle, row.held.partner) : (row.handles[row.held.handle] ?? row.held.handle)) : ""
    }

    QQC2.ToolButton {
        icon.name: "go-up"
        enabled: row.index > 0
        onClicked: row.form.param(row.param.id, {
                "at": row.index - 1
            })
    }

    QQC2.ToolButton {
        icon.name: "edit-delete"
        onClicked: row.form.param(row.param.id, {
                "remove": true
            })
    }
}
