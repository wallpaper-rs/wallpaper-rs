import QtQuick
import QtQuick.Layouts

ColumnLayout {
    id: panel

    required property var page

    ScenePools {
        Layout.fillWidth: true
        page: panel.page
    }

    SceneUses {
        Layout.fillWidth: true
        page: panel.page
    }
}
