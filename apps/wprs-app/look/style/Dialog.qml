import QtQuick
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.Dialog {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding, implicitHeaderWidth, implicitFooterWidth)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding + (implicitHeaderHeight > 0 ? implicitHeaderHeight + spacing : 0) + (implicitFooterHeight > 0 ? implicitFooterHeight + spacing : 0))
    padding: Look.e4
    font.family: Look.letraTexto
    font.pixelSize: Look.tCuerpo

    background: Panel {
    }

    header: T.Label {
        text: control.title
        visible: control.title
        elide: Text.ElideRight
        padding: Look.e4
        bottomPadding: 0
        font.family: Look.letraTitulo
        font.pixelSize: Look.tFila
        font.weight: Font.Bold
        color: Look.escarcha
    }

    footer: DialogButtonBox {
        visible: count > 0
    }

    T.Overlay.modal: Rectangle {
        color: Look.veloColor
    }

    T.Overlay.modeless: Rectangle {
        color: "transparent"
    }
}
