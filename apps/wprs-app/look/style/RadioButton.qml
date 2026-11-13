import QtQuick
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.RadioButton {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding, implicitIndicatorHeight + topPadding + bottomPadding)
    padding: Look.e1
    spacing: Look.e3
    font.pixelSize: Look.tCuerpo

    indicator: Rectangle {
        x: control.text ? (control.mirrored ? control.width - width - control.rightPadding : control.leftPadding) : control.leftPadding + (control.availableWidth - width) / 2
        y: control.topPadding + (control.availableHeight - height) / 2
        implicitWidth: Look.marca
        implicitHeight: Look.marca
        radius: width / 2
        color: control.checked ? (control.enabled ? Look.escarcha : Look.bruma) : control.down ? Look.noche : Look.nocheLinea
        border.width: control.checked ? 0 : Look.linea
        border.color: control.enabled && control.hovered ? Look.bruma : Look.nocheLinea

        Rectangle {
            anchors.centerIn: parent
            width: Look.e2
            height: Look.e2
            radius: width / 2
            visible: control.checked
            color: Look.noche
        }

        FocusRing {
            corner: parent.radius
            visible: control.visualFocus
        }
    }

    contentItem: CheckText {
        control: control
    }
}
