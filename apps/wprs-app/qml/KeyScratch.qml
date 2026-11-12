import QtQuick
import QtQuick.Controls as QQC2

Flickable {
    id: scratch

    property int clicks
    readonly property alias button: button
    readonly property alias combo: combo
    readonly property alias slider: slider

    width: 300
    height: 200
    z: 1000
    clip: true
    contentHeight: 1200

    Column {
        width: scratch.width

        QQC2.Button {
            id: button

            implicitWidth: 80
            onClicked: scratch.clicks++
        }

        QQC2.ComboBox {
            id: combo

            model: ["a", "b", "c", "d"]
            currentIndex: 1
        }

        QQC2.Slider {
            id: slider

            from: 0
            to: 10
            value: 5
        }

        Item {
            width: 1
            height: 1000
        }
    }
}
