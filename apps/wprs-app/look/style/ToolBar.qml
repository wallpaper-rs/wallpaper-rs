import QtQuick
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.ToolBar {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, contentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, contentHeight + topPadding + bottomPadding)
    leftPadding: Look.e3
    rightPadding: Look.e3
    topPadding: Look.e1
    bottomPadding: Look.e1

    background: Rectangle {
        implicitHeight: Look.altoControl + Look.e1
        color: Look.nocheAlta

        Rectangle {
            width: parent.width
            height: Look.linea
            y: control.position === T.ToolBar.Footer ? 0 : parent.height - height
            color: Look.nocheLinea
        }
    }
}
