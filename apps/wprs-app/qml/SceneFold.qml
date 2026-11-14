import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtCore as Core
import io.github.wallpaper_rs.app

ColumnLayout {
    id: fold

    required property string key
    property string title
    property bool open: true
    property var saved: null
    default property alias content: inside.data
    readonly property Component keeper: Component {
        Core.Settings {
            location: Core.StandardPaths.writableLocation(Core.StandardPaths.ConfigLocation) + "/wallpaper-rs/editor.conf"
            category: "folds"
        }
    }

    Component.onCompleted: {
        if (App.checking)
            return;
        saved = keeper.createObject(fold);
        open = saved.value(key, open ? "open" : "closed") !== "closed";
    }

    spacing: 0

    QQC2.ToolButton {
        Layout.fillWidth: true
        text: fold.title
        icon.name: fold.open ? "caret-down" : "caret-right"
        checkable: true
        checked: fold.open
        Accessible.role: Accessible.Button
        Accessible.name: fold.title
        onToggled: {
            fold.open = checked;
            if (fold.saved)
                fold.saved.setValue(fold.key, checked ? "open" : "closed");
        }
    }

    ColumnLayout {
        id: inside

        Layout.fillWidth: true
        visible: fold.open
    }
}
