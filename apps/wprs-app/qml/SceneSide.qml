import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: panel

    required property var page
    property alias tabs: side
    property alias settings: settings

    Flow {
        id: side

        property int currentIndex: 0

        Layout.fillWidth: true
        Layout.minimumWidth: 0
        Layout.preferredWidth: 0

        Repeater {
            model: [App.tr("Layers"), App.tr("Assets"), App.tr("Package"), App.tr("Player"), App.tr("Try")]

            delegate: QQC2.ToolButton {
                required property int index
                required property string modelData

                text: modelData
                checkable: true
                autoExclusive: true
                checked: side.currentIndex === index
                onClicked: side.currentIndex = index
            }
        }
    }

    StackLayout {
        Layout.fillWidth: true
        Layout.fillHeight: true
        Layout.minimumWidth: 0
        Layout.preferredWidth: 0
        currentIndex: side.currentIndex

        SceneLayers {
            page: panel.page
        }

        SceneAssets {
            page: panel.page
        }

        SceneProperties {
            id: settings

            page: panel.page
        }

        SceneMedia {
            page: panel.page
        }

        SceneTry {
            page: panel.page
        }
    }
}
