import QtQuick
import QtQuick.Layouts
import io.github.wallpaper_rs.app

Rectangle {
    id: fields

    required property var page

    color: Look.nocheAlta
    clip: true

    Rectangle {
        width: Look.linea
        height: parent.height
        color: Look.nocheLinea
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.leftMargin: Look.linea
        spacing: 0

        SceneInspector {
            Layout.fillWidth: true
            Layout.fillHeight: true
            page: fields.page
        }

        SceneProblems {
            Layout.fillWidth: true
            Layout.fillHeight: false
            page: fields.page
        }
    }
}
