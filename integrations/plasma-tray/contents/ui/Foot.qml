import QtQuick
import QtQuick.Layouts
import "../look"

RowLayout {
    id: foot

    required property var daemon
    property bool note: true

    spacing: Look.e2

    Command {
        Layout.fillWidth: true
        visible: foot.daemon.hasApp
        glyph: "images"
        text: i18n("Open wallpaper-rs")
        onClicked: foot.daemon.openApp("")
    }

    Command {
        Layout.fillWidth: true
        visible: foot.daemon.hasApp
        quiet: true
        glyph: "gear"
        text: i18n("Settings…")
        onClicked: foot.daemon.openApp("settings")
    }

    Words {
        Layout.fillWidth: true
        visible: !foot.daemon.hasApp && foot.note
        text: i18n("The other settings: in Settings… or with wprs-ctl")
        soft: true
        font.pixelSize: Look.tPie
        wrapMode: Text.Wrap
    }
}
