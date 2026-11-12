import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

RowLayout {
    id: row

    required property var owner
    required property string name
    required property var spec
    readonly property string kind: spec.type ?? "number"
    readonly property bool ranged: kind === "number"

    function send(extra) {
        const body = {
            "id": name,
            "type": kind,
            "default": spec.default
        };
        if (spec.min !== undefined)
            body.min = spec.min;
        if (spec.max !== undefined)
            body.max = spec.max;
        owner.commit(Object.assign(body, extra));
    }

    function number(text) {
        const value = parseFloat(text);
        return isNaN(value) ? undefined : value;
    }

    function pair(first, second) {
        return [number(first) ?? 0, number(second) ?? 0];
    }

    spacing: Kirigami.Units.smallSpacing

    QQC2.TextField {
        Layout.preferredWidth: Kirigami.Units.gridUnit * 8
        text: {
            row.owner.revision;
            return row.name;
        }
        onEditingFinished: {
            if (text !== row.name)
                row.owner.commit({
                        "id": row.name,
                        "to": text
                    });
        }
    }

    QQC2.ComboBox {
        model: row.owner.kinds.map(kind => kind[1])
        currentIndex: row.owner.kinds.findIndex(kind => kind[0] === row.kind)
        onActivated: index => {
            if (row.owner.kinds[index][0] !== row.kind)
                row.owner.commit({
                        "id": row.name,
                        "type": row.owner.kinds[index][0]
                    });
        }
    }

    QQC2.TextField {
        visible: row.kind === "number" || row.kind === "text"
        Layout.fillWidth: true
        placeholderText: App.tr("Default")
        text: {
            row.owner.revision;
            return String(row.spec.default ?? "");
        }
        onEditingFinished: row.send({
                "default": row.kind === "number" ? (row.number(text) ?? 0) : text
            })
    }

    QQC2.CheckBox {
        visible: row.kind === "bool"
        text: App.tr("On by default")
        checked: row.spec.default === true
        onToggled: row.send({
                "default": checked
            })
    }

    SceneColor {
        visible: row.kind === "color"
        hex: typeof row.spec.default === "string" ? row.spec.default : "#ffffff"
        onPicked: hex => row.send({
                    "default": hex
                })
    }

    QQC2.TextField {
        id: first

        visible: row.kind === "vec2"
        Layout.preferredWidth: Kirigami.Units.gridUnit * 4
        text: {
            row.owner.revision;
            return String((row.spec.default ?? [0, 0])[0]);
        }
        onEditingFinished: row.send({
                "default": row.pair(text, second.text)
            })
    }

    QQC2.TextField {
        id: second

        visible: row.kind === "vec2"
        Layout.preferredWidth: Kirigami.Units.gridUnit * 4
        text: {
            row.owner.revision;
            return String((row.spec.default ?? [0, 0])[1]);
        }
        onEditingFinished: row.send({
                "default": row.pair(first.text, text)
            })
    }

    QQC2.TextField {
        visible: row.ranged
        Layout.preferredWidth: Kirigami.Units.gridUnit * 4
        placeholderText: App.tr("Min")
        text: {
            row.owner.revision;
            return String(row.spec.min ?? "");
        }
        onEditingFinished: row.send({
                "min": row.number(text)
            })
    }

    QQC2.TextField {
        visible: row.ranged
        Layout.preferredWidth: Kirigami.Units.gridUnit * 4
        placeholderText: App.tr("Max")
        text: {
            row.owner.revision;
            return String(row.spec.max ?? "");
        }
        onEditingFinished: row.send({
                "max": row.number(text)
            })
    }

    QQC2.ToolButton {
        icon.name: "edit-delete"
        text: App.tr("Remove")
        display: QQC2.AbstractButton.IconOnly
        QQC2.ToolTip.text: text
        QQC2.ToolTip.visible: hovered
        onClicked: row.owner.commit({
                "id": row.name,
                "remove": true
            })
    }
}
