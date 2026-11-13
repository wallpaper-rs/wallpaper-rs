import QtQuick
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.RangeSlider {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, first.implicitHandleWidth + leftPadding + rightPadding, second.implicitHandleWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, first.implicitHandleHeight + topPadding + bottomPadding, second.implicitHandleHeight + topPadding + bottomPadding)
    padding: Look.e1 + Look.anillo + Look.anilloHueco

    first.handle: Knob {
        x: control.leftPadding + (control.horizontal ? control.first.visualPosition * (control.availableWidth - width) : (control.availableWidth - width) / 2)
        y: control.topPadding + (control.horizontal ? (control.availableHeight - height) / 2 : control.first.visualPosition * (control.availableHeight - height))
        live: control.enabled
        lit: control.first.handle.activeFocus && control.visualFocus
        held: control.first.pressed
    }

    second.handle: Knob {
        x: control.leftPadding + (control.horizontal ? control.second.visualPosition * (control.availableWidth - width) : (control.availableWidth - width) / 2)
        y: control.topPadding + (control.horizontal ? (control.availableHeight - height) / 2 : control.second.visualPosition * (control.availableHeight - height))
        live: control.enabled
        lit: control.second.handle.activeFocus && control.visualFocus
        held: control.second.pressed
    }

    background: Rail {
        x: control.leftPadding + (control.horizontal ? 0 : (control.availableWidth - width) / 2)
        y: control.topPadding + (control.horizontal ? (control.availableHeight - height) / 2 : 0)
        implicitWidth: control.horizontal ? 160 : Look.rielAlto
        implicitHeight: control.horizontal ? Look.rielAlto : 160
        width: control.horizontal ? control.availableWidth : implicitWidth
        height: control.horizontal ? implicitHeight : control.availableHeight
        upright: control.vertical
        from: control.horizontal ? control.first.position : 1 - control.second.visualPosition
        to: control.horizontal ? control.second.position : 1 - control.first.visualPosition
        live: control.enabled
    }
}
