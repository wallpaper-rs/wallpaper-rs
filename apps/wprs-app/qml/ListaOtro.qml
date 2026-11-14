import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

PanelChico {
    id: other

    required property var editor
    readonly property bool valid: /^\d+\s*[smh]$/.test(field.text.trim())

    opener: editor.interval
    title: App.tr("Change every")
    text: App.tr("Write a number and s, m or h: 45s, 20m or 3h.")
    principal: App.tr("Use")
    principalIcon: "check"
    first: field
    onAccepted: valid && other.editor.rule("interval", field.text.trim().replace(/\s+/g, ""))
    foot: [
        QQC2.TextField {
            id: field

            width: Look.panelChico - 2 * Look.e4
            placeholderText: "45m"
            onAccepted: other.valid && (other.accepted(), other.close())
        }
    ]
}
