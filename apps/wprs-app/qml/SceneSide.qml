import QtQuick
import QtQuick.Layouts
import io.github.wallpaper_rs.app

Rectangle {
    id: panel

    required property var page
    property alias tabs: stack
    property alias settings: settings
    property alias layers: layers

    color: Look.nocheAlta

    Rectangle {
        anchors.right: parent.right
        width: Look.linea
        height: parent.height
        color: Look.nocheLinea
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.rightMargin: Look.linea
        spacing: 0

        Filtro {
            Layout.fillWidth: true
            Layout.margins: Look.e2
            Layout.bottomMargin: 0
            sections: true
            label: App.tr("Sections")
            current: stack.currentIndex
            words: [App.tr("Layers"), App.tr("Assets"), App.tr("Options"), App.tr("Try")].map(text => ({
                        "text": text
                    }))
            onPicked: index => stack.currentIndex = index
        }

        Rectangle {
            Layout.fillWidth: true
            implicitHeight: Look.linea
            color: Look.nocheLinea
        }

        StackLayout {
            id: stack

            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.minimumWidth: 0
            Layout.preferredWidth: 0

            SceneLayers {
                id: layers

                page: panel.page
            }

            SceneAssets {
                page: panel.page
            }

            SceneProperties {
                id: settings

                page: panel.page
            }

            SceneTrial {
                page: panel.page
            }
        }
    }
}
