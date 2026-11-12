import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

RowLayout {
    id: row

    required property var form
    required property var pass
    required property int index
    property bool lighter: false

    function set(key, value) {
        const set = {};
        set[key] = value;
        form.pass(row.index, {
                "set": set
            }, row.lighter);
    }

    QQC2.TextField {
        Layout.fillWidth: true
        placeholderText: App.tr("Entry point")
        text: row.pass.entry ?? ""
        onEditingFinished: row.set("entry", text)
    }

    QQC2.SpinBox {
        from: 1
        to: 100
        value: Math.round((row.pass.scale ?? 1) * 100)
        textFromValue: value => App.tr("%1 %", value)
        onValueModified: row.set("scale", value / 100)
    }

    QQC2.TextField {
        Layout.fillWidth: true
        placeholderText: App.tr("Inputs, such as previous, pass:0 or target:soft")
        text: (row.pass.inputs ?? ["previous"]).join(", ")
        onEditingFinished: row.set("inputs", text.split(",").map(s => s.trim()).filter(s => s !== ""))
    }

    QQC2.TextField {
        Layout.preferredWidth: Kirigami.Units.gridUnit * 5
        placeholderText: App.tr("Target")
        text: row.pass.target ?? ""
        onEditingFinished: text === "" ? row.form.pass(row.index, {
                "clear": ["target"]
            }, row.lighter) : row.set("target", text)
    }

    QQC2.ToolButton {
        icon.name: "edit-delete"
        onClicked: row.form.pass(row.index, {
                "remove": true
            }, row.lighter)
    }
}
