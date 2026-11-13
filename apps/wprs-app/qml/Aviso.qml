import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

FocusScope {
    id: aviso

    property string text
    property string tone
    property string icon: Tono.icon(tone)
    property string action
    readonly property bool held: hover.hovered || activeFocus

    signal acted
    signal closed

    implicitWidth: Math.min(Look.avisoAncho, line.implicitWidth + Look.e4 + 10)
    implicitHeight: line.implicitHeight + 2 * Look.e3
    Accessible.role: Accessible.AlertMessage
    Accessible.name: text

    HoverHandler {
        id: hover
    }

    Rectangle {
        anchors.fill: parent
        radius: Look.radio
        color: Look.nocheAlta
        border.width: Look.linea
        border.color: Look.nocheLinea
    }

    RowLayout {
        id: line

        anchors.fill: parent
        anchors.leftMargin: Look.e4
        anchors.rightMargin: 10
        spacing: Look.e3

        IconImage {
            name: aviso.icon
            sourceSize: Qt.size(Look.iconoAviso, Look.iconoAviso)
            color: Tono.of(aviso.tone, false, Look.bruma)
        }

        Text {
            Layout.fillWidth: true
            Layout.rightMargin: Look.e2
            text: aviso.text
            wrapMode: Text.WordWrap
            lineHeight: 1.35
            font.family: Look.letraTexto
            font.pixelSize: Look.tCuerpo
            color: Look.escarcha
        }

        QQC2.Button {
            visible: aviso.action !== ""
            flat: true
            text: aviso.action
            onClicked: aviso.acted()
        }

        QQC2.AbstractButton {
            id: close

            implicitWidth: Look.altoCerrar
            implicitHeight: Look.altoCerrar
            Accessible.role: Accessible.Button
            Accessible.name: App.tr("Close")
            onClicked: aviso.closed()

            Rectangle {
                anchors.fill: parent
                radius: Look.radioChico
                color: close.hovered ? Look.nocheLinea : "transparent"

                Anillo {
                    corner: Look.radioChico
                    visible: close.visualFocus
                }
            }

            IconImage {
                anchors.centerIn: parent
                name: "x"
                sourceSize: Qt.size(Look.icono, Look.icono)
                color: close.hovered ? Look.escarcha : Look.bruma
            }
        }
    }
}
