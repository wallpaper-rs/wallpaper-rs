import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: trial

    required property var page
    property bool player: false

    spacing: 0

    SceneTry {
        Layout.fillWidth: true
        Layout.fillHeight: true
        page: trial.page
    }

    Rectangle {
        Layout.fillWidth: true
        implicitHeight: Look.linea
        color: Look.nocheLinea
    }

    QQC2.ToolButton {
        Layout.fillWidth: true
        text: App.tr("Simulated player")
        icon.name: trial.player ? "caret-down" : "caret-right"
        checkable: true
        checked: trial.player
        onToggled: trial.player = checked
    }

    SceneMedia {
        Layout.fillWidth: true
        Layout.fillHeight: true
        visible: trial.player
        page: trial.page
    }
}
