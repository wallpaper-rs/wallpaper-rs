import QtQuick
import io.github.wallpaper_rs.app

Column {
    id: rules

    required property var playlist
    readonly property var values: playlist.rules ?? {}
    readonly property int count: repeater.count

    function shown(key) {
        const value = values[key];
        return Array.isArray(value) ? value.join(",") : value === null || value === undefined ? "" : String(value);
    }

    function set(key, value) {
        App.act("SetRule", [playlist.uid, key, value], "Playlists1");
    }

    spacing: Look.e2

    Repeater {
        id: repeater

        model: (App.schema.rules ?? []).filter(field => field.key !== "interval")

        Fila {
            required property var modelData
            readonly property var values: modelData.values ?? []

            width: rules.width
            text: App.field(modelData.key)
            kind: modelData.kind === "bool" ? "llave" : modelData.kind === "enum" ? "elige" : "campo"
            checked: rules.values[modelData.key] === true
            options: values.map(value => App.value(value))
            current: Math.max(0, values.indexOf(rules.shown(modelData.key)))
            fieldText: rules.shown(modelData.key)
            placeholder: modelData.kind === "weekdays" ? values.join(",") : modelData.default === null ? "" : String(modelData.default)
            onToggled: rules.set(modelData.key, String(checked))
            onChosen: index => rules.set(modelData.key, values[index])
            onEdited: text => rules.set(modelData.key, text.trim())
        }
    }
}
