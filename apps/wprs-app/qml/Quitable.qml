import QtQuick
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

FocusScope {
    id: quitable

    property string text

    signal removed

    implicitWidth: row.implicitWidth + Look.e3 + Look.quitableHueco
    implicitHeight: Look.quitableAlto

    Rectangle {
        anchors.fill: parent
        radius: height / 2
        color: Look.nocheAlta
        border.width: Look.linea
        border.color: Look.nocheLinea
    }

    Row {
        id: row

        x: Look.e3
        anchors.verticalCenter: parent.verticalCenter
        spacing: Look.quitableHueco

        Text {
            anchors.verticalCenter: parent.verticalCenter
            text: quitable.text
            font.family: Look.letraTexto
            font.pixelSize: Look.tQuitable
            font.weight: Font.Medium
            color: Look.escarcha
        }

        Rectangle {
            id: close

            readonly property bool lit: hover.hovered || activeFocus

            width: Look.quitableBoton
            height: Look.quitableBoton
            radius: width / 2
            color: lit ? Look.nocheLinea : "transparent"
            focus: true
            activeFocusOnTab: true
            Accessible.role: Accessible.Button
            Accessible.name: App.tr("Remove the filter %1", quitable.text)
            Accessible.onPressAction: quitable.removed()
            Keys.onReturnPressed: quitable.removed()
            Keys.onEnterPressed: quitable.removed()
            Keys.onSpacePressed: quitable.removed()

            IconImage {
                anchors.centerIn: parent
                width: Look.tQuitable * 1.25
                height: width
                sourceSize.width: width
                sourceSize.height: height
                name: "x"
                color: close.lit ? Look.escarcha : Look.bruma
            }

            Anillo {
                visible: close.activeFocus
                corner: close.radius
            }

            HoverHandler {
                id: hover

                cursorShape: Qt.PointingHandCursor
            }

            TapHandler {
                onTapped: quitable.removed()
            }
        }
    }
}
