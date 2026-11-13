import QtQuick
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.Switch {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding, implicitIndicatorHeight + topPadding + bottomPadding)
    padding: Look.e1
    spacing: Look.e3
    font.pixelSize: Look.tCuerpo

    indicator: Rectangle {
        x: control.text ? (control.mirrored ? control.width - width - control.rightPadding : control.leftPadding) : control.leftPadding + (control.availableWidth - width) / 2
        y: control.topPadding + (control.availableHeight - height) / 2
        implicitWidth: Look.llaveAncho
        implicitHeight: Look.llaveAlto
        radius: height / 2
        color: control.checked ? (control.enabled ? Look.escarcha : Look.bruma) : Look.nocheLinea

        Rectangle {
            readonly property real hueco: (parent.height - height) / 2

            x: control.checked ? parent.width - width - hueco : hueco
            y: hueco
            width: Look.llavePunto
            height: Look.llavePunto
            radius: height / 2
            color: control.checked ? Look.noche : control.enabled ? Look.bruma : Look.brumaBaja

            Behavior on x {
                enabled: control.visible
                NumberAnimation {
                    duration: Look.paso
                    easing.type: Easing.Bezier
                    easing.bezierCurve: Look.pasoCurva
                }
            }
        }

        FocusRing {
            corner: parent.radius
            visible: control.visualFocus
        }
    }

    contentItem: Text {
        leftPadding: control.indicator && !control.mirrored ? control.indicator.width + control.spacing : 0
        rightPadding: control.indicator && control.mirrored ? control.indicator.width + control.spacing : 0
        text: control.text
        font: control.font
        color: control.enabled ? Look.escarcha : Look.brumaBaja
        elide: Text.ElideRight
        verticalAlignment: Text.AlignVCenter
    }
}
