import QtQuick
import QtQuick.Layouts
import io.github.wallpaper_rs.app

ColumnLayout {
    id: parts

    required property var page

    SceneFold {
        Layout.fillWidth: true
        key: "scripts"
        title: scripts.scripts.length > 0 ? App.tr("Scripts · %1", scripts.scripts.length) : App.tr("Scripts")

        SceneScriptsList {
            id: scripts

            Layout.fillWidth: true
            visible: scripts.scripts.length > 0
            page: parts.page
        }

        SceneScriptAdd {
            Layout.fillWidth: true
            page: parts.page
        }
    }

    SceneFold {
        Layout.fillWidth: true
        key: "templates"
        visible: tree.templates.length > 0
        title: App.tr("Templates")

        SceneTemplateTree {
            id: tree

            Layout.fillWidth: true
            page: parts.page
        }
    }

    SceneFold {
        Layout.fillWidth: true
        key: "effects"
        visible: effects.count > 0
        title: App.tr("Effects")

        SceneEffectsList {
            id: effects

            Layout.fillWidth: true
            page: parts.page
        }
    }

    SceneFold {
        Layout.fillWidth: true
        key: "pools"
        title: App.tr("Shape pools")

        ScenePoolsPanel {
            Layout.fillWidth: true
            page: parts.page
        }
    }
}
