import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

RowLayout {
    id: copy

    property string text: App.tr("No copy saved")

    signal save

    spacing: Look.e2

    IconImage {
        name: "warning"
        sourceSize: Qt.size(Look.iconoNota, Look.iconoNota)
        color: Look.ambar
    }

    Text {
        Layout.fillWidth: true
        text: copy.text
        wrapMode: Text.WordWrap
        font.family: Look.letraTexto
        font.pixelSize: Look.tPie
        color: Look.ambar
    }

    QQC2.Button {
        flat: true
        text: App.tr("Save a copy…")
        onClicked: copy.save()
    }
}
