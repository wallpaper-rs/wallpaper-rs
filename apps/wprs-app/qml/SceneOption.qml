import QtQuick
import io.github.wallpaper_rs.app

Fila {
    id: option

    required property var modelData
    readonly property var field: modelData
    readonly property bool log: field.log === true
    readonly property real low: field.min ?? 0
    readonly property real high: field.max ?? 1
    readonly property int places: field.decimals ?? 0
    readonly property var choices: field.options ?? []
    property real pending: 0

    signal tuned(string key, var value)

    function real(p) {
        return log ? low * Math.pow(high / low, p) : p;
    }

    function spot(v) {
        return log ? Math.log(Math.max(v, low) / low) / Math.log(high / low) : v;
    }

    function shown(v) {
        return Number(v).toLocaleString(Qt.locale(), "f", places) + (field.unit ? " " + field.unit : "");
    }

    enabled: field.enabled !== false
    off: !enabled
    text: field.label ?? ""
    note: field.help ?? ""
    kind: field.kind === "bool" ? "llave" : field.kind === "slider" ? "riel" : "elige"
    from: log ? 0 : low
    to: log ? 1 : high
    stepSize: log ? 0.01 : (field.step || (high - low) / 100)
    number: field.kind === "slider" ? shown(real(at)) : ""
    value: field.kind === "bool" ? (checked ? App.tr("On") : App.tr("Off")) : ""
    options: choices.map(choice => choice.label)
    onToggled: tuned(field.key, checked)
    onChosen: index => tuned(field.key, choices[index].value)
    onMoved: position => {
        pending = position;
        settle.restart();
    }

    Timer {
        id: settle

        interval: 150
        onTriggered: option.tuned(option.field.key, option.real(option.pending))
    }

    Binding {
        target: option
        property: "at"
        value: option.spot(Number(option.field.value ?? option.low))
        restoreMode: Binding.RestoreNone
    }

    Binding {
        target: option
        property: "checked"
        value: option.field.value === true
        restoreMode: Binding.RestoreNone
    }

    Binding {
        target: option
        property: "current"
        value: Math.max(0, option.choices.findIndex(choice => choice.value === option.field.value))
        restoreMode: Binding.RestoreNone
    }
}
