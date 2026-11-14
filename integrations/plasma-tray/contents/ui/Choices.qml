import QtQuick
import QtQuick.Layouts
import "../look"

ColumnLayout {
    id: choices

    property var model: []
    property string current: ""
    signal picked(string value)

    spacing: 0

    Repeater {
        model: choices.model

        delegate: Option {
            required property var modelData

            Layout.fillWidth: true
            leftPadding: Look.e4 + Look.e5
            text: modelData[1]
            hint: modelData[0] === choices.current ? "✓" : ""
            onClicked: choices.picked(modelData[0])
        }
    }
}
