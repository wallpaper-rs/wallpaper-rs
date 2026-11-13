import QtQuick
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.ScrollIndicator {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    padding: 2

    contentItem: Rectangle {
        implicitWidth: Look.e1
        implicitHeight: Look.e1
        radius: Math.min(width, height) / 2
        color: Look.nocheLinea
        visible: control.size < 1
        opacity: control.active ? 1 : 0

        Behavior on opacity {
            enabled: control.visible
            NumberAnimation {
                duration: Look.paso
            }
        }
    }
}
