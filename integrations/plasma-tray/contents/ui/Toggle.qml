import QtQuick
import QtQuick.Layouts
import "../look"

Option {
    id: toggle

    checkable: true
    Accessible.role: Accessible.CheckBox
    Accessible.checked: checked

    Rectangle {
        parent: toggle.contentItem
        Layout.preferredWidth: Look.llaveAncho
        Layout.preferredHeight: Look.llaveAlto
        radius: height / 2
        color: toggle.checked ? Look.escarcha : Look.nocheLinea

        Rectangle {
            x: toggle.checked ? parent.width - width - 3 : 3
            y: 3
            width: Look.llavePunto
            height: Look.llavePunto
            radius: width / 2
            color: toggle.checked ? Look.noche : Look.bruma
        }
    }
}
