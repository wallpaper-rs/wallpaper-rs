import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: panel

    required property var page

    function pick(level) {
        SceneLook.quality = level;
        SceneLook.preview(panel.page);
    }

    spacing: Look.e1

    QQC2.Label {
        Layout.fillWidth: true
        Layout.leftMargin: Look.e2
        text: App.tr("See the scene as on a")
        color: Look.bruma
        font.pixelSize: Look.tPie
        wrapMode: Text.Wrap
    }

    Repeater {
        model: SceneLook.levels

        delegate: QQC2.ToolButton {
            required property int index
            required property string modelData

            Layout.fillWidth: true
            icon.name: "gauge"
            text: SceneLook.names[index]
            checkable: true
            checked: SceneLook.quality === modelData
            Accessible.name: text + ", " + modelData.toUpperCase()
            onClicked: panel.pick(modelData)
        }
    }

    Rectangle {
        Layout.fillWidth: true
        implicitHeight: Look.linea
        color: Look.nocheLinea
    }

    QQC2.Switch {
        Layout.fillWidth: true
        icon.name: "battery-medium"
        text: App.tr("On battery")
        checked: SceneLook.battery
        onToggled: {
            SceneLook.battery = checked;
            SceneLook.preview(panel.page);
        }
    }

    QQC2.Switch {
        Layout.fillWidth: true
        icon.name: "person-simple-walk"
        text: App.tr("Reduced motion")
        checked: SceneLook.calm
        onToggled: {
            SceneLook.calm = checked;
            SceneLook.preview(panel.page);
        }
    }

    ScenePreviewLanguage {
        Layout.fillWidth: true
        page: panel.page
    }

    Rectangle {
        Layout.fillWidth: true
        implicitHeight: Look.linea
        color: Look.nocheLinea
    }

    ScenePreviewCompare {
        Layout.fillWidth: true
        page: panel.page
    }

    Rectangle {
        Layout.fillWidth: true
        implicitHeight: Look.linea
        color: Look.nocheLinea
    }

    QQC2.ToolButton {
        Layout.fillWidth: true
        icon.name: "arrows-clockwise"
        text: App.tr("Restart the preview")
        onClicked: SceneLook.again(panel.page)
    }

    SceneFold {
        Layout.fillWidth: true
        key: "see-as-effects"
        title: App.tr("Effects on this computer")
        open: false

        ScenePreviewEffects {
            Layout.fillWidth: true
            page: panel.page
        }
    }
}
