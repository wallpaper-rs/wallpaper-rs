import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: inputs

    required property var page
    readonly property var declared: page.manifest.inputs ?? []
    readonly property var choices: [["audio", App.tr("Hears the sound that plays"), App.tr("The page reads window.wprs.audio; it is asked for before it hears anything.")], ["pointer", App.tr("Follows the pointer"), App.tr("The page reads where the pointer is over its screen.")], ["clicks", App.tr("Reacts to clicks"), App.tr("The page reacts to presses of the pointer on the screen.")], ["media", App.tr("Follows the media player"), App.tr("The page reads the track, cover and position that window.wprs.onMedia reports.")]]

    function lights(on) {
        if (on)
            page.change("/manifest/permissions/rgb", true);
        else
            page.run("remove", {
                    "pointer": "/manifest/permissions/rgb"
                });
    }

    function set(input, on) {
        page.manifestEdit("inputs", {
                "inputs": [input],
                "add": on,
                "remove": !on
            });
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        text: App.tr("A page cannot tell which inputs it uses, so declare them here; the user is asked before the page gets any of them.")
    }

    Repeater {
        model: inputs.choices

        delegate: QQC2.CheckBox {
            id: box

            required property var modelData

            Layout.fillWidth: true
            text: modelData[1]
            checked: inputs.declared.includes(modelData[0])
            onToggled: inputs.set(modelData[0], checked)
            QQC2.ToolTip.text: modelData[2]
            QQC2.ToolTip.visible: hovered
        }
    }

    QQC2.CheckBox {
        id: lights

        Layout.fillWidth: true
        text: App.tr("Sends its colors to the RGB lighting")
        checked: inputs.page.manifest.permissions?.rgb === true
        onToggled: inputs.lights(checked)
        QQC2.ToolTip.text: App.tr("The page calls window.wprs.rgb.setImage; the user is asked before any color reaches the keyboard, mouse or other lights.")
        QQC2.ToolTip.visible: hovered
    }

    QQC2.CheckBox {
        Layout.fillWidth: true
        enabled: false
        checked: inputs.declared.includes("network")
        text: App.tr("Reaches the network (set by the origins above)")
    }
}
