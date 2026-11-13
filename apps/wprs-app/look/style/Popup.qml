import QtQuick
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.Popup {
    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    padding: Look.e3

    background: Panel {
    }

    T.Overlay.modal: Rectangle {
        color: Look.veloColor
    }

    T.Overlay.modeless: Rectangle {
        color: "transparent"
    }
}
