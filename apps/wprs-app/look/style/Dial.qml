import QtQuick
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.Dial {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding) || Look.e7
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding) || Look.e7

    background: Rectangle {
        x: control.width / 2 - width / 2
        y: control.height / 2 - height / 2
        implicitWidth: Look.e7
        implicitHeight: Look.e7
        width: Math.max(Look.e6, Math.min(control.width, control.height))
        height: width
        radius: width / 2
        color: control.pressed ? Look.noche : Look.nocheAlta
        border.width: Look.rielAlto
        border.color: control.hovered ? Look.brumaBaja : Look.nocheLinea

        FocusRing {
            corner: parent.radius
            visible: control.visualFocus
        }
    }

    handle: Knob {
        readonly property real reach: control.background.width / 2 - Look.rielAlto - width / 2

        x: control.background.x + control.background.width / 2 - width / 2 + reach * Math.sin(control.angle * Math.PI / 180)
        y: control.background.y + control.background.height / 2 - height / 2 - reach * Math.cos(control.angle * Math.PI / 180)
        live: control.enabled
        held: control.pressed
    }
}
