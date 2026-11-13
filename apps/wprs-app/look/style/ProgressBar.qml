import QtQuick
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.ProgressBar {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)

    contentItem: Item {
        implicitWidth: 160
        implicitHeight: Look.rielAlto
        clip: control.indeterminate

        Rectangle {
            id: fill

            readonly property real span: control.indeterminate ? 0.3 : control.visualPosition

            width: span * parent.width
            height: parent.height
            radius: height / 2
            color: control.enabled ? Look.escarcha : Look.brumaBaja

            NumberAnimation on x {
                running: control.indeterminate && control.visible && control.enabled
                loops: Animation.Infinite
                from: -fill.width
                to: control.availableWidth
                duration: Look.paso * 8
            }
        }
    }

    background: Rectangle {
        implicitWidth: 160
        implicitHeight: Look.rielAlto
        y: (control.height - height) / 2
        height: Look.rielAlto
        radius: height / 2
        color: Look.nocheLinea
    }
}
