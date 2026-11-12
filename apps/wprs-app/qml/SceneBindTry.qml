import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.Menu {
    id: menu

    required property var page
    required property var field

    signal watch
    readonly property bool bound: field.source !== "" && field.source !== "keys"

    title: App.tr("Try and shape the response")
    enabled: menu.bound

    QQC2.MenuItem {
        text: App.tr("Watch the levels of the sound…")
        visible: menu.field.source === "audio"
        height: visible ? implicitHeight : 0
        onTriggered: menu.watch()
    }
}
