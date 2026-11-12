import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

RowLayout {
    id: trigger

    required property var host
    readonly property var page: host.page
    readonly property var known: host.field.required ? host.field.options : [null].concat(host.field.options)
    readonly property var options: host.at(known, host.value) >= 0 || host.value === null ? known : known.concat([host.value])
    readonly property string chosen: typeof host.value === "string" ? host.value : ""
    readonly property var started: chosen ? (page.view, JSON.parse(page.editor.rig("", JSON.stringify({
            "fires": chosen
        })) || "{}")) : ({})

    function said() {
        const names = (started.timelines ?? []).map(t => App.tr("the timeline %1", t)).concat((started.sounds ?? []).map(s => App.tr("the sound %1", s)));
        return names.length ? App.tr("Fires %1 in the preview, as the scene would, starting %2", chosen, names.join(", ")) : App.tr("Fires %1 in the preview, as the scene would; nothing in the scene starts on it yet", chosen);
    }

    QQC2.ComboBox {
        Layout.fillWidth: true
        model: trigger.options.map(option => option === null ? App.tr("Not set") : trigger.host.name(option))
        currentIndex: trigger.host.at(trigger.options, trigger.host.value)
        onActivated: index => {
            if (trigger.options[index] === null)
                trigger.page.run("remove", {
                    "pointer": trigger.host.field.pointer
                });
            else
                trigger.host.commit(trigger.options[index]);
        }
    }

    QQC2.ToolButton {
        icon.name: "media-playback-start"
        text: App.tr("Fire this trigger in the preview")
        display: QQC2.AbstractButton.IconOnly
        enabled: trigger.chosen !== ""
        onClicked: trigger.page.editor.fire(trigger.chosen)
        QQC2.ToolTip.visible: hovered
        QQC2.ToolTip.text: trigger.chosen ? trigger.said() : text
    }
}
