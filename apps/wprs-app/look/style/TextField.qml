import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.TextField {
    id: control

    implicitWidth: implicitBackgroundWidth + leftInset + rightInset || Math.max(contentWidth, placeholder.implicitWidth) + leftPadding + rightPadding
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, contentHeight + topPadding + bottomPadding, placeholder.implicitHeight + topPadding + bottomPadding)
    topPadding: Look.e1
    bottomPadding: Look.e1
    leftPadding: Look.e3
    rightPadding: Look.e3
    font.family: Look.letraTexto
    font.pixelSize: Look.tCuerpo
    color: enabled ? Look.escarcha : Look.brumaBaja
    selectionColor: Look.luna
    selectedTextColor: Look.noche
    placeholderTextColor: enabled ? Look.bruma : Look.brumaBaja
    verticalAlignment: TextInput.AlignVCenter

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
        elide: Text.ElideRight
        renderType: control.renderType
    }

    background: Field {
        lit: control.activeFocus && !control.readOnly
        over: control.hovered
        live: control.enabled && !control.readOnly
    }
}
