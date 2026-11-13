import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.ItemDelegate {
    id: control

    property bool chosen: false

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    leftPadding: Look.e3
    rightPadding: Look.e3
    spacing: Look.e3
    font.family: Look.letraTexto
    font.pixelSize: Look.tCuerpo
    font.weight: highlighted || chosen ? Font.DemiBold : Font.Normal
    icon.width: Look.icono
    icon.height: Look.icono
    icon.color: highlighted ? Look.noche : enabled ? Look.bruma : Look.brumaBaja

    contentItem: IconLabel {
        spacing: control.spacing
        mirrored: control.mirrored
        display: control.display
        alignment: Qt.AlignLeft | Qt.AlignVCenter
        icon: control.icon
        text: control.text
        font: control.font
        color: control.highlighted ? Look.noche : control.enabled ? Look.escarcha : Look.brumaBaja
    }

    background: Option {
        lit: control.highlighted
        over: control.hovered
        down: control.down
    }
}
