import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.TabButton {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    padding: 0
    leftPadding: Look.e3
    rightPadding: Look.e3
    spacing: 7
    font.family: Look.letraTitulo
    font.pixelSize: Look.tFiltro
    font.weight: checked ? Font.Bold : Font.Medium
    icon.width: Look.icono
    icon.height: Look.icono
    icon.color: control.ink

    readonly property color ink: !enabled ? Look.brumaBaja : checked || hovered || visualFocus ? Look.escarcha : Look.bruma

    contentItem: IconLabel {
        spacing: control.spacing
        mirrored: control.mirrored
        display: control.display
        alignment: Qt.AlignLeft | Qt.AlignVCenter
        icon: control.icon
        text: control.text
        font: control.font
        color: control.ink
    }

    background: Item {
        implicitHeight: Look.altoControl

        Rectangle {
            x: control.mirrored ? parent.width - width - control.rightPadding : control.leftPadding
            y: parent.height - height - 2
            width: Look.e4
            height: Look.e1
            radius: 1
            visible: control.checked
            color: control.enabled ? Look.luna : Look.brumaBaja
        }

        FocusRing {
            corner: Look.radioChico
            visible: control.visualFocus
        }
    }
}
