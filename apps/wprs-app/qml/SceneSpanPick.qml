import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.ToolButton {
    id: pick

    required property var page
    property int current: 0
    readonly property string layout: ["", "dual", "triple", "vertical", "mine"][current]
    readonly property var names: [App.tr("One screen"), App.tr("Two side by side"), App.tr("Three side by side"), App.tr("Two stacked"), App.tr("My screens")]

    text: current === 0 ? App.tr("Several screens") : names[current]
    icon.name: "caret-down"
    Accessible.name: App.tr("Several screens: %1", names[current])
    onClicked: menu.popup(pick, 0, pick.height)

    QQC2.Menu {
        id: menu


        Repeater {
            model: pick.names

            delegate: QQC2.MenuItem {
                required property int index
                required property string modelData

                text: modelData
                checkable: true
                checked: pick.current === index
                onTriggered: pick.current = index
            }
        }

        QQC2.MenuSeparator {
            visible: pick.current > 0
        }

        QQC2.MenuItem {
            visible: pick.current > 0
            height: visible ? implicitHeight : 0
            text: App.tr("Spread over every screen")
            checkable: true
            checked: pick.page.editor.value("/manifest/display/span") === "\"all-outputs\""
            onTriggered: pick.page.run("manifest", {
                    "display": checked ? {
                        "values": {
                            "span": "all-outputs"
                        }
                    } : {
                        "clear": ["span"]
                    }
                })
        }
    }
}
