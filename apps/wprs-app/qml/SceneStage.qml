import QtQuick
import QtQuick.Layouts
import io.github.wallpaper_rs.app

Rectangle {
    id: stage

    required property var page
    readonly property bool empty: page.editor.ready && (page.view.layers ?? []).length === 0

    color: Look.noche

    ColumnLayout {
        anchors.fill: parent
        spacing: 0

        SceneNotes {
            Layout.fillWidth: true
            page: stage.page
            desk: true
        }

        SceneCanvas {
            Layout.fillWidth: true
            Layout.fillHeight: true
            page: stage.page
        }
    }

    SceneEmpty {
        anchors.fill: parent
        anchors.topMargin: Look.barraLienzoAlto
        visible: stage.empty
        page: stage.page
    }
}
