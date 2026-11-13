import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.MenuItem {
    id: control

    readonly property bool lit: highlighted && (!hovered || visualFocus)
    readonly property color ink: lit ? Look.noche : enabled ? Look.escarcha : Look.brumaBaja
    readonly property color mark: lit ? Look.noche : enabled ? Look.bruma : Look.brumaBaja

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding, implicitIndicatorHeight + topPadding + bottomPadding)
    leftPadding: Look.e3
    rightPadding: Look.e3
    spacing: Look.e3
    font.family: Look.letraTexto
    font.pixelSize: Look.tCuerpo
    font.weight: lit ? Font.DemiBold : Font.Normal
    icon.width: Look.icono
    icon.height: Look.icono
    icon.color: mark

    contentItem: IconLabel {
        readonly property real arrowPadding: control.subMenu && control.arrow ? control.arrow.width + control.spacing : 0
        readonly property real indicatorPadding: control.checkable && control.indicator ? control.indicator.width + control.spacing : 0

        leftPadding: !control.mirrored ? indicatorPadding : arrowPadding
        rightPadding: control.mirrored ? indicatorPadding : arrowPadding
        spacing: control.spacing
        mirrored: control.mirrored
        display: control.display
        alignment: Qt.AlignLeft | Qt.AlignVCenter
        icon: control.icon
        text: control.text
        font: control.font
        color: control.ink
    }

    indicator: IconImage {
        x: control.mirrored ? control.width - width - control.rightPadding : control.leftPadding
        y: control.topPadding + (control.availableHeight - height) / 2
        width: Look.icono
        height: Look.icono
        sourceSize: Qt.size(Look.icono, Look.icono)
        visible: control.checkable
        opacity: control.checked ? 1 : 0
        name: "check-bold"
        color: control.mark
    }

    arrow: IconImage {
        x: control.mirrored ? control.leftPadding : control.width - width - control.rightPadding
        y: control.topPadding + (control.availableHeight - height) / 2
        width: Look.icono
        height: Look.icono
        sourceSize: Qt.size(Look.icono, Look.icono)
        visible: control.subMenu
        mirror: control.mirrored
        name: "caret-right"
        color: control.mark
    }

    background: Option {
        lit: control.lit
        over: control.hovered || control.highlighted
        down: control.down
    }
}
