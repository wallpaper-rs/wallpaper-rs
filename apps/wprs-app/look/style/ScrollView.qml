import QtQuick
import QtQuick.Templates as T

T.ScrollView {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)

    T.ScrollBar.vertical: ScrollBar {
        id: upright

        parent: control
        x: control.mirrored ? 0 : control.width - width
        y: control.topPadding
        height: control.availableHeight
        active: level.active
    }

    T.ScrollBar.horizontal: ScrollBar {
        id: level

        parent: control
        x: control.leftPadding
        y: control.height - height
        width: control.availableWidth
        active: upright.active
    }
}
