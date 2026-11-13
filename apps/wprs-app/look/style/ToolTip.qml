import QtQuick
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.ToolTip {
    id: control

    x: parent ? (parent.width - implicitWidth) / 2 : 0
    y: -implicitHeight - Look.e1
    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    margins: Look.e2
    topPadding: 6
    bottomPadding: 6
    leftPadding: Look.e3
    rightPadding: Look.e3
    font.family: Look.letraTexto
    font.pixelSize: Look.tPie
    closePolicy: T.Popup.CloseOnEscape | T.Popup.CloseOnPressOutsideParent | T.Popup.CloseOnReleaseOutsideParent

    contentItem: Text {
        text: control.text
        font: control.font
        wrapMode: Text.Wrap
        color: Look.escarcha
    }

    background: Rectangle {
        radius: Look.radio
        color: Look.noche
        border.width: Look.linea
        border.color: Look.nocheLinea
    }
}
