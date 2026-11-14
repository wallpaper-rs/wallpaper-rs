import QtQuick
import io.github.wallpaper_rs.app

Rectangle {
    id: mandos

    property string label
    default property alias controls: row.data

    implicitWidth: row.implicitWidth + Look.linea * 2
    implicitHeight: Look.altoControl
    radius: Look.radio
    color: Look.noche
    border.width: Look.linea
    border.color: Look.nocheLinea
    Accessible.role: Accessible.Grouping
    Accessible.name: label

    Row {
        id: row

        x: Look.linea
        height: parent.height
    }
}
