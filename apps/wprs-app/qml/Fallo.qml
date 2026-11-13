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
    property alias detailsOpen: fold.open

    signal acted

    width: Math.min(big ? Look.errorGrandeAncho : Look.errorAncho, parent ? parent.width : Look.errorAncho)
    spacing: Look.e4
    Accessible.role: Accessible.AlertMessage
    Accessible.name: title
    Accessible.description: text

    IconImage {
        Layout.alignment: Qt.AlignTop
        Layout.topMargin: fallo.big ? Look.e2 : 2
        name: "warning-circle"
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

        QQC2.Button {
            Layout.topMargin: Look.e2
            visible: fallo.action !== ""
            text: fallo.action
            onClicked: fallo.acted()
        }

        Detalles {
            id: fold

            Layout.fillWidth: true
            visible: fallo.details !== ""
            details: fallo.details
        }
    }
}
