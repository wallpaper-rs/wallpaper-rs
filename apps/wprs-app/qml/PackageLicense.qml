import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: license

    required property var host
    readonly property var options: host.field.options ?? []
    readonly property var known: options.filter(o => o.id !== "")
    readonly property string current: String(host.value ?? "")
    readonly property var explained: options.find(o => o.id === current) ?? options.find(o => o.id === "") ?? null

    spacing: Kirigami.Units.smallSpacing

    QQC2.ComboBox {
        id: pick

        Layout.fillWidth: true
        editable: true
        model: license.known.map(o => o.id)
        editText: license.current
        Accessible.name: App.tr("License")
        onActivated: license.host.commit(currentText)
        onAccepted: license.host.commit(editText.trim())

        delegate: QQC2.ItemDelegate {
            required property int index
            readonly property var option: license.known[index]

            width: ListView.view ? ListView.view.width : implicitWidth
            text: option.id + " · " + option.summary
            highlighted: pick.highlightedIndex === index
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: license.current !== "" && license.explained !== null
        wrapMode: Text.Wrap
        textFormat: Text.PlainText
        text: license.explained?.summary ?? ""
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: license.current === ""
        wrapMode: Text.Wrap
        color: Look.coral
        text: App.tr("Choose the license that says what others may do with your package.")
    }
}
