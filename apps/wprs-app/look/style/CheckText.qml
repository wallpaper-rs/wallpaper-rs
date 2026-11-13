import QtQuick
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

Text {
    required property T.AbstractButton control

    leftPadding: control.indicator && !control.mirrored ? control.indicator.width + control.spacing : 0
    rightPadding: control.indicator && control.mirrored ? control.indicator.width + control.spacing : 0
    text: control.text
    font: control.font
    color: control.enabled ? Look.escarcha : Look.brumaBaja
    elide: Text.ElideRight
    verticalAlignment: Text.AlignVCenter
}
