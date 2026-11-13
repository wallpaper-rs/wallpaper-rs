import QtQuick
import QtQuick.Controls as QQC2

Flickable {
    id: scratch

    property int clicks
    readonly property alias button: button
    readonly property alias combo: combo
    readonly property alias slider: slider
    readonly property alias pad: pad
    readonly property alias list: list
    property var landed: [0, 0]

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

        AlignPad {
            id: pad

            px: scratch.landed[0]
            py: scratch.landed[1]
            onSettled: (x, y) => scratch.landed = [x, y]
        }

        FocusList {
            id: list

            width: 100
            height: 60
            model: 5
            delegate: Text {
                required property int index

                text: index
            }
        }

        Item {
            width: 1
            height: 1000
        }
    }
}
