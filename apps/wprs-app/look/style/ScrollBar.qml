import QtQuick
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.ScrollBar {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    padding: 2
    visible: control.policy !== T.ScrollBar.AlwaysOff
    minimumSize: orientation === Qt.Horizontal ? height / width : width / height

    contentItem: Rectangle {
        implicitWidth: control.interactive ? Look.barra : Look.e1
        implicitHeight: control.interactive ? Look.barra : Look.e1
        radius: Math.min(width, height) / 2
        color: control.pressed ? Look.bruma : control.hovered ? Look.brumaBaja : Look.nocheLinea
        opacity: 0

        states: State {
            name: "shown"
            when: control.size < 1 && (control.policy === T.ScrollBar.AlwaysOn || control.active)

            PropertyChanges {
                target: control.contentItem
                opacity: 1
            }
        }

        transitions: Transition {
            from: "shown"

            SequentialAnimation {
                PauseAnimation {
                    duration: Look.paso * 4
                }

                NumberAnimation {
                    target: control.contentItem
                    property: "opacity"
                    to: 0
                    duration: Look.paso
                }
            }
        }
    }
}
