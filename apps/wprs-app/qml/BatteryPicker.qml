import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

QQC2.ComboBox {
    id: picker

    required property var daemon
    readonly property string chosen: String((daemon.state?.options ?? {})["pause.on_battery"] ?? "always")
    readonly property var presets: ["below:10", "below:15", "below:20", "below:30"]
    readonly property var values: ["never", "always"].concat(presets.includes(chosen) || !chosen.startsWith("below:") ? presets : presets.concat([chosen]))
    readonly property var choices: values.map(value => ({
                "value": value,
                "text": value === "never" ? daemon.tr("Never") : value === "always" ? daemon.tr("Always") : daemon.tr("Below %1 %", value.slice(6))
            }))

    Layout.preferredWidth: Kirigami.Units.gridUnit * 14
    Accessible.name: daemon.tr("Pause on battery")
    model: choices
    textRole: "text"
    valueRole: "value"
    currentIndex: Math.max(0, values.indexOf(chosen))
    onActivated: daemon.act("SetOption", ["pause.on_battery", currentValue])
}
