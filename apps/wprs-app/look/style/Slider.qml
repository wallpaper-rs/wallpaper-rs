import QtQuick
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.Slider {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitHandleWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitHandleHeight + topPadding + bottomPadding)
    padding: Look.e1 + Look.anillo + Look.anilloHueco

    handle: Knob {
        x: control.leftPadding + (control.horizontal ? control.visualPosition * (control.availableWidth - width) : (control.availableWidth - width) / 2)
        y: control.topPadding + (control.horizontal ? (control.availableHeight - height) / 2 : control.visualPosition * (control.availableHeight - height))
        live: control.enabled
        lit: control.visualFocus
        held: control.pressed
    }

    background: Rail {
        x: control.leftPadding + (control.horizontal ? 0 : (control.availableWidth - width) / 2)
        y: control.topPadding + (control.horizontal ? (control.availableHeight - height) / 2 : 0)
        implicitWidth: control.horizontal ? 160 : Look.rielAlto
        implicitHeight: control.horizontal ? Look.rielAlto : 160
        width: control.horizontal ? control.availableWidth : implicitWidth
        height: control.horizontal ? implicitHeight : control.availableHeight
        upright: control.vertical
        from: 0
        to: control.horizontal ? control.position : 1 - control.visualPosition
        live: control.enabled
    }
}
