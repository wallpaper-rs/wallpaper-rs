import QtQuick
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.DialogButtonBox {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    spacing: Look.e2
    padding: Look.e3
    leftPadding: Look.e4
    rightPadding: Look.e4
    alignment: Qt.AlignRight

    delegate: Button {
        highlighted: T.DialogButtonBox.buttonRole === T.DialogButtonBox.AcceptRole || T.DialogButtonBox.buttonRole === T.DialogButtonBox.YesRole
    }

    contentItem: ListView {
        implicitWidth: contentWidth
        model: control.contentModel
        spacing: control.spacing
        orientation: ListView.Horizontal
        boundsBehavior: Flickable.StopAtBounds
        snapMode: ListView.SnapToItem
    }

    background: Item {
        implicitHeight: Look.altoBoton + Look.e3 * 2

        Rectangle {
            width: parent.width
            height: Look.linea
            y: control.position === T.DialogButtonBox.Footer ? 0 : parent.height - height
            color: Look.nocheLinea
        }
    }
}
