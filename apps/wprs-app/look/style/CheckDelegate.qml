import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.CheckDelegate {
    id: control

    readonly property bool lit: checkState !== Qt.Unchecked

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding, implicitIndicatorHeight + topPadding + bottomPadding)
    leftPadding: Look.e3
    rightPadding: Look.e3
    spacing: Look.e3
    font.pixelSize: Look.tCuerpo
    icon.width: Look.icono
    icon.height: Look.icono
    icon.color: enabled ? Look.bruma : Look.brumaBaja

    contentItem: IconLabel {
        leftPadding: control.mirrored ? control.indicator.width + control.spacing : 0
        rightPadding: !control.mirrored ? control.indicator.width + control.spacing : 0
        spacing: control.spacing
        mirrored: control.mirrored
        display: control.display
        alignment: Qt.AlignLeft | Qt.AlignVCenter
        icon: control.icon
        text: control.text
        font: control.font
        color: control.enabled ? Look.escarcha : Look.brumaBaja
    }

    indicator: Rectangle {
        x: control.mirrored ? control.leftPadding : control.width - width - control.rightPadding
        y: control.topPadding + (control.availableHeight - height) / 2
        implicitWidth: Look.marca
        implicitHeight: Look.marca
        radius: Look.radioChico
        color: control.lit ? (control.enabled ? Look.escarcha : Look.bruma) : Look.nocheLinea

        IconImage {
            anchors.centerIn: parent
            width: Look.marca - Look.e1
            height: Look.marca - Look.e1
            sourceSize: Qt.size(width, height)
            visible: control.lit
            name: control.checkState === Qt.PartiallyChecked ? "minus-bold" : "check-bold"
            color: Look.noche
        }
    }

    background: Rectangle {
        implicitWidth: 200
        implicitHeight: Look.altoOpcion
        radius: Look.radioChico
        visible: control.down || control.hovered || control.visualFocus || control.highlighted
        color: control.down ? Look.noche : Look.nocheAlta

        FocusRing {
            corner: parent.radius
            visible: control.visualFocus
        }
    }
}
