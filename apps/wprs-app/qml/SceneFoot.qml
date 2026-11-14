import QtQuick
import io.github.wallpaper_rs.app

Rectangle {
    id: foot

    required property var page

    color: Look.noche

    Rectangle {
        width: parent.width
        height: Look.linea
        color: Look.nocheLinea
    }

    SceneTimes {
        anchors.fill: parent
        anchors.topMargin: Look.linea
        page: foot.page
    }
}
