import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

Column {
    id: detalles

    property string text: App.tr("Details")
    property string details
    property bool open: false

    spacing: Look.e2

    QQC2.AbstractButton {
        id: fold

        implicitWidth: summary.implicitWidth
        implicitHeight: summary.implicitHeight
        Accessible.role: Accessible.Button
        Accessible.name: detalles.text
        Accessible.checkable: true
        Accessible.checked: detalles.open
        onClicked: detalles.open = !detalles.open

        Row {
            id: summary

            spacing: Look.listaHueco

            IconImage {
                anchors.verticalCenter: parent.verticalCenter
                name: "caret-right"
                sourceSize: Qt.size(Look.iconoNota, Look.iconoNota)
                color: Look.bruma
                rotation: detalles.open ? 90 : 0

                Behavior on rotation {
                    enabled: detalles.visible
                    NumberAnimation {
                        duration: Look.paso
                        easing.type: Easing.BezierSpline
                        easing.bezierCurve: Look.pasoCurva
                    }
                }
            }

            Text {
                anchors.verticalCenter: parent.verticalCenter
                text: detalles.text
                font.family: Look.letraTexto
                font.pixelSize: Look.tPie
                font.weight: Font.DemiBold
                color: Look.bruma
            }
        }

        Anillo {
            corner: Look.radioChico
            visible: fold.visualFocus
        }
    }

    Rectangle {
        visible: detalles.open
        width: parent.width
        implicitHeight: technical.implicitHeight + 2 * 10
        radius: Look.radioChico
        color: Look.nocheAlta

        Area {
            id: technical

            anchors.fill: parent
            readOnly: true
            selectByMouse: true
            activeFocusOnTab: false
            leftPadding: Look.e3
            rightPadding: Look.e3
            topPadding: 10
            bottomPadding: 10
            palette.mid: Look.nocheLinea
            palette.highlight: Look.luna
            text: detalles.details
            wrapMode: Text.WrapAnywhere
            font.family: Look.letraFija
            font.pixelSize: Look.tTecla
            color: Look.bruma
            Accessible.name: detalles.text
        }
    }
}
