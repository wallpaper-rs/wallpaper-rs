import QtQuick
import QtQuick.Controls.Basic as Basic
import io.github.wallpaper_rs.app

Basic.ItemDelegate {
    id: control

    background: Rectangle {
        implicitWidth: 100
        implicitHeight: 40
        visible: control.down || control.highlighted || control.visualFocus || control.hovered
        radius: Look.radioChico
        color: control.highlighted ? (control.down ? Look.lunaHonda : Look.luna) : control.down ? Look.noche : Look.nocheLinea
    }
}
