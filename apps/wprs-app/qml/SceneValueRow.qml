import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

RowLayout {
    id: row

    property string text
    default property alias control: slot.data

    Layout.fillWidth: true
    Layout.minimumHeight: Look.altoCampo
    spacing: Look.e3

    QQC2.Label {
        Layout.fillWidth: true
        Layout.preferredWidth: Look.e8
        Layout.minimumWidth: Look.e7
        text: row.text
        color: Look.bruma
        elide: Text.ElideRight
    }

    RowLayout {
        id: slot

        Layout.fillWidth: true
        Layout.maximumWidth: implicitWidth
        spacing: Look.e2
    }
}
