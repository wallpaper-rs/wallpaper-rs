import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2

RowLayout {
    id: pair

    property string label
    property alias value: box.value
    property alias from: box.from
    property alias to: box.to

    QQC2.Label {
        text: pair.label
    }

    QQC2.SpinBox {
        id: box

        editable: true
    }
}
