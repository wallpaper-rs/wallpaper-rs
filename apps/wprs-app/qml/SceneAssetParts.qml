import QtQuick
import QtQuick.Layouts

ColumnLayout {
    id: parts

    required property var page

    SceneScriptsList {
        page: parts.page
    }

    SceneTemplateTree {
        Layout.fillWidth: true
        page: parts.page
    }

    SceneEffectsList {
        page: parts.page
    }

    ScenePoolsPanel {
        Layout.fillWidth: true
        page: parts.page
    }
}
