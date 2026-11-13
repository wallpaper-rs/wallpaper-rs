import QtQuick
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

FocusScope {
    id: buscar

    property alias text: input.text
    property string placeholder

    signal accepted

    implicitWidth: Look.buscarAncho
    implicitHeight: Look.altoBoton
    activeFocusOnTab: true
    Accessible.role: Accessible.EditableText
    Accessible.name: placeholder

    Rectangle {
        anchors.fill: parent
        radius: Look.radio
        color: Look.nocheAlta
        border.width: Look.linea
        border.color: Look.nocheLinea

        Anillo {
            visible: buscar.activeFocus
            corner: Look.radio
        }
    }

    IconImage {
        id: glass

        x: Look.buscarLado
        anchors.verticalCenter: parent.verticalCenter
        width: Look.icono
        height: Look.icono
        sourceSize.width: width
        sourceSize.height: height
        name: "magnifying-glass"
        color: Look.bruma
    }

    TextInput {
        id: input

        anchors.left: glass.right
        anchors.leftMargin: Look.buscarHueco
        anchors.right: parent.right
        anchors.rightMargin: Look.buscarLado
        anchors.verticalCenter: parent.verticalCenter
        focus: true
        clip: true
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
        color: Look.escarcha
        selectionColor: Look.escarcha
        selectedTextColor: Look.noche
        selectByMouse: true
        Accessible.ignored: true
        onAccepted: buscar.accepted()

        Text {
            visible: !input.text && !input.preeditText
            anchors.verticalCenter: parent.verticalCenter
            text: buscar.placeholder
            font: input.font
            color: Look.bruma
        }
    }

    TapHandler {
        onTapped: buscar.forceActiveFocus()
    }
}
