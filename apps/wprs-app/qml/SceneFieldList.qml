import QtQuick
import QtQuick.Layouts
import io.github.wallpaper_rs.app

ColumnLayout {
    id: list

    required property var page
    property var rows: []

    spacing: Look.e1

    SceneRepeater {
        rows: list.rows

        delegate: SceneInput {
            required property int index

            Layout.fillWidth: true
            page: list.page
            field: list.rows[index] ?? {}
        }
    }
}
