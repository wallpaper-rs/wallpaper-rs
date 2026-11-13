import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.BusyIndicator {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    padding: Look.e1

    contentItem: Item {
        implicitWidth: Look.e6
        implicitHeight: Look.e6

        IconImage {
            anchors.fill: parent
            sourceSize: Qt.size(width, height)
            name: "circle-notch"
            color: Look.bruma
            opacity: control.running ? 1 : 0

            RotationAnimator on rotation {
                running: control.running && control.visible
                loops: Animation.Infinite
                from: 0
                to: 360
                duration: Look.paso * 5
            }
        }
    }
}
