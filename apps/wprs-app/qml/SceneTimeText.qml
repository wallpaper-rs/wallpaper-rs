import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

RowLayout {
    id: row

    required property var host

    QQC2.Button {
        flat: true
        icon.name: "clock"
        text: App.tr("Show the time or the date…")
        onClicked: menu.popup()

        SceneBindTime {
            id: menu

            page: row.host.page
            field: row.host.field
            onPicked: source => row.host.page.run("set", {
                    "pointer": row.host.field.pointer,
                    "value": source
                })
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        opacity: 0.7
        text: App.tr("Words in braces such as {time} are shown as typed: use this button to follow the clock or the date.")
    }
}
