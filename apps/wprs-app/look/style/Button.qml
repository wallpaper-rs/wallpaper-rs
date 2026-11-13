import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.Button {
    id: control

    readonly property bool principal: highlighted
    readonly property bool palabra: flat && !highlighted
    readonly property color fondo: palette.highlight
    readonly property color hondo: Qt.colorEqual(fondo, Look.luna) ? Look.lunaHonda : Look.mix(fondo, Look.noche, 0.2)
    readonly property color tinta: !enabled ? (principal ? Look.bruma : Look.brumaBaja) : principal ? Look.noche : Look.escarcha

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    leftPadding: palabra ? 2 : Look.e4
    rightPadding: palabra ? 2 : principal ? 20 : Look.e4
    topPadding: 0
    bottomPadding: 0
    spacing: 9
    font.family: principal ? Look.letraTitulo : Look.letraTexto
    font.pixelSize: principal ? Look.tBoton : Look.tCuerpo
    font.weight: principal ? Font.Bold : palabra ? Font.DemiBold : Font.Medium
    font.underline: palabra && enabled && (hovered || visualFocus)
    icon.width: 18
    icon.height: 18
    icon.color: palabra ? (enabled ? Look.bruma : Look.brumaBaja) : tinta
    transform: Translate {
        y: control.down && !control.palabra ? 1 : 0
    }

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
        implicitHeight: control.principal ? Look.altoBoton : control.palabra ? 32 : Look.altoControl
        radius: Look.radio
        border.width: control.principal || control.palabra ? 0 : Look.linea
        border.color: control.checked ? Look.bruma : Look.nocheLinea
        color: {
            if (control.palabra)
                return "transparent";
            if (control.principal)
                return !control.enabled ? Look.nocheLinea : control.down ? control.hondo : control.hovered ? Look.mix(control.fondo, Look.escarcha, 0.12) : control.fondo;
            if (!control.enabled)
                return "transparent";
            return control.down ? Look.noche : control.hovered || control.checked ? Look.nocheLinea : Look.nocheAlta;
        }

        FocusRing {
            corner: Look.radio
            visible: control.visualFocus && !control.palabra
        }
    }
}
