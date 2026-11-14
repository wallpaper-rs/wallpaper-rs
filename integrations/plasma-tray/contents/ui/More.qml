import QtQuick
import QtQuick.Layouts
import "../look"

ColumnLayout {
    id: menu

    required property var daemon
    readonly property var options: daemon.current.options ?? ({})
    readonly property string battery: options["pause.on_battery"] ?? "always"
    readonly property var batteries: [["never", i18n("Never")], ["always", i18n("Always")], ["below:20", i18n("Below 20 %")]]
    property bool choosing: false

    spacing: 0

    Repeater {
        model: [["playback.scene_sound", i18n("Let scenes play their sounds"), true], ["playback.scene_capture", i18n("Let scenes react to the sound that plays"), true], ["playback.duck", i18n("Silence wallpapers while another application plays sound"), false], ["playback.allow_flashing", i18n("Allow wallpapers that flash"), false]]

        delegate: Toggle {
            required property var modelData

            Layout.fillWidth: true
            text: modelData[1]
            checked: (menu.options[modelData[0]] ?? modelData[2]) === true
            onToggled: {
                menu.daemon.act("SetOption", [modelData[0], checked ? "true" : "false"]);
                checked = Qt.binding(() => (menu.options[modelData[0]] ?? modelData[2]) === true);
            }
        }
    }

    Option {
        Layout.fillWidth: true
        text: i18n("Pause on battery")
        hint: (menu.batteries.find(entry => entry[0] === menu.battery) ?? ["", menu.battery])[1]
        more: true
        open: menu.choosing
        onClicked: menu.choosing = !menu.choosing
    }

    Choices {
        Layout.fillWidth: true
        visible: menu.choosing
        model: menu.batteries
        current: menu.battery
        onPicked: value => menu.daemon.act("SetOption", ["pause.on_battery", value])
    }

    Words {
        Layout.fillWidth: true
        Layout.margins: Look.e4
        visible: !menu.daemon.hasApp
        text: i18n("The other settings: in Settings… or with wprs-ctl")
        soft: true
        font.pixelSize: Look.tPie
        wrapMode: Text.Wrap
    }
}
