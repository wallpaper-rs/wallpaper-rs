import QtQuick
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

Rectangle {
    id: way

    property string text
    property string reason
    property bool marked

    signal picked

    width: parent ? parent.width : implicitWidth
    implicitHeight: words.implicitHeight + 2 * Look.e3
    radius: Look.radio
    color: "transparent"
    border.width: Look.linea
    border.color: marked ? Look.escarcha : Look.nocheLinea
    activeFocusOnTab: true
    Accessible.role: Accessible.RadioButton
    Accessible.name: text
    Accessible.description: reason
    Accessible.checkable: true
    Accessible.checked: marked
    Accessible.onPressAction: picked()
    Keys.onSpacePressed: picked()
    Keys.onReturnPressed: picked()
    Keys.onEnterPressed: picked()

    Anillo {
        visible: way.marked || way.activeFocus
        corner: way.radius
    }

    IconImage {
        id: mark

        x: Look.e3
        y: Look.e3 + 1
        name: way.marked ? "radio-button-fill" : "radio-button"
        sourceSize: Qt.size(Look.iconoAviso, Look.iconoAviso)
        color: way.marked ? Look.escarcha : Look.bruma
    }

    Column {
        id: words

        x: mark.x + mark.width + Look.e3
        y: Look.e3
        width: way.width - x - Look.e3

        Text {
            width: parent.width
            text: way.text
            wrapMode: Text.WordWrap
            font.family: Look.letraTexto
            font.pixelSize: Look.tCuerpo
            font.weight: Font.DemiBold
            color: Look.escarcha
        }

        Text {
            width: parent.width
            text: way.reason
            wrapMode: Text.WordWrap
            lineHeight: 1.3
            font.family: Look.letraTexto
            font.pixelSize: Look.tPie
            color: Look.bruma
        }
    }

    TapHandler {
        onTapped: way.picked()
    }
}
