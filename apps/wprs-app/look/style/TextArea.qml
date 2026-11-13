import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.TextArea {
    id: control

    implicitWidth: Math.max(contentWidth + leftPadding + rightPadding, implicitBackgroundWidth + leftInset + rightInset, placeholder.implicitWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(contentHeight + topPadding + bottomPadding, implicitBackgroundHeight + topInset + bottomInset, placeholder.implicitHeight + topPadding + bottomPadding)
    topPadding: Look.e2
    bottomPadding: Look.e2
    leftPadding: Look.e3
    rightPadding: Look.e3
    font.family: Look.letraTexto
    font.pixelSize: Look.tCuerpo
    color: enabled ? Look.escarcha : Look.brumaBaja
    selectionColor: Look.luna
    selectedTextColor: Look.noche
    placeholderTextColor: enabled ? Look.bruma : Look.brumaBaja

    PlaceholderText {
        id: placeholder

        x: control.leftPadding
        y: control.topPadding
        width: control.width - (control.leftPadding + control.rightPadding)
        height: control.height - (control.topPadding + control.bottomPadding)
        text: control.placeholderText
        font: control.font
        color: control.placeholderTextColor
        verticalAlignment: control.verticalAlignment
        visible: !control.length && !control.preeditText && (!control.activeFocus || control.horizontalAlignment !== Qt.AlignHCenter)
        wrapMode: control.wrapMode
        elide: Text.ElideRight
        renderType: control.renderType
    }

    background: Field {
        lit: control.activeFocus && !control.readOnly
        over: control.hovered
        live: control.enabled && !control.readOnly
    }
}
