import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: lineage

    property var restricted: []
    readonly property bool confirmed: restricted.length === 0 || allowed.checked
    readonly property alias helper: helper
    readonly property alias allowed: allowed

    function inspect(path) {
        const found = path !== "" ? JSON.parse(helper.copyLineage(path) || "[]") : [];
        restricted = Array.isArray(found) ? found : [];
        allowed.checked = false;
    }

    visible: restricted.length > 0
    spacing: Look.e2

    SceneEditor {
        id: helper
    }

    QQC2.Switch {
        id: allowed

        Layout.fillWidth: true
        text: App.tr("I made the original or its author allows this copy")
    }

    Repeater {
        model: lineage.restricted

        delegate: Nota {
            required property string modelData

            Layout.fillWidth: true
            Layout.leftMargin: Look.llaveAncho + Look.e3
            text: modelData
        }
    }
}
