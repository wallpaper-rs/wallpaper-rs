import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.ToolButton {
    id: control

    readonly property bool chosen: checked || highlighted
    readonly property color tinta: !enabled ? Look.brumaBaja : chosen ? Look.noche : hovered || visualFocus || down ? Look.escarcha : Look.bruma

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    padding: (Look.altoHerr - Look.icono) / 2
    leftPadding: display === T.AbstractButton.IconOnly ? padding : 10
    rightPadding: display === T.AbstractButton.IconOnly ? padding : 10
    spacing: 6
    font.family: Look.letraTexto
    font.pixelSize: Look.tHerr
    font.weight: chosen ? Font.DemiBold : Font.Medium
    icon.width: Look.icono
    icon.height: Look.icono
    icon.color: tinta

    contentItem: IconLabel {
        spacing: control.spacing
        mirrored: control.mirrored
        display: control.display
        icon: control.icon
        text: control.text
        font: control.font
        color: control.tinta
    }

    background: Rectangle {
        implicitWidth: Look.altoHerr
        implicitHeight: Look.altoHerr
        radius: Look.radioChico
        color: {
            if (!control.enabled)
                return control.chosen ? Look.nocheLinea : "transparent";
            if (control.chosen)
                return control.down ? Look.bruma : Look.escarcha;
            return control.down ? Look.noche : control.hovered ? Look.nocheLinea : "transparent";
        }

        FocusRing {
            corner: Look.radioChico
            visible: control.visualFocus
        }
    }
}
