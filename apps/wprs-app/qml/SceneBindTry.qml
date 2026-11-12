import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.Menu {
    id: menu

    required property var page
    required property var field
    readonly property bool bound: field.source !== "" && field.source !== "keys"
    readonly property bool shaped: ["number", "integer", "vec2", "color", "rgb"].includes(field.widget)

    signal watch

    function smooth() {
        if (field.value?.smooth === undefined)
            page.run("set", {
                    "pointer": field.pointer + "/smooth",
                    "value": 0.2
                });
        page.editor.select(field.pointer);
    }

    title: App.tr("Try and shape the response")
    enabled: menu.bound

    QQC2.MenuItem {
        text: App.tr("Watch the levels of the sound…")
        visible: menu.field.source === "audio"
        height: visible ? implicitHeight : 0
        onTriggered: menu.watch()
    }

    QQC2.MenuItem {
        text: App.tr("Try the pointer")
        visible: menu.field.source === "cursor"
        height: visible ? implicitHeight : 0
        checkable: true
        checked: menu.page.editor.sweeping
        onTriggered: menu.page.editor.sweep(checked)
    }

    QQC2.MenuItem {
        text: App.tr("Smooth the changes…")
        enabled: menu.shaped
        onTriggered: menu.smooth()
    }
}
