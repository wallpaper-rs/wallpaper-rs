import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

RowLayout {
    id: fallo

    property string title
    property string text
    property string details
    property string action
    property bool big: false
    property string icon: "warning-circle"
    property string actionIcon
    property bool principal: false
    property string other
    readonly property alias button: main
    property alias detailsOpen: fold.open

    signal acted
    signal otherActed

    width: Math.min(big ? Look.errorGrandeAncho : Look.errorAncho, parent ? parent.width : Look.errorAncho)
    spacing: Look.e4
    Accessible.role: Accessible.AlertMessage
    Accessible.name: title
    Accessible.description: text

    IconImage {
        Layout.alignment: Qt.AlignTop
        Layout.topMargin: fallo.big ? Look.e2 : 2
        name: fallo.icon
        sourceSize: fallo.big ? Qt.size(Look.iconoErrorGrande, Look.iconoErrorGrande) : Qt.size(Look.iconoError, Look.iconoError)
        color: Look.coral
    }

    ColumnLayout {
        Layout.fillWidth: true
        Layout.alignment: Qt.AlignTop
        spacing: Look.e2

        Text {
            Layout.fillWidth: true
            text: fallo.title
            wrapMode: Text.WordWrap
            lineHeight: 1.1
            font.family: Look.letraTitulo
            font.pixelSize: fallo.big ? Look.tPagina : Look.tFila
            font.weight: Font.Bold
            font.letterSpacing: fallo.big ? -0.03 * Look.tPagina : -0.015 * Look.tFila
            color: Look.escarcha
        }

        Text {
            Layout.fillWidth: true
            visible: fallo.text !== ""
            text: fallo.text
            wrapMode: Text.WordWrap
            lineHeight: 1.3
            font.family: Look.letraTexto
            font.pixelSize: fallo.big ? Look.tExplica : Look.tCuerpo
            color: Look.bruma
        }

        RowLayout {
            Layout.topMargin: Look.e2
            visible: fallo.action !== "" || fallo.other !== ""
            spacing: Look.e3

            QQC2.Button {
                id: main

                visible: fallo.action !== ""
                highlighted: fallo.principal
                text: fallo.action
                icon.name: fallo.actionIcon
                onClicked: fallo.acted()
            }

            PiecesTecla {
                visible: fallo.principal && main.visible
                text: App.trc("key", "Enter")
            }

            QQC2.Button {
                visible: fallo.other !== ""
                text: fallo.other
                onClicked: fallo.otherActed()
            }
        }

        Detalles {
            id: fold

            Layout.fillWidth: true
            visible: fallo.details !== ""
            details: fallo.details
        }
    }
}
