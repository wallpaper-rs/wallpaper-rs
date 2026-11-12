import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: lighter

    required property var form
    readonly property var variant: form.parsed.lighter ?? null
    readonly property var cost: Object.assign({
            "taps": 1,
            "passes": 1,
            "scale": 1
        }, lighter.variant?.cost ?? {})
    readonly property var levels: ["", "q0", "q1", "q2"]
    readonly property var forms: form.report.levels ?? []

    function change(set) {
        form.edit("lighter", {
                "set": set
            });
    }

    function costs(key, value) {
        const next = Object.assign({}, lighter.cost);
        next[key] = value;
        lighter.change({
                "cost": next
            });
    }

    function shown(level) {
        const names = {
            "full": App.tr("full"),
            "lighter": App.tr("lighter variant"),
            "frozen": App.tr("frozen"),
            "out": App.tr("left out")
        };
        return level.quality.toUpperCase() + " " + (names[level.form] ?? level.form);
    }

    visible: (form.source.stage ?? "layer") === "layer"

    Kirigami.Heading {
        level: 5
        text: App.tr("Lighter variant for low quality levels")
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        visible: lighter.variant === null
        text: App.tr("Without one, the effect is left out at the levels whose budget it does not fit.")
    }

    QQC2.Button {
        visible: lighter.variant === null
        icon.name: "list-add"
        text: App.tr("Add a lighter variant")
        onClicked: lighter.change({})
    }

    SceneRepeater {
        id: passes

        rows: lighter.variant?.passes ?? []

        delegate: SceneEffectPass {
            Layout.fillWidth: true
            form: lighter.form
            pass: passes.rows[index]
            lighter: true
        }
    }

    RowLayout {
        visible: lighter.variant !== null

        QQC2.Button {
            icon.name: "list-add"
            text: App.tr("Add a pass")
            onClicked: lighter.form.pass(null, {}, true)
        }

        QQC2.Label {
            text: App.tr("Its cost: taps, passes and scale")
        }

        QQC2.SpinBox {
            from: 0
            to: 4096
            value: lighter.cost.taps
            onValueModified: lighter.costs("taps", value)
        }

        QQC2.SpinBox {
            from: 0
            to: 16
            value: lighter.cost.passes
            onValueModified: lighter.costs("passes", value)
        }

        QQC2.SpinBox {
            from: 1
            to: 100
            value: Math.round(lighter.cost.scale * 100)
            textFromValue: value => App.tr("%1 %", value)
            onValueModified: lighter.costs("scale", value / 100)
        }
    }

    RowLayout {
        visible: lighter.variant !== null

        QQC2.Label {
            text: App.tr("Replaces the effect")
        }

        QQC2.ComboBox {
            Layout.fillWidth: true
            model: [App.tr("Only where the effect would be left out"), App.tr("At Q0"), App.tr("At Q1 and below"), App.tr("At Q2 and below")]
            currentIndex: Math.max(0, lighter.levels.indexOf(lighter.variant?.up_to ?? ""))
            onActivated: index => lighter.change({
                    "up_to": index === 0 ? null : lighter.levels[index]
                })
        }

        QQC2.ToolButton {
            icon.name: "edit-delete"
            text: App.tr("Remove the lighter variant")
            onClicked: lighter.form.edit("lighter", {
                    "remove": true
                })
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        visible: lighter.forms.length > 0
        text: App.tr("Drawn at each level: %1", lighter.forms.map(l => lighter.shown(l)).join(", "))
    }
}
