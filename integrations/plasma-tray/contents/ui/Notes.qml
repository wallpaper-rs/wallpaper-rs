import QtQuick
import QtQuick.Layouts
import "../look"

ColumnLayout {
    id: notes

    required property var host
    readonly property var info: host.info
    readonly property var lines: [info.left_out && info.left_out.length > 0 ? info.left_out.map(l => host.reasons.text(l)).join("\n") : "", info.away ? i18n("%1 exists only on another device. Publishing it to the server or to a repository makes it available here.", info.away) : "", info.particles === "cpu" ? i18n("Its particles run on the processor, so fewer of them are shown") : "", info.quality ? host.reasons.level(info, host.daemon.current) : "", host.reasons.privacy(host.pkg.exposes ?? [], info.sandbox ?? ""), host.pkg.photosensitive === true && host.pkg.flash_disabled === true ? i18n("Has flashes, turned off") : ""]

    spacing: Look.e1
    visible: info.error || lines.some(line => line !== "")

    Words {
        Layout.fillWidth: true
        visible: !!notes.info.error
        text: notes.info.error ?? ""
        color: Look.coral
        font.pixelSize: Look.tPie
        wrapMode: Text.Wrap
    }

    Repeater {
        model: notes.lines.filter(line => line !== "")

        delegate: Words {
            required property string modelData

            Layout.fillWidth: true
            text: modelData
            soft: true
            font.pixelSize: Look.tPie
            wrapMode: Text.Wrap
        }
    }

    Command {
        visible: (notes.info.left_out ?? []).length > 0
        quiet: true
        glyph: "arrow-clockwise"
        text: i18n("Try again")
        onClicked: notes.host.daemon.act("RetryEffects", [])
    }
}
