import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.ToolButton {
    id: keys

    required property var host
    readonly property var info: {
        host.page.view;
        return JSON.parse(host.page.editor.rig("", JSON.stringify({
                        "strings": {}
                    })) || "{}");
    }
    readonly property var options: (info.keys ?? []).map(k => k.key).concat((info.engine ?? []).map(e => e.key))

    icon.name: "insert-text"
    text: App.tr("Use a text by key")
    display: QQC2.AbstractButton.IconOnly
    enabled: options.length > 0
    QQC2.ToolTip.text: text
    QQC2.ToolTip.visible: hovered
    onClicked: menu.popup()

    QQC2.Menu {
        id: menu

        Repeater {
            model: keys.options

            delegate: QQC2.MenuItem {
                required property string modelData

                text: "@" + modelData
                onTriggered: keys.host.page.change(keys.host.field.pointer, "@" + modelData)
            }
        }
    }
}
