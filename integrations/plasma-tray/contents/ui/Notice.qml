import QtQuick
import QtQuick.Layouts
import "../look"

ColumnLayout {
    id: notice

    required property var daemon
    required property int screens
    property bool details: false
    readonly property bool broken: daemon.lastError !== ""

    visible: broken || screens === 0
    spacing: Look.e2

    Glyph {
        name: notice.broken ? "warning" : "monitor"
        tone: notice.broken ? Look.coral : Look.bruma
        Layout.alignment: Qt.AlignHCenter
        Layout.topMargin: Look.e4
    }

    Words {
        Layout.fillWidth: true
        text: notice.broken ? i18n("The wallpaper service is not answering") : i18n("No screen shows wallpaper-rs")
        title: true
        horizontalAlignment: Text.AlignHCenter
        wrapMode: Text.Wrap
    }

    Words {
        Layout.fillWidth: true
        visible: !notice.broken
        text: i18n("Choose wallpaper-rs as the wallpaper type in the desktop settings.")
        soft: true
        horizontalAlignment: Text.AlignHCenter
        wrapMode: Text.Wrap
    }

    RowLayout {
        Layout.alignment: Qt.AlignHCenter
        visible: notice.broken
        spacing: Look.e2

        Command {
            glyph: "arrow-clockwise"
            text: i18n("Restart the service")
            onClicked: notice.daemon.restart()
        }

        Command {
            quiet: true
            text: i18n("Details")
            onClicked: notice.details = !notice.details
        }
    }

    Words {
        Layout.fillWidth: true
        visible: notice.broken && notice.details
        text: notice.daemon.lastError
        soft: true
        font.pixelSize: Look.tPie
        wrapMode: Text.Wrap
    }
}
