import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

GridLayout {
    id: fields

    required property var form
    readonly property var parsed: form.parsed
    readonly property var cost: Object.assign({
            "taps": 1,
            "passes": 1,
            "scale": 1
        }, parsed.cost ?? {})
    readonly property var fuses: ["", "uv", "color"]
    readonly property var reaches: [""].concat(form.floats)

    function costs(key, value) {
        const next = Object.assign({}, fields.cost);
        next[key] = value;
        form.field("cost", next);
    }

    columns: 2

    QQC2.Label {
        text: App.tr("Declared cost: taps, passes and scale")
    }

    RowLayout {
        QQC2.SpinBox {
            from: 0
            to: 4096
            value: fields.cost.taps
            onValueModified: fields.costs("taps", value)
        }

        QQC2.SpinBox {
            from: 0
            to: 16
            value: fields.cost.passes
            onValueModified: fields.costs("passes", value)
        }

        QQC2.SpinBox {
            from: 1
            to: 100
            value: Math.round(fields.cost.scale * 100)
            textFromValue: value => App.tr("%1 %", value)
            onValueModified: fields.costs("scale", value / 100)
        }
    }

    QQC2.Label {
        visible: (fields.form.source.stage ?? "layer") === "layer"
        text: App.tr("Mask")
    }

    QQC2.ComboBox {
        visible: (fields.form.source.stage ?? "layer") === "layer"
        model: [App.tr("Blend the result by the mask"), App.tr("The shader scales itself by mask_at")]
        currentIndex: fields.parsed.mask_mode === "strength" ? 1 : 0
        onActivated: index => fields.form.field("mask_mode", index === 1 ? "strength" : null)
    }

    QQC2.Label {
        visible: (fields.form.source.stage ?? "layer") === "layer"
        text: App.tr("Fused as")
    }

    QQC2.ComboBox {
        visible: (fields.form.source.stage ?? "layer") === "layer"
        model: [App.tr("Not fused"), App.tr("A bend of the picture (uv_)"), App.tr("A change of color (color_)")]
        currentIndex: Math.max(0, fields.fuses.indexOf(fields.parsed.fuse ?? ""))
        onActivated: index => fields.form.field("fuse", index === 0 ? null : fields.fuses[index])
    }

    QQC2.Label {
        visible: (fields.form.source.stage ?? "layer") === "layer"
        text: App.tr("Grows the layer by")
    }

    QQC2.ComboBox {
        visible: (fields.form.source.stage ?? "layer") === "layer"
        model: [App.tr("Nothing")].concat(fields.form.floats)
        currentIndex: Math.max(0, fields.reaches.indexOf(fields.parsed.reach ?? ""))
        onActivated: index => fields.form.field("reach", index === 0 ? null : fields.reaches[index])
    }
}
