import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Rectangle {
    id: panel

    required property var page
    property alias tabs: stack
    property alias settings: settings
    property alias layers: layers

    color: Look.nocheAlta
    clip: true

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

        RowLayout {
            Layout.fillWidth: true
            Layout.leftMargin: Look.e2
            Layout.topMargin: Look.e2
            spacing: 0

            Repeater {
                model: [App.tr("Layers"), App.tr("Assets"), App.tr("Options"), App.tr("Try")]

                delegate: QQC2.TabButton {
                    required property string modelData
                    required property int index
                    readonly property bool lit: stack.currentIndex === index

                    text: modelData
                    leftPadding: Look.e2
                    rightPadding: Look.e2
                    font.pixelSize: Look.tCuerpo
                    font.weight: lit ? Font.DemiBold : Font.Normal
                    checked: lit
                    Accessible.role: Accessible.PageTab
                    onClicked: stack.currentIndex = index

                    Rectangle {
                        anchors.bottom: parent.bottom
                        width: parent.width
                        height: Look.linea * 2
                        visible: parent.lit
                        color: Look.luna
                    }
                }
            }
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
