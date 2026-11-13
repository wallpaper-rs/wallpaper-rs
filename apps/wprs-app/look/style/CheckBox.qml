import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.CheckBox {
    id: control

    readonly property bool lit: checkState !== Qt.Unchecked

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
        radius: Look.radioChico
        color: control.lit ? (control.enabled ? Look.escarcha : Look.bruma) : control.down ? Look.noche : Look.nocheLinea
        border.width: control.lit ? 0 : Look.linea
        border.color: control.enabled && control.hovered ? Look.bruma : Look.nocheLinea

        IconImage {
            anchors.centerIn: parent
            width: Look.marca - Look.e1
            height: Look.marca - Look.e1
            sourceSize: Qt.size(width, height)
            visible: control.lit
            name: control.checkState === Qt.PartiallyChecked ? "minus-bold" : "check-bold"
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
