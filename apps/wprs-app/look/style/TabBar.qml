import QtQuick
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.TabBar {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, contentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, contentHeight + topPadding + bottomPadding)
    spacing: Look.e1

    contentItem: ListView {
        model: control.contentModel
        currentIndex: control.currentIndex
        spacing: control.spacing
        orientation: ListView.Horizontal
        boundsBehavior: Flickable.StopAtBounds
        flickableDirection: Flickable.AutoFlickIfNeeded
        snapMode: ListView.SnapToItem
        highlightMoveDuration: 0
        highlightRangeMode: ListView.ApplyRange
        preferredHighlightBegin: Look.e7
        preferredHighlightEnd: width - Look.e7
    }

    background: Item {
        implicitHeight: Look.altoControl

        Rectangle {
            width: parent.width
            height: Look.linea
            y: control.position === T.TabBar.Footer ? 0 : parent.height - height
            color: Look.nocheLinea
        }
    }
}
