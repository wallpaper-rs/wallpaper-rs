import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: lineage

    property var restricted: []
    readonly property bool confirmed: restricted.length === 0 || allowed.checked
    readonly property alias helper: helper

    function inspect(path) {
        const found = path.endsWith(".wpak") ? JSON.parse(helper.copyLineage(path)) : [];
        restricted = Array.isArray(found) ? found : [];
        allowed.checked = false;
    }

    visible: restricted.length > 0

    SceneEditor {
        id: helper
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        visible: true
        type: Kirigami.MessageType.Warning
        text: lineage.restricted.join("\n")
    }

    QQC2.CheckBox {
        id: allowed

        text: App.tr("I made the original or its author allows this copy")
    }
}
